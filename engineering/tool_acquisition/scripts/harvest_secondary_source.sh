#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

lock="engineering/tool_acquisition/SECONDARY_HARVEST_LOCK.json"

usage() {
  cat <<'EOF'
Usage:
  engineering/tool_acquisition/scripts/harvest_secondary_source.sh list
  engineering/tool_acquisition/scripts/harvest_secondary_source.sh <source-id>

This materializes exactly one secondary source at its recorded pin.
It does not build, install, activate, or add the source as a runtime dependency.
EOF
}

if [[ ! -f "$lock" ]]; then
  echo "ERROR: missing secondary lock: $lock" >&2
  exit 1
fi

cmd="${1:-}"
if [[ -z "$cmd" ]]; then
  usage >&2
  exit 2
fi

if [[ "$cmd" == "list" ]]; then
  python3 - "$lock" <<'PY'
import json, sys
with open(sys.argv[1], encoding='utf-8') as f:
    data = json.load(f)
for src in data['sources']:
    note = src.get('disposition_note', '')
    suffix = f" — {note}" if note else ''
    print(f"{src['id']:<28} {src['category']:<28} {src['repo']}{suffix}")
PY
  exit 0
fi

if [[ $# -ne 1 ]]; then
  usage >&2
  exit 2
fi

mapfile -t fields < <(python3 - "$lock" "$cmd" <<'PY'
import json, sys
lock_path, wanted = sys.argv[1], sys.argv[2]
with open(lock_path, encoding='utf-8') as f:
    data = json.load(f)
for src in data['sources']:
    if src['id'] == wanted:
        print(src['path'])
        print(src['commit'])
        print(src['repo'])
        raise SystemExit(0)
print(f"ERROR: unknown secondary source id: {wanted}", file=sys.stderr)
raise SystemExit(3)
PY
)

if [[ ${#fields[@]} -ne 3 ]]; then
  echo "ERROR: failed to resolve source '$cmd' from lock" >&2
  exit 3
fi

path="${fields[0]}"
expected="${fields[1]}"
repo="${fields[2]}"

case "$path" in
  external/harvest/secondary/*) ;;
  *)
    echo "ERROR: lock resolved outside secondary quarantine: $path" >&2
    exit 4
    ;;
esac

if ! git config -f .gitmodules --get-regexp '^submodule\..*\.path$' | awk '{print $2}' | grep -Fxq -- "$path"; then
  echo "ERROR: source path is not registered in .gitmodules: $path" >&2
  exit 5
fi

printf 'Harvesting %s\n  repo: %s\n  path: %s\n  pin:  %s\n' "$cmd" "$repo" "$path" "$expected"

git submodule sync -- "$path"

# Secondary .gitmodules entries use update=none so normal recursive sync is cheap.
# Override that setting for this one deliberate checkout only.
git -c "submodule.${path}.update=checkout" \
  submodule update --init --recommend-shallow -- "$path"

if [[ ! -d "$path/.git" && ! -f "$path/.git" ]]; then
  echo "ERROR: checkout did not materialize as a git worktree: $path" >&2
  exit 6
fi

got="$(git -C "$path" rev-parse HEAD)"
if [[ "$got" != "$expected" ]]; then
  echo "ERROR: pin mismatch for $cmd" >&2
  echo "  expected: $expected" >&2
  echo "  got:      $got" >&2
  exit 7
fi

if [[ -n "$(git -C "$path" status --porcelain --untracked-files=no)" ]]; then
  echo "ERROR: harvested source is unexpectedly modified: $path" >&2
  git -C "$path" status --short >&2
  exit 8
fi

printf 'VERIFIED %s %s\n' "$cmd" "$got"
printf 'State: downloaded=yes built=no installed=no activated=no adopted=no\n'
