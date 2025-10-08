# ==============================================================================
# Variables
# ==============================================================================
SOLUTION_DEV_ROOT := $(shell pwd)/../lru-cache-solution-dev
COURSE_ROOT := $(shell pwd)/..
PWD := $(shell pwd)

# User can override these via environment variables
# Example: SYSTEMQUEST_REPOSITORY_DIR=/my/impl make test_solution_stage1
SYSTEMQUEST_REPOSITORY_DIR ?= $(SOLUTION_DEV_ROOT)/python/01-jq3/code
SYSTEMQUEST_TEST_CASES_JSON ?= $(STAGE1_BASIC)

# Test cases JSON configurations (use = for deferred expansion to avoid # comment issues)
STAGE1_BASIC = [{"slug":"jq3","tester_log_prefix":"stage-1","title":"Stage \#1: Basic cache operations"}]
STAGE1_ALL = [{"slug":"jq3","tester_log_prefix":"stage-1.1","title":"Stage \#1.1: Basic cache"},{"slug":"jq3-multiple-keys","tester_log_prefix":"stage-1.2","title":"Stage \#1.2: Multiple keys"},{"slug":"jq3-update","tester_log_prefix":"stage-1.3","title":"Stage \#1.3: Key updates"}]
STAGE2_BASIC = [{"slug":"ze6","tester_log_prefix":"stage-2","title":"Stage \#2: FIFO eviction"}]
STAGE2_ALL = [{"slug":"ze6","tester_log_prefix":"stage-2.1","title":"Stage \#2.1: FIFO eviction"},{"slug":"ze6-update","tester_log_prefix":"stage-2.2","title":"Stage \#2.2: Update no reorder"},{"slug":"ze6-size","tester_log_prefix":"stage-2.3","title":"Stage \#2.3: SIZE with eviction"}]
STAGE3_BASIC = [{"slug":"ch7","tester_log_prefix":"stage-3","title":"Stage \#3: LRU eviction"}]
STAGE3_ALL = [{"slug":"ch7","tester_log_prefix":"stage-3.1","title":"Stage \#3.1: LRU eviction"},{"slug":"ch7-vs-fifo","tester_log_prefix":"stage-3.2","title":"Stage \#3.2: LRU vs FIFO"},{"slug":"ch7-multiple","tester_log_prefix":"stage-3.3","title":"Stage \#3.3: Multiple access"},{"slug":"ch7-sequential","tester_log_prefix":"stage-3.4","title":"Stage \#3.4: Sequential evictions"}]
STAGE4_BASIC = [{"slug":"vh5","tester_log_prefix":"stage-4","title":"Stage \#4: Custom DLL"}]
STAGE4_ALL = [{"slug":"vh5","tester_log_prefix":"stage-4.1","title":"Stage \#4.1: LRU eviction"},{"slug":"vh5-vs-fifo","tester_log_prefix":"stage-4.2","title":"Stage \#4.2: LRU vs FIFO"},{"slug":"vh5-multiple","tester_log_prefix":"stage-4.3","title":"Stage \#4.3: Multiple access"},{"slug":"vh5-sequential","tester_log_prefix":"stage-4.4","title":"Stage \#4.4: Sequential evictions"},{"slug":"vh5-capacity-one","tester_log_prefix":"stage-4.5","title":"Stage \#4.5: Capacity one"},{"slug":"vh5-empty-cache","tester_log_prefix":"stage-4.6","title":"Stage \#4.6: Empty cache"},{"slug":"vh5-repeated-ops","tester_log_prefix":"stage-4.7","title":"Stage \#4.7: Repeated ops"},{"slug":"vh5-eviction-cycle","tester_log_prefix":"stage-4.8","title":"Stage \#4.8: Eviction cycle"}]

.PHONY: build test clean release all help
.PHONY: test_stage1 test_stage1_all test_stage2 test_stage2_all test_stage3 test_stage3_all test_stage4 test_stage4_all test_starter test_manual
.PHONY: test_solution_stage1 test_solution_stage2 test_solution_stage2_all test_solution_stage3 test_solution_stage3_all test_solution_stage4 test_solution_stage4_all

# ==============================================================================
# Build & Test
# ==============================================================================

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

# Full workflow: build + test
all: build test

# ==============================================================================
# Test Helpers (internal/test_helpers)
# ==============================================================================
# Note: We intentionally do NOT provide test helpers with working implementations.
# Users must set SYSTEMQUEST_REPOSITORY_DIR to their own implementation.
# See internal/test_helpers/README.md for details.

