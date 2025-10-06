.PHONY: build test clean release test_stage1 test_starter test_manual

# Build the tester binary to dist/tester (following CodeCrafters pattern)
build:
	cargo build --release
	mkdir -p dist
	cp target/release/lru-cache-tester dist/tester

# Run unit tests (cargo test)
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist

# Release (placeholder for version tagging)
release:
	@echo "Release target not yet implemented"

# Test Stage 1 with test helper (should pass)
test_stage1: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/pass_stage1 \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"s1-basic","tester_log_prefix":"stage-1","title":"Stage #1: Basic cache operations"}]' \
	./dist/tester

# Test with compiled starter (should fail - not implemented yet)
test_starter: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/../build-your-own-lru-cache/compiled_starters/python \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"s1-basic","tester_log_prefix":"stage-1","title":"Stage #1: Basic cache operations"}]' \
	./dist/tester || true

# Manual test - run solution directly for debugging
test_manual:
	@echo "Testing solution manually..."
	@printf "INIT 10\nPUT name Alice\nGET name\nGET age\nPUT name Bob\nGET name" | \
		python3 ../build-your-own-lru-cache/solutions/python/01-s1-basic/code/app/main.py
all: compile build test

# 发布新版本
current_version_number := $(shell git tag --list "v*" | sort -V | tail -n 1 | cut -c 2-)
next_version_number := $(shell echo $$(($(current_version_number)+1)))

release:
	git tag v$(next_version_number)
	git push origin main v$(next_version_number)

# 帮助信息
help:
	@echo "LRU Cache Course - Makefile Commands"
	@echo ""
	@echo "  make build         - Build the Rust tester"
	@echo "  make test          - Run tests against Stage 1 solution"
	@echo "  make test_debug    - Run tests with debug output"
	@echo "  make test_starter  - Test compiled starter (should fail)"
	@echo "  make test_manual   - Manually test solution with stdin"
	@echo "  make compile       - Compile course starters"
	@echo "  make all           - Full workflow: compile + build + test"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make release       - Tag and release new version"
	@echo ""
