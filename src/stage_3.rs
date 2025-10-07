use tester_utils::{TestCaseHarness, TesterError};
use crate::helpers::CommandRunner;

/// Stage 3: LRU Eviction
/// 
/// Test LRU (Least Recently Used) eviction policy:
/// - When cache reaches capacity, evict the least recently used item
/// - GET operation updates access time (moves item to "most recent")
/// - PUT operation also updates access time for existing keys
/// - Key difference from FIFO: access order matters!

/// Test basic LRU eviction
/// 
/// Scenario:
/// 1. Init cache with capacity 2
/// 2. Add 'a' and 'b'
/// 3. Access 'a' (moves 'a' to most recent)
/// 4. Add 'c' (should evict 'b', not 'a')
/// 
/// This demonstrates the core LRU behavior: accessing an item
/// prevents it from being evicted.
pub fn test_lru_eviction(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU eviction", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Step 1: Initialize cache with capacity 2", &[]);
    harness.logger.debugf("Step 2: Add 'a' and 'b'", &[]);
    harness.logger.debugf("Step 3: Access 'a' (updates access time)", &[]);
    harness.logger.debugf("Step 4: Add 'c' (should evict 'b', the least recently used)", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "GET a",    // Access 'a' - makes it "recently used"
        "PUT c 3",  // This should evict 'b' (least recently used)
        "GET a",    // Should return 1 (still in cache)
        "GET b",    // Should return NULL (evicted)
        "GET c",    // Should return 3 (just added)
    ])?;
    
    // Verify responses
    let expected = vec!["OK", "OK", "OK", "1", "OK", "1", "NULL", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: In LRU, accessing an item (GET) should make it 'recently used'.\n\
                When cache is full, the least recently accessed item should be evicted.",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ LRU eviction working correctly", &[]);
    harness.logger.debugf("  - GET operation updated access time for 'a'", &[]);
    harness.logger.debugf("  - Least recently used item 'b' was evicted", &[]);
    harness.logger.debugf("  - Recently accessed item 'a' was retained", &[]);
    
    Ok(())
}

/// Test LRU vs FIFO difference
/// 
/// This test directly compares LRU and FIFO behavior with the same
/// sequence of operations. The key difference is how GET affects eviction.
/// 
/// Scenario (identical to Stage 2 FIFO test):
/// 1. Init capacity 2
/// 2. Add 'a', then 'b'
/// 3. Update 'a' value
/// 4. Add 'c'
/// 
/// Expected behavior:
/// - FIFO: Would evict 'a' (oldest insertion)
/// - LRU: Should evict 'b' (least recently used, since 'a' was just updated)
pub fn test_lru_vs_fifo(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU vs FIFO difference", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Step 1: Add items in order: a, b", &[]);
    harness.logger.debugf("Step 2: Update 'a' (in LRU this moves 'a' to most recent)", &[]);
    harness.logger.debugf("Step 3: Add 'c' (should evict 'b', not 'a')", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "PUT a 100",  // Update 'a' - in LRU this makes 'a' most recent
        "PUT c 3",    // This should evict 'b' (least recent)
        "GET a",      // Should return 100 (retained, was just updated)
        "GET b",      // Should return NULL (evicted)
        "GET c",      // Should return 3 (just added)
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "OK", "100", "NULL", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: This is the key difference between LRU and FIFO!\n\
                - FIFO: Update doesn't change eviction order (would evict 'a')\n\
                - LRU: Update makes item 'recently used' (should evict 'b')",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ LRU correctly differs from FIFO", &[]);
    harness.logger.debugf("  - PUT operation updated access time for 'a'", &[]);
    harness.logger.debugf("  - Item 'b' evicted (least recently accessed)", &[]);
    harness.logger.debugf("  - Item 'a' retained (recently updated)", &[]);
    
    Ok(())
}

/// Test multiple GET operations update access order
/// 
/// Scenario:
/// 1. Init capacity 3
/// 2. Add 'a', 'b', 'c' (cache full)
/// 3. Access 'a' and 'b' (moves them to recent)
/// 4. Add 'd' (should evict 'c', the only item not accessed)
pub fn test_lru_multiple_access(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU with multiple access patterns", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Step 1: Fill cache with a, b, c", &[]);
    harness.logger.debugf("Step 2: Access 'a' and 'b' (updates their access time)", &[]);
    harness.logger.debugf("Step 3: Add 'd' (should evict 'c', the only unaccessed item)", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 3",
        "PUT a 1",
        "PUT b 2",
        "PUT c 3",
        "GET a",    // Access 'a'
        "GET b",    // Access 'b'
        "PUT d 4",  // Should evict 'c' (least recently used)
        "GET a",    // Should return 1
        "GET b",    // Should return 2
        "GET c",    // Should return NULL (evicted)
        "GET d",    // Should return 4
        "SIZE",     // Should be 3
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "1", "2", "OK", "1", "2", "NULL", "4", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: Multiple GET operations should all update access time.\n\
                The item that hasn't been accessed should be evicted first.",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ LRU handles multiple accesses correctly", &[]);
    harness.logger.debugf("  - Items 'a' and 'b' retained (recently accessed)", &[]);
    harness.logger.debugf("  - Item 'c' evicted (not accessed after insertion)", &[]);
    
    Ok(())
}

/// Test LRU with sequential evictions
/// 
/// Verify that after each eviction, the access order is correctly maintained
/// for subsequent evictions.
/// 
/// Scenario:
/// 1. Capacity 2, add a, b
/// 2. Add c (evicts a)
/// 3. Add d (evicts b)
/// 4. Access c
/// 5. Add e (evicts d, not c)
pub fn test_lru_sequential_evictions(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU with sequential evictions", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Testing sequence: a, b, c(evict a), d(evict b), GET c, e(evict d)", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "PUT c 3",  // Evicts 'a'
        "PUT d 4",  // Evicts 'b'
        "GET c",    // Access 'c', makes it recent
        "PUT e 5",  // Should evict 'd' (not 'c')
        "GET c",    // Should return 3
        "GET d",    // Should return NULL
        "GET e",    // Should return 5
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "OK", "3", "OK", "3", "NULL", "5"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: LRU should maintain correct access order through multiple evictions.",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Sequential evictions maintain correct LRU order", &[]);
    
    Ok(())
}
