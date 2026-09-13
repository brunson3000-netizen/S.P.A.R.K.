#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

url="https://github.com/rtk-ai/rtk.git"
pin="d0c2985155568d1d76fca03bc65d5098f136bbcd"
dest="external/harvest/high_priority/rtk"

if [[ -d "$dest/.git" ]] || git -C "$dest" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git -C "$dest" fetch origin "$pin"
else
  mkdir -p "$(dirname "$dest")"
  git clone --no-checkout "$url" "$dest"
fi

git -C "$dest" checkout --detach "$pin"
actual="$(git -C "$dest" rev-parse HEAD)"
[[ "$actual" == "$pin" ]] || { echo "RTK pin mismatch: $actual" >&2; exit 1; }

echo "RTK source harvested and verified at $actual"
echo "No RTK hooks, init, install, or activation were performed."
