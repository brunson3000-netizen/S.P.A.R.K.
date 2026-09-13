#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

lock="engineering/tool_acquisition/MIDDLE_LAYER_HARVEST_LOCK.json"
[[ -f "$lock" ]] || { echo "ERROR: missing $lock" >&2; exit 2; }

usage() {
  echo "Usage: $0 list | <source-id>"
  echo "Materializes exactly one pinned middle-layer source; it does not build, install, or activate it."
}

cmd="${1:-list}"

if [[ "$cmd" == "list" ]]; then
  python3 - "$lock" <<'PY'
import json, sys
p=sys.argv[1]
with open(p, encoding='utf-8') as f:
    d=json.load(f)
for s in d['sources']:
    print(f"{s['id']:<28} {s['priority']:<12} {s['repo']}  {s['commit'][:12]}")
PY
  exit 0
fi

mapfile -t fields < <(python3 - "$lock" "$cmd" <<'PY'
import json, sys
p, wanted=sys.argv[1:]
with open(p, encoding='utf-8') as f:
    d=json.load(f)
for s in d['sources']:
    if s['id'] == wanted:
        print(s['path'])
        print(s['commit'])
        print(s['url'])
        break
else:
    raise SystemExit(3)
PY
) || {
  echo "ERROR: unknown source id: $cmd" >&2
  usage >&2
  exit 3
}

[[ ${#fields[@]} -eq 3 ]] || { echo "ERROR: could not resolve source id: $cmd" >&2; exit 3; }
path="${fields[0]}"
expected="${fields[1]}"
url="${fields[2]}"

echo "Source:   $cmd"
echo "Path:     $path"
echo "Upstream: $url"
echo "Pin:      $expected"

git submodule sync -- "$path"
# Explicit --checkout overrides the lane's normal update=none policy for this one requested source.
git submodule update --init --depth 1 --checkout -- "$path"

actual="$(git -C "$path" rev-parse HEAD)"
if [[ "$actual" != "$expected" ]]; then
  echo "ERROR: pin mismatch: expected $expected got $actual" >&2
  exit 4
fi

if [[ -n "$(git -C "$path" status --porcelain)" ]]; then
  echo "ERROR: materialized source is dirty" >&2
  git -C "$path" status --short >&2
  exit 5
fi

echo "PASS — $cmd materialized at exact pin $actual"
echo "No build/install/hook/gateway/runtime activation was performed."
