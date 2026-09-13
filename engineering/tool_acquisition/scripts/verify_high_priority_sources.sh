#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"
python3 - <<'PY2'
import json, subprocess, pathlib, sys
root=pathlib.Path.cwd()
lock=json.loads((root/'engineering/tool_acquisition/UPSTREAM_LOCK.json').read_text())
errors=[]
for src in lock['sources']:
    p=root/src['path']
    if not p.exists():
        errors.append(f"missing checkout: {src['path']}")
        continue
    got=subprocess.check_output(['git','-C',str(p),'rev-parse','HEAD'],text=True).strip()
    if got != src['commit']:
        errors.append(f"pin mismatch {src['id']}: expected {src['commit']} got {got}")
    else:
        print(f"OK {src['id']} {got}")
if errors:
    print("\n".join(errors),file=sys.stderr)
    raise SystemExit(1)
print(f"Verified {len(lock['sources'])} pinned external source repositories.")
PY2
