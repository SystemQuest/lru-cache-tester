#!/bin/sh
#
# Stage 1 test helper - basic cache only
# Points to Stage 1 solution (no eviction)
#

set -e

# Point to Stage 1 solution
SOLUTION_DIR="/Users/boboweike/mystudy/2025/2025_2/codecrafters-workspace/systemquest-courses/build-your-own-lru-cache/solutions/python/01-s1-basic/code"

cd "$SOLUTION_DIR"
exec python3 -u -m app.main "$@"
