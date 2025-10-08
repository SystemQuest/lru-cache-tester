use tester_utils::{TestCaseHarness, TesterError};
use crate::helpers::CommandRunner;

/// Stage 4: Custom Doubly Linked List Implementation
/// 
/// Stage 4 implements LRU cache from scratch using HashMap + Doubly Linked List,
/// without relying on language built-ins like OrderedDict/LinkedHashMap.
/// 
/// Functionally identical to Stage 3, but tests implementation correctness
/// and edge cases specific to manual pointer management.

/// Test basic LRU eviction (same as Stage 3)
/// 
/// This ensures Stage 4's manual implementation has the same behavior
/// as Stage 3's OrderedDict implementation.
pub fn test_lru_eviction(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU eviction with custom DLL", &[]);
    
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
    
    let expected = vec!["OK", "OK", "OK", "1", "OK", "1", "NULL", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: Check your doubly linked list pointer updates.\n\
                - Is move_to_head() correctly updating prev/next pointers?\n\
                - Is remove_lru() removing the tail.prev node?",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ LRU eviction working correctly with custom DLL", &[]);
    
    Ok(())
}

/// Test LRU vs FIFO difference (same as Stage 3)
pub fn test_lru_vs_fifo(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing LRU vs FIFO difference with custom DLL", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Verifying PUT updates access order in custom implementation", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "PUT a 100",  // Update 'a' - should move to head in DLL
        "PUT c 3",    // Should evict 'b' (least recent)
        "GET a",      // Should return 100 (retained)
        "GET b",      // Should return NULL (evicted)
        "GET c",      // Should return 3 (just added)
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "OK", "100", "NULL", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: When updating an existing key with PUT:\n\
                - Update the value in the node\n\
                - Call move_to_head() to mark as recently used\n\
                - Don't forget both steps!",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ PUT correctly updates access order in custom DLL", &[]);
    
    Ok(())
}

/// Test multiple access patterns (same as Stage 3)
pub fn test_lru_multiple_access(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing multiple access patterns with custom DLL", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 3",
        "PUT a 1",
        "PUT b 2",
        "PUT c 3",
        "GET a",    // Access 'a'
        "GET b",    // Access 'b'
        "PUT d 4",  // Should evict 'c'
        "GET a",    // Should return 1
        "GET b",    // Should return 2
        "GET c",    // Should return NULL
        "GET d",    // Should return 4
        "SIZE",     // Should be 3
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "1", "2", "OK", "1", "2", "NULL", "4", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: Each GET should call move_to_head().\n\
                Verify your DLL maintains correct order through multiple operations.",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Multiple accesses handled correctly", &[]);
    
    Ok(())
}

/// Test sequential evictions (same as Stage 3)
pub fn test_lru_sequential_evictions(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing sequential evictions with custom DLL", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "PUT b 2",
        "PUT c 3",  // Evicts 'a'
        "PUT d 4",  // Evicts 'b'
        "GET c",    // Access 'c'
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
                Hint: After each eviction, verify:\n\
                - HashMap is updated (del cache[lru_node.key])\n\
                - DLL head/tail pointers are correct\n\
                - No dangling pointers remain",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Sequential evictions maintain correct order", &[]);
    
    Ok(())
}

/// NEW: Test capacity = 1 edge case
/// 
/// This tests the smallest possible cache, which stresses
/// boundary conditions in the DLL implementation.
pub fn test_capacity_one(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing capacity = 1 edge case", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Testing smallest possible cache (capacity = 1)", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 1",
        "PUT a 1",
        "GET a",    // Should return 1
        "PUT b 2",  // Should evict 'a'
        "GET a",    // Should return NULL
        "GET b",    // Should return 2
        "SIZE",     // Should be 1
        "PUT c 3",  // Should evict 'b'
        "GET b",    // Should return NULL
        "GET c",    // Should return 3
    ])?;
    
    let expected = vec!["OK", "OK", "1", "OK", "NULL", "2", "1", "OK", "NULL", "3"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: With capacity=1, every PUT should evict the previous item.\n\
                - Check if your DLL handles single-node cases correctly\n\
                - Verify head.next == tail.prev when size=1",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Capacity = 1 edge case handled correctly", &[]);
    
    Ok(())
}

/// NEW: Test empty cache operations
/// 
/// Verifies correct handling of operations on an empty cache.
pub fn test_empty_cache(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing empty cache operations", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Testing operations on empty cache", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 3",
        "GET nonexistent",  // Should return NULL
        "SIZE",             // Should be 0
        "PUT a 1",
        "SIZE",             // Should be 1
        "GET a",            // Should return 1
    ])?;
    
    let expected = vec!["OK", "NULL", "0", "OK", "1", "1"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: Empty cache should:\n\
                - Return NULL for any GET\n\
                - Return 0 for SIZE\n\
                - Handle first PUT correctly (initialize DLL)",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Empty cache operations work correctly", &[]);
    
    Ok(())
}

/// NEW: Test repeated operations on same key
/// 
/// Stresses the move_to_head operation with repeated accesses.
pub fn test_repeated_operations(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing repeated operations on same key", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Testing repeated GET/PUT on same key", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        "PUT a 1",
        "GET a",    // Access 1
        "GET a",    // Access 2
        "GET a",    // Access 3
        "PUT a 2",  // Update
        "PUT a 3",  // Update again
        "GET a",    // Should return 3
        "SIZE",     // Should still be 1
        "PUT b 4",
        "PUT c 5",  // Should evict 'a' (LRU: a is oldest, added before b)
        "GET a",    // Should return NULL (a was evicted)
        "GET b",    // Should return 4
    ])?;
    
    let expected = vec!["OK", "OK", "1", "1", "1", "OK", "OK", "3", "1", "OK", "OK", "NULL", "4"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: Repeated operations on same key should:\n\
                - Not create duplicate nodes in DLL\n\
                - Correctly update value in place\n\
                - Still maintain LRU order (most recent operation wins)\n\
                - When evicting, remove the node that was least recently accessed",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Repeated operations maintain correct LRU order", &[]);
    
    Ok(())
}

/// NEW: Test full eviction cycle
/// 
/// Fill cache, evict all items, then refill.
/// Tests that DLL can recover from being emptied.
pub fn test_full_eviction_cycle(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing full eviction and refill cycle", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    harness.logger.debugf("Fill -> evict all -> refill cycle", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 2",
        // Fill cache
        "PUT a 1",
        "PUT b 2",
        "SIZE",     // Should be 2
        // Evict all
        "PUT c 3",  // Evicts 'a'
        "PUT d 4",  // Evicts 'b'
        "GET a",    // NULL
        "GET b",    // NULL
        "SIZE",     // Should still be 2
        // Refill
        "PUT e 5",  // Evicts 'c'
        "PUT f 6",  // Evicts 'd'
        "GET e",    // Should return 5
        "GET f",    // Should return 6
        "SIZE",     // Should be 2
    ])?;
    
    let expected = vec![
        "OK", "OK", "OK", "2",      // Fill
        "OK", "OK", "NULL", "NULL", "2",  // Evict
        "OK", "OK", "5", "6", "2"   // Refill
    ];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'\n\
                Hint: After evicting all original items:\n\
                - DLL should still be valid (head.next != tail, tail.prev != head)\n\
                - SIZE should remain at capacity\n\
                - New items should work correctly",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {}", i + 1, actual), &[]);
    }
    
    harness.logger.successf("✓ Full eviction cycle works correctly", &[]);
    
    Ok(())
}
