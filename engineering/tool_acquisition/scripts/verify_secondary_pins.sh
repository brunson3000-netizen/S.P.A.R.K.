#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

python3 - <<'PY'
import json, pathlib, subprocess, sys
root = pathlib.Path.cwd()
lock_path = root / 'engineering/tool_acquisition/SECONDARY_HARVEST_LOCK.json'
data = json.loads(lock_path.read_text(encoding='utf-8'))
errors = []
for src in data['sources']:
    path = src['path']
    expected = src['commit']
    out = subprocess.run(
        ['git', 'ls-tree', 'HEAD', '--', path],
        text=True, capture_output=True, check=False,
    )
    if out.returncode != 0 or not out.stdout.strip():
        errors.append(f"missing gitlink: {path}")
        continue
    parts = out.stdout.strip().split(None, 3)
    if len(parts) < 4:
        errors.append(f"unparseable gitlink: {path}: {out.stdout.strip()}")
        continue
    mode, typ, got, _ = parts
    if mode != '160000' or typ != 'commit':
        errors.append(f"not a submodule gitlink: {path}: mode={mode} type={typ}")
        continue
    if got != expected:
        errors.append(f"pin mismatch {src['id']}: expected {expected} got {got}")
        continue
    name = path
    update = subprocess.run(
        ['git', 'config', '-f', '.gitmodules', '--get', f'submodule.{name}.update'],
        text=True, capture_output=True, check=False,
    ).stdout.strip()
    if update != 'none':
        errors.append(f"unsafe update policy {src['id']}: expected none got {update!r}")
        continue
    print(f"OK {src['id']} {got}")
if errors:
    print('\n'.join(errors), file=sys.stderr)
    raise SystemExit(1)
print(f"Verified {len(data['sources'])} secondary gitlinks and update=none policy.")
PY
