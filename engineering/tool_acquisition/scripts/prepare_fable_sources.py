#!/usr/bin/env python3
"""Materialize pinned research sources only. Requires Python 3 and Git."""
import argparse
import datetime
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'engineering/tool_acquisition'
LOCKS = ['UPSTREAM_LOCK.json', 'SECONDARY_HARVEST_LOCK.json',
         'MIDDLE_LAYER_HARVEST_LOCK.json', 'COMMUNITY_HARVEST_LOCK.json']
ENV = dict(os.environ, GIT_LFS_SKIP_SMUDGE='1', GIT_TERMINAL_PROMPT='0')


def git(*args, cwd=ROOT):
    return subprocess.run(['git', '-c', 'core.hooksPath=/dev/null', *args],
                          cwd=cwd, env=ENV, text=True, capture_output=True,
                          timeout=1800)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--verify-only', action='store_true')
    p.add_argument('--max-mib', type=int, default=256,
                   help='Per-source tracked-tree limit; default 256 MiB')
    p.add_argument('--jobs', type=int, default=4)
    p.add_argument('--report', type=Path)
    a = p.parse_args()
    if a.max_mib < 1 or not 1 <= a.jobs <= 8:
        p.error('max-mib must be positive; jobs must be between 1 and 8')
    if a.report is None:
        report_path = git('rev-parse', '--git-path', 'fable-source-checkout-report.json')
        if report_path.returncode:
            raise RuntimeError(report_path.stderr)
        a.report = ROOT / report_path.stdout.strip()
    inventory = json.loads((BASE / 'FABLE_SOURCE_INVENTORY.json').read_text())
    sources = inventory['sources']
    canonical = {}
    for name in LOCKS:
        for s in json.loads((BASE / name).read_text())['sources']:
            canonical[s['path']] = s['commit']
    if len(canonical) != len(sources):
        raise RuntimeError('Inventory and canonical locks differ; refresh inventory')
    rows, selected = [], []
    for s in sources:
        path = s['path']
        if not path.startswith('external/harvest/') or '..' in Path(path).parts:
            raise RuntimeError('Invalid source path: ' + path)
        if canonical.get(path) != s['commit']:
            raise RuntimeError('Inventory pin differs from canonical lock: ' + path)
        index = git('ls-files', '--stage', '--', path)
        fields = index.stdout.split()
        if index.returncode or fields[:3] != ['160000', s['commit'], '0']:
            raise RuntimeError('Gitlink does not match lock: ' + path)
        url = git('config', '-f', '.gitmodules', '--get', 'submodule.' + path + '.url')
        if url.returncode or url.stdout.strip() != 'https://github.com/' + s['repo'] + '.git':
            raise RuntimeError('Unexpected upstream URL: ' + path)
        row = {'id': s['id'], 'path': path, 'expected_commit': s['commit'],
               'tracked_blob_bytes': s['tracked_blob_bytes'],
               'nested_gitlinks': s['nested_gitlinks']}
        rows.append(row)
        if not s['tree_complete'] or s['tracked_blob_bytes'] > a.max_mib * 1024**2:
            row['status'] = 'DEFERRED_SIZE'
            print('DEFERRED_SIZE', s['id'], flush=True)
            continue
        dest = ROOT / path
        if dest.exists() and any(dest.iterdir()):
            if not (dest / '.git').exists():
                raise RuntimeError('Refusing to overwrite non-Git directory: ' + path)
            dirty = git('status', '--porcelain', '--untracked-files=normal', cwd=dest)
            head = git('rev-parse', 'HEAD', cwd=dest)
            if dirty.returncode or dirty.stdout.strip() or head.stdout.strip() != s['commit']:
                row['status'] = 'BLOCKED_EXISTING_WORKTREE'
                print(row['status'], s['id'], flush=True)
                continue
        selected.append((s, row))
    if selected and not a.verify_only:
        paths = [s['path'] for s, _ in selected]
        # One Git process owns shared submodule configuration; child clones use --jobs.
        sync = git('submodule', 'sync', '--', *paths)
        if sync.returncode:
            raise RuntimeError(sync.stderr)
        print('Downloading', len(paths), 'pinned sources; no builds or installs', flush=True)
        result = git('submodule', 'update', '--init', '--checkout', '--depth', '1',
                     '--jobs', str(a.jobs), '--', *paths)
        if result.returncode:
            print(result.stderr[-12000:], file=sys.stderr)
        # Verify each checkout even if Git reported a partial download failure.
    for s, row in selected:
        dest = ROOT / s['path']
        if not (dest / '.git').exists():
            row['status'] = 'MISSING_CHECKOUT'
        else:
            head = git('rev-parse', 'HEAD', cwd=dest)
            clean = git('status', '--porcelain', '--untracked-files=normal', cwd=dest)
            row['actual_commit'] = head.stdout.strip()
            if head.returncode or row['actual_commit'] != s['commit']:
                row['status'] = 'PIN_MISMATCH'
            elif clean.returncode or clean.stdout.strip():
                row['status'] = 'DIRTY_CHECKOUT'
            else:
                row['status'] = 'VERIFIED_TOP_LEVEL_CHECKOUT'
                tree = git('rev-parse', 'HEAD^{tree}', cwd=dest)
                sizes = git('ls-tree', '-r', '-l', 'HEAD', cwd=dest)
                row['actual_tree_sha'] = tree.stdout.strip()
                blobs = [line.split('\t', 1)[0].split() for line in sizes.stdout.splitlines()]
                row['actual_tracked_blob_bytes'] = sum(int(b[3]) for b in blobs if b[1] == 'blob')
                if tree.returncode or sizes.returncode or row['actual_tracked_blob_bytes'] != s['tracked_blob_bytes']:
                    row['status'] = 'TREE_MEASUREMENT_MISMATCH'
                row['nested_sources_materialized'] = False if s['nested_gitlinks'] else None
                row['lfs_payloads_materialized'] = False
        print(row['status'], s['id'], flush=True)
    nested_rows = []
    nested = json.loads((BASE / 'FABLE_NESTED_SOURCE_INVENTORY.json').read_text())['sources']
    for s in nested:
        row = dict(s)
        nested_rows.append(row)
        parent_row = next(r for r in rows if r['id'] == s['parent'])
        parent = ROOT / s['parent_path']
        dest = parent / s['path']
        if '..' in Path(s['path']).parts or Path(s['path']).is_absolute():
            raise RuntimeError('Invalid nested path')
        if parent_row['status'] != 'VERIFIED_TOP_LEVEL_CHECKOUT':
            row['status'] = 'PARENT_UNAVAILABLE'
        elif not s['tree_complete'] or s['tracked_blob_bytes'] > a.max_mib * 1024**2:
            row['status'] = 'DEFERRED_SIZE'
        else:
            entry = git('ls-tree', 'HEAD', '--', s['path'], cwd=parent).stdout.split()
            url = git('config', '-f', '.gitmodules', '--get',
                      'submodule.' + s['path'] + '.url', cwd=parent).stdout.strip()
            if entry[:3] != ['160000', 'commit', s['commit']] or url.removesuffix('.git') != 'https://github.com/' + s['repo']:
                raise RuntimeError('Nested source metadata mismatch: ' + s['repo'])
            if dest.exists() and any(dest.iterdir()):
                if not (dest / '.git').exists():
                    raise RuntimeError('Refusing to overwrite nested directory: ' + str(dest))
                dirty = git('status', '--porcelain', cwd=dest)
                head = git('rev-parse', 'HEAD', cwd=dest)
                if dirty.returncode or dirty.stdout.strip() or head.stdout.strip() != s['commit']:
                    row['status'] = 'BLOCKED_EXISTING_WORKTREE'
            if 'status' not in row and not a.verify_only:
                print('Downloading nested source', s['repo'], flush=True)
                sync = git('submodule', 'sync', '--', s['path'], cwd=parent)
                if sync.returncode:
                    raise RuntimeError(sync.stderr)
                result = git('submodule', 'update', '--init', '--checkout', '--depth', '1',
                             '--', s['path'], cwd=parent)
                if result.returncode:
                    print(result.stderr[-4000:], file=sys.stderr)
            if 'status' not in row:
                if not (dest / '.git').exists():
                    row['status'] = 'MISSING_CHECKOUT'
                else:
                    head = git('rev-parse', 'HEAD', cwd=dest)
                    dirty = git('status', '--porcelain', cwd=dest)
                    row['actual_commit'] = head.stdout.strip()
                    row['status'] = ('VERIFIED_CHECKOUT' if not head.returncode and
                                     not dirty.returncode and not dirty.stdout.strip() and
                                     row['actual_commit'] == s['commit'] else 'CHECKOUT_MISMATCH')
                    sizes = git('ls-tree', '-r', '-l', 'HEAD', cwd=dest)
                    blobs = [line.split('\t', 1)[0].split() for line in sizes.stdout.splitlines()]
                    row['actual_tracked_blob_bytes'] = sum(int(b[3]) for b in blobs if b[1] == 'blob')
                    if sizes.returncode or row['actual_tracked_blob_bytes'] != s['tracked_blob_bytes']:
                        row['status'] = 'TREE_MEASUREMENT_MISMATCH'
        print(row['status'], s['repo'], flush=True)
    for r in rows:
        children = [n for n in nested_rows if n['parent'] == r['id']]
        if children:
            r['nested_sources_materialized'] = all(n['status'] == 'VERIFIED_CHECKOUT' for n in children)
    failed = [r for r in rows if r['status'] not in
              ('VERIFIED_TOP_LEVEL_CHECKOUT', 'DEFERRED_SIZE')]
    nested_failed = [r for r in nested_rows if r['status'] not in ('VERIFIED_CHECKOUT', 'DEFERRED_SIZE')]
    report = {'recorded_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'max_source_mib': a.max_mib, 'source_count': len(rows),
              'verified_count': sum(r['status'] == 'VERIFIED_TOP_LEVEL_CHECKOUT' for r in rows),
              'deferred_count': sum(r['status'] == 'DEFERRED_SIZE' for r in rows),
              'failed_count': len(failed) + len(nested_failed), 'sources': rows,
              'nested_sources': nested_rows,
              'nested_verified_count': sum(r['status'] == 'VERIFIED_CHECKOUT' for r in nested_rows),
              'scope': 'Pinned Git content including measured nested sources; LFS payloads excluded. No build/install/runtime execution.'}
    a.report.parent.mkdir(parents=True, exist_ok=True)
    a.report.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: report[k] for k in
                      ('source_count', 'verified_count', 'deferred_count', 'nested_verified_count', 'failed_count')}))
    return 1 if failed or nested_failed else 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (RuntimeError, subprocess.TimeoutExpired) as exc:
        print('ERROR:', exc, file=sys.stderr)
        sys.exit(1)
