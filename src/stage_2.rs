use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::CacheTestCase;
use crate::helpers::CommandRunner;

/// Stage 2: FIFO Eviction
/// 
/// Test FIFO (First In First Out) eviction policy:
/// - When cache reaches capacity, evict the oldest item
/// - Updating a key does NOT change its position in eviction order
/// - SIZE reflects capacity limit after evictions

/// Test FIFO eviction when cache is full
/// 
/// Scenario:
/// 1. Init cache with capacity 2
/// 2. Add 3 items (a, b, c)
/// 3. First item 'a' should be evicted
/// 4. Items 'b' and 'c' should remain
pub fn test_fifo_eviction(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing FIFO eviction",
        vec![
            "INIT 2",
            "PUT a 1",
            "PUT b 2",
            "PUT c 3",  // This should evict 'a' (oldest)
            "GET a",    // Should return NULL (evicted)
            "GET b",    // Should return 2
            "GET c",    // Should return 3
        ],
        vec!["OK", "OK", "OK", "OK", "NULL", "2", "3"],
    )
    .with_hint("In FIFO, the oldest item should be evicted first. When cache is full, adding 'c' should evict 'a' (the first inserted item).")
    .run(harness)
}

/// Test that updating a key doesn't change eviction order (FIFO property)
/// 
/// This is the key difference between FIFO and LRU:
/// - FIFO: Update doesn't change order (still evicts oldest insertion)
/// - LRU: Update moves item to end (makes it "recently used")
/// 
/// Scenario:
/// 1. Init cache with capacity 2
/// 2. Add 'a' and 'b'
/// 3. Update 'a' (should NOT change its position)
/// 4. Add 'c' (should evict 'a', not 'b')
pub fn test_fifo_update_no_reorder(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing FIFO with key updates", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Step 1: Add items in order: a, b", &[]);
    harness.logger.debugf("Step 2: Update 'a' (should NOT change eviction order)", &[]);
    harness.logger.debugf("Step 3: Add 'c' (should evict 'a', the oldest)", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "PUT a 100",  // Update 'a' - in FIFO this doesn't change order
        "PUT c 3",    // This should evict 'a' (oldest insertion)
        "GET a",      // Should return NULL (evicted)
        "GET b",      // Should return 2 (still in cache)
        "GET c",      // Should return 3 (just added)
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "OK", "NULL", "2", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\nHint: In FIFO, updating a key should NOT change its eviction order.\n\
                The eviction order is based on insertion time, not last update time.",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ FIFO correctly maintains insertion order", &[]);
    harness.logger.debugf("  - Update doesn't change eviction priority", &[]);
    harness.logger.debugf("  - Item 'a' evicted (oldest insertion)", &[]);
    
    Ok(())
}

/// Test SIZE command with FIFO eviction
/// 
/// Verify that:
/// - SIZE increases as items are added
/// - SIZE doesn't exceed capacity
/// - SIZE remains at capacity after evictions
pub fn test_fifo_size(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing SIZE with FIFO eviction", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Step 1: Add items one by one, check SIZE", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 3",
        "PUT a 1",
        "SIZE",      // Should be 1
        "PUT b 2",
        "PUT c 3",
        "SIZE",      // Should be 3 (at capacity)
        "PUT d 4",   // Should evict 'a'
        "SIZE",      // Should remain 3 (not exceed capacity)
        "GET a",     // Should return NULL (evicted)
    ])?;
    
    let expected = vec!["OK", "OK", "1", "OK", "OK", "3", "OK", "3", "NULL"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\nHint: SIZE should not exceed capacity even after evictions",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ SIZE correct with FIFO eviction", &[]);
    harness.logger.debugf("  - SIZE grows to capacity", &[]);
    harness.logger.debugf("  - SIZE stays at capacity after evictions", &[]);
    
    Ok(())
}
