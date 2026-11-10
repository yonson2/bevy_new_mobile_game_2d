#!/usr/bin/env bash
# The checks that need no build: each issue tracker is valid and its index is in sync,
# each asset has its ledger row, and the live docs are in order.
set -euo pipefail
cd "$(dirname "$0")/.."
for dir in issues meta/issues; do
    if [ -f "$dir/README.md" ]; then python3 tools/issues.py --dir "$dir" check; fi
done
python3 tools/ledger.py check
python3 tools/check-docs.py