# Test with compiled starter (should fail - not implemented yet)
test_starter: build
	SYSTEMQUEST_REPOSITORY_DIR=$(COURSE_ROOT)/build-your-own-lru-cache/compiled_starters/python \
	SYSTEMQUEST_TEST_CASES_JSON='$(STAGE1_BASIC)' \
	./dist/tester || true

# Test error message when REPOSITORY_DIR not set
test_error_message: build
	@echo "Testing error message (should fail with clear message)..."
	@./dist/tester || true

# Test pass_all placeholder (should fail with error message)
test_pass_all_error: build
	@echo "Testing pass_all placeholder (should show error message)..."
	SYSTEMQUEST_REPOSITORY_DIR=$(PWD)/internal/test_helpers/pass_all \
	SYSTEMQUEST_TEST_CASES_JSON='$(STAGE1_BASIC)' \
	./dist/tester || true

# ==============================================================================
# Solution Dev Testing (lru-cache-solution-dev)
# ==============================================================================
# Users can override SYSTEMQUEST_REPOSITORY_DIR and SYSTEMQUEST_TEST_CASES_JSON:
# Example: SYSTEMQUEST_REPOSITORY_DIR=/my/impl make test_solution_stage1

# Test solution-dev Stage 1
test_solution_stage1: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/01-jq3/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE1_BASIC)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 2
test_solution_stage2: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/02-ze6/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE2_BASIC)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 2 with all test cases
test_solution_stage2_all: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/02-ze6/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE2_ALL)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 3
test_solution_stage3: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/03-ch7/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE3_BASIC)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 3 with all test cases
test_solution_stage3_all: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/03-ch7/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE3_ALL)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 4
test_solution_stage4: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/04-vh5/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE4_BASIC)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Test solution-dev Stage 4 with all test cases
test_solution_stage4_all: build
	@REPO_DIR=$${SYSTEMQUEST_REPOSITORY_DIR:-$(SOLUTION_DEV_ROOT)/python/04-vh5/code}; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE4_ALL)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$REPO_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# Generic test target - fully customizable via environment variables
test_custom: build
	@if [ -z "$$SYSTEMQUEST_REPOSITORY_DIR" ]; then \
		echo "❌ Error: SYSTEMQUEST_REPOSITORY_DIR not set"; \
		echo "Usage: SYSTEMQUEST_REPOSITORY_DIR=/path/to/impl make test_custom"; \
		exit 1; \
	fi; \
	TEST_CASES=$${SYSTEMQUEST_TEST_CASES_JSON:-'$(STAGE1_BASIC)'}; \
	SYSTEMQUEST_REPOSITORY_DIR=$$SYSTEMQUEST_REPOSITORY_DIR \
	SYSTEMQUEST_TEST_CASES_JSON=$$TEST_CASES \
	./dist/tester

# ==============================================================================
# Release
# ==============================================================================

current_version_number := $(shell git tag --list "v*" | sort -V | tail -n 1 | cut -c 2-)
next_version_number := $(shell echo $$(($(current_version_number)+1)))

release:
	git tag v$(next_version_number)
	git push origin main v$(next_version_number)

# ==============================================================================
# Help
# ==============================================================================

help:
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "LRU Cache Tester - Makefile Commands"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo ""
	@echo "Build & Test:"
	@echo "  make build                  - Build the Rust tester"
	@echo "  make test                   - Run cargo unit tests"
	@echo "  make all                    - Build + test"
	@echo "  make clean                  - Clean build artifacts"
	@echo ""
	@echo "Test Error Handling:"
	@echo "  make test_starter           - Test compiled starter (should fail)"
	@echo "  make test_error_message     - Test error when REPOSITORY_DIR not set"
	@echo "  make test_pass_all_error    - Test pass_all placeholder error"
	@echo ""
	@echo "Solution Dev Testing (override with SYSTEMQUEST_REPOSITORY_DIR=/path):"
	@echo "  make test_solution_stage1   - Test solution-dev Stage 1"
	@echo "  make test_solution_stage2   - Test solution-dev Stage 2 basic"
	@echo "  make test_solution_stage2_all - Test solution-dev Stage 2 all"
	@echo "  make test_solution_stage3   - Test solution-dev Stage 3 basic"
	@echo "  make test_solution_stage3_all - Test solution-dev Stage 3 all"
	@echo "  make test_solution_stage4   - Test solution-dev Stage 4 basic"
	@echo "  make test_solution_stage4_all - Test solution-dev Stage 4 all"
	@echo "  make test_custom            - Test custom impl (requires REPOSITORY_DIR)"
	@echo ""
	@echo "Release:"
	@echo "  make release                - Tag and push new version"
	@echo ""
	@echo "Help:"
	@echo "  make help                   - Show this message"
	@echo ""
