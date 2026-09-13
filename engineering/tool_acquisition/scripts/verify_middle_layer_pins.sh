#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"
lock="engineering/tool_acquisition/MIDDLE_LAYER_HARVEST_LOCK.json"
[[ -f "$lock" ]] || { echo "ERROR: missing $lock" >&2; exit 2; }

fail=0
while IFS=$'\t' read -r id path expected; do
  actual="$(git ls-tree HEAD -- "$path" | awk '{print $3}')"
  if [[ -z "$actual" ]]; then
    echo "FAIL missing gitlink: $id  $path"
    fail=1
  elif [[ "$actual" != "$expected" ]]; then
    echo "FAIL pin mismatch: $id expected=$expected actual=$actual"
    fail=1
  else
    echo "PASS $id $expected"
  fi

done < <(python3 - "$lock" <<'PY'
import json, sys
with open(sys.argv[1], encoding='utf-8') as f:
    d=json.load(f)
for s in d['sources']:
    print(f"{s['id']}\t{s['path']}\t{s['commit']}")
PY
)

if [[ $fail -ne 0 ]]; then
  exit 1
fi

echo "PASS — all middle-layer gitlinks match the lock file."
echo "This verifies repository pins only; it does not materialize or activate sources."
