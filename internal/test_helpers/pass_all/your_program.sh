#!/bin/sh
#
# Test helper script that implements a working LRU cache
# This is used to test the tester itself
#

set -e

# For now, use the reference solution from the course repo
SOLUTION_DIR="/Users/boboweike/mystudy/2025/2025_2/codecrafters-workspace/systemquest-courses/build-your-own-lru-cache/solutions/python/01-s1-basic/code"

cd "$SOLUTION_DIR"
exec python3 -u -m app.main "$@"
