# Test Helpers

This directory contains test fixtures and debugging helpers for the LRU Cache tester.

## Directory Structure

```
test_helpers/
├── pass_all/           ❌ Placeholder (intentionally fails)
├── fixtures/           ✅ Test fixtures for regression testing
└── debug/              ✅ Debug helpers and test scenarios
```

## pass_all/ - Intentionally Empty

**Important**: Unlike CodeCrafters' testers (which test against real products like Redis, Git, etc.), we don't have a real LRU cache product to test against.

Therefore, `pass_all/` is **intentionally a placeholder** that returns an error message.

### Why?

1. **No real product to test against**: CodeCrafters' `redis-tester` can test against real Redis. We don't have an equivalent "real LRU cache" product.

2. **Security**: Not providing any reference implementation prevents solution leakage.

3. **Clear expectations**: Users must explicitly set their implementation directory.

### Usage

Users **must** set the `SYSTEMQUEST_REPOSITORY_DIR` environment variable:

```bash
# Set to your implementation directory
export SYSTEMQUEST_REPOSITORY_DIR=/path/to/your/lru-cache-impl

# Or inline
SYSTEMQUEST_REPOSITORY_DIR=. ./lru-cache-tester
```

### What happens if not set?

The tester will fail with a clear error message:

```
❌ Error: No implementation found in pass_all/

This is intentional. You must set the SYSTEMQUEST_REPOSITORY_DIR environment
variable to point to your LRU cache implementation directory.

Example:
  export SYSTEMQUEST_REPOSITORY_DIR=/path/to/your/lru-cache-implementation
  ./your-tester-binary
```

## fixtures/ - Test Fixtures

Contains recorded test fixtures for regression testing. These are used to ensure consistent test behavior across runs.

## debug/ - Debug Helpers

Contains debugging scripts and test scenarios for development purposes.

---

## For Developers

### Testing the Tester

When developing the tester itself, you need a working implementation to test against. You have two options:

**Option 1: Use a local reference implementation**
```bash
# Create a simple reference implementation
cd /tmp
mkdir lru-cache-ref
cd lru-cache-ref
# ... implement basic LRU cache ...

# Test the tester
cd /path/to/lru-cache-tester
SYSTEMQUEST_REPOSITORY_DIR=/tmp/lru-cache-ref cargo test
```

**Option 2: Use the solutions repository**
```bash
# Clone the solutions repo (private)
git clone git@github.com:SystemQuest/lru-cache-solutions.git

# Test against Stage 1 solution
SYSTEMQUEST_REPOSITORY_DIR=../lru-cache-solutions/python/01-s1-basic/code cargo test test_stage1

# Test against Stage 2 solution
SYSTEMQUEST_REPOSITORY_DIR=../lru-cache-solutions/python/02-s2-fifo/code cargo test test_stage2
```

### Adding New Test Cases

1. Add test case to `src/tests/test_stage*.rs`
2. Run against a known-good implementation
3. Verify expected output
4. (Optional) Record fixtures: `SYSTEMQUEST_RECORD_FIXTURES=true cargo test`

---

## Comparison with CodeCrafters

| Aspect | CodeCrafters redis-tester | SystemQuest lru-cache-tester |
|--------|---------------------------|------------------------------|
| **pass_all/** | Launches real Redis | ❌ Intentionally fails |
| **Purpose** | Test tester against real product | No real product available |
| **Default behavior** | Can run without user code | ❌ Requires REPOSITORY_DIR |
| **Security** | Real product = no leakage | No implementation = no leakage |

### Why the difference?

CodeCrafters challenges are based on **real-world products**:
- build-your-own-redis → Real Redis
- build-your-own-git → Real Git  
- build-your-own-http-server → Real HTTP servers

Their `pass_all/` can launch the real product for testing.

Our challenge is a **learning exercise** without a standard "real LRU cache" product, so we intentionally don't provide any default implementation.

---

## Philosophy

> **"Make it clear, not clever."**

By explicitly requiring users to set `SYSTEMQUEST_REPOSITORY_DIR`, we:
- ✅ Prevent accidental solution leakage
- ✅ Make the testing process transparent
- ✅ Encourage users to understand the setup
- ✅ Follow the principle of least surprise

Users who forget to set the variable get a **helpful error message**, not a mysterious failure.
