#!/bin/sh
#
# pass_all placeholder - NOT a real implementation
#
# This directory is intentionally empty. Unlike CodeCrafters' redis-tester
# (which can test against real Redis), we don't have a real LRU cache product
# to test against.
#
# Users MUST set SYSTEMQUEST_REPOSITORY_DIR to their implementation directory.
#

cat >&2 << 'EOF'
❌ Error: No implementation found in pass_all/

This is intentional. You must set the SYSTEMQUEST_REPOSITORY_DIR environment
variable to point to your LRU cache implementation directory.

Example:
  export SYSTEMQUEST_REPOSITORY_DIR=/path/to/your/lru-cache-implementation
  ./your-tester-binary

Or:
  SYSTEMQUEST_REPOSITORY_DIR=. ./your-tester-binary

For more information, see: https://systemquest.io/docs/setup
EOF

exit 1
