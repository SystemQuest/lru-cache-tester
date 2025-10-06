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

# Test Stage 1 with stage1 test helper (should pass)
test_stage1: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/stages/stage1 \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"jq3","tester_log_prefix":"stage-1","title":"Stage #1: Basic cache operations"}]' \
	./dist/tester

# Test Stage 1 with pass_all (should pass all Stage 1 tests)
test_stage1_all: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/pass_all \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"jq3","tester_log_prefix":"stage-1.1","title":"Stage #1.1: Basic cache"},{"slug":"jq3-multiple-keys","tester_log_prefix":"stage-1.2","title":"Stage #1.2: Multiple keys"},{"slug":"jq3-update","tester_log_prefix":"stage-1.3","title":"Stage #1.3: Key updates"}]' \
	./dist/tester

# Test Stage 2 with stage2 test helper (should pass when implemented)
test_stage2: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/stages/stage2 \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"ze6","tester_log_prefix":"stage-2","title":"Stage #2: FIFO eviction"}]' \
	./dist/tester

# Test Stage 2 with pass_all (should pass all Stage 2 tests when implemented)
test_stage2_all: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/pass_all \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"ze6","tester_log_prefix":"stage-2.1","title":"Stage #2.1: FIFO eviction"},{"slug":"ze6-update","tester_log_prefix":"stage-2.2","title":"Stage #2.2: Update no reorder"},{"slug":"ze6-size","tester_log_prefix":"stage-2.3","title":"Stage #2.3: SIZE with eviction"}]' \
	./dist/tester

# Test with compiled starter (should fail - not implemented yet)
test_starter: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/../build-your-own-lru-cache/compiled_starters/python \
	SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"jq3","tester_log_prefix":"stage-1","title":"Stage #1: Basic cache operations"}]' \
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
	@echo "Build & Test:"
	@echo "  make build          - Build the Rust tester"
	@echo "  make test           - Run cargo tests"
	@echo "  make test_stage1    - Test Stage 1 (basic cache)"
	@echo "  make test_stage1_all - Test all Stage 1 test cases"
	@echo "  make test_stage2    - Test Stage 2 (FIFO eviction)"
	@echo "  make test_stage2_all - Test all Stage 2 test cases"
	@echo "  make test_starter   - Test compiled starter (should fail)"
	@echo "  make test_manual    - Manually test solution with stdin"
	@echo ""
	@echo "Utilities:"
	@echo "  make all            - Full workflow: build + test"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make release        - Tag and release new version"
	@echo "  make help           - Show this help message"
	@echo ""
