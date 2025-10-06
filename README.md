# LRU Cache Tester

This is the tester for the [Build Your Own LRU Cache](../build-your-own-lru-cache/) challenge on SystemQuest.

## ⚠️ Important: You Must Set SYSTEMQUEST_REPOSITORY_DIR

This tester requires you to provide your own LRU cache implementation. Unlike CodeCrafters' testers (which can test against real Redis, Git, etc.), we don't provide any default implementation.

**You must set the `SYSTEMQUEST_REPOSITORY_DIR` environment variable:**

```bash
# Set to your implementation directory
export SYSTEMQUEST_REPOSITORY_DIR=/path/to/your/lru-cache-impl

# Or use inline
SYSTEMQUEST_REPOSITORY_DIR=. cargo run
```

If not set, you'll see this error:
```
❌ Error: No implementation found in pass_all/

This is intentional. You must set the SYSTEMQUEST_REPOSITORY_DIR environment
variable to point to your LRU cache implementation directory.
```

## Quick Start

```bash
# Build the tester
make build

# Run unit tests
cargo test

# Test your implementation (set REPOSITORY_DIR first!)
export SYSTEMQUEST_REPOSITORY_DIR=/path/to/your/impl
./dist/tester

# Clean build artifacts
make clean
```

## Project Structure

```
lru-cache-tester/
├── src/
│   ├── bin/main.rs           # Entry point
│   ├── lib.rs                # Library exports
│   ├── helpers.rs            # CommandRunner for batch mode
│   └── stage_1.rs            # Stage 1 test implementations
├── internal/
│   └── test_helpers/
│       └── pass_stage1/      # Test fixtures for Stage 1
│           ├── systemquest.yml
│           └── your_program.sh
├── dist/
│   └── tester                # Compiled binary (created by build)
├── Makefile                  # Build and test targets
├── test.sh                   # Entry point for CI/CD
└── Cargo.toml                # Rust dependencies
```

## Testing Architecture

This tester follows the CodeCrafters/SystemQuest pattern:

### Local Development (Testing the Tester)

```bash
make test_stage1
```

This runs the tester against `internal/test_helpers/pass_stage1/`, which contains a working implementation.

### Platform Testing (Student Code)

On the SystemQuest platform, the tester is run as follows:

```bash
SYSTEMQUEST_REPOSITORY_DIR=/path/to/student/repo \
SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"s1-basic",...}]' \
./test.sh
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SYSTEMQUEST_REPOSITORY_DIR` | Path to the student's code repository |
| `SYSTEMQUEST_TEST_CASES_JSON` | JSON array of test cases to run |

## Test Cases

### Stage 1: Basic Cache Operations

**Slug**: `s1-basic`

Tests:
- INIT command to create cache with capacity
- PUT command to store key-value pairs
- GET command to retrieve values
- GET for non-existent keys returns NULL
- PUT updates existing keys

## Development Guide

### Adding a New Test

1. **Implement the test function** in `src/stage_X.rs`:
   ```rust
   pub fn test_new_feature(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
       let logger = &harness.logger;
       let executable = &harness.executable;
       
       logger.infof("Testing new feature...");
       
       let mut runner = CommandRunner::new();
       runner.add_command("YOUR_COMMAND");
       
       let output = runner.run_batch(executable)?;
       
       assert_eq!(output[0], "EXPECTED", "Error message");
       
       logger.successf("✓ Test passed");
       Ok(())
   }
   ```

2. **Register the test** in `src/bin/main.rs`:
   ```rust
   definition.add_test_case(TestCase::new(
       "test-slug".to_string(),
       lru_cache_tester::stage_X::test_new_feature,
   ));
   ```

3. **Update Makefile** to add a test target if needed

4. **Run the test**:
   ```bash
   make build
   SYSTEMQUEST_REPOSITORY_DIR=./internal/test_helpers/pass_stageX \
   SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"test-slug","tester_log_prefix":"test","title":"Test Title"}]' \
   ./dist/tester
   ```

### CommandRunner Pattern

The tester uses a **batch stdin/stdout** pattern:

```rust
let mut runner = CommandRunner::new();
runner.add_command("INIT 10");
runner.add_command("PUT key value");
runner.add_command("GET key");

let output = runner.run_batch(executable)?;

// Output is a Vec<String> of all stdout lines
assert_eq!(output[0], "OK");
assert_eq!(output[1], "OK");
assert_eq!(output[2], "value");
```

This approach:
- Sends all commands at once via stdin
- Waits for program to complete
- Parses all output lines
- Simple and suitable for Week 1 testing

## Troubleshooting

### Test fails with "ModuleNotFoundError: No module named 'app'"

This means pipenv was not set up correctly in the test directory. Ensure:

1. The test helper directory has a `Pipfile`
2. Or `your_program.sh` uses `python3 -m app.main` directly without pipenv
3. Or `your_program.sh` points to a directory with pipenv already set up

### Test runs but produces no output

Check that `systemquest.yml` exists in the repository directory and has valid syntax:

```yaml
current_stage: 1
debug: false
```

### "Error finding module specification"

The student's code structure must be:
```
repo/
├── systemquest.yml
├── your_program.sh
└── app/
    └── main.py
```

## Related Documentation

- [TESTING-ARCHITECTURE.md](./TESTING-ARCHITECTURE.md) - Detailed explanation of testing patterns
- [Course Definition](../build-your-own-lru-cache/course-definition.yml)
- [tester-utils-rs](../tester-utils-rs/) - Shared testing utilities

## Dependencies

- Rust 2021 edition
- [tester-utils](../tester-utils-rs/) - Local path dependency
- anyhow, serde, serde_json, tokio

## License

See [LICENSE](../build-your-own-lru-cache/LICENSE)
