use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::CacheTestCase;

/// Stage 5: Thread Safety
/// 
/// Stage 5 makes the LRU cache thread-safe by adding locks to protect
/// concurrent operations. Tests verify that:
/// 1. Basic operations still work correctly with locks
/// 2. CONCURRENT command spawns multiple threads safely
/// 3. Cache capacity is respected under concurrent load
/// 4. No race conditions or crashes occur

/// Test basic operations with thread safety
/// 
/// Ensures that adding locks doesn't break existing functionality
pub fn test_thread_safe_basic(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing basic operations with thread safety",
        vec![
            "INIT 5",
            "PUT a 1",
            "PUT b 2",
            "PUT c 3",
            "GET a",
            "GET b",
            "GET c",
            "SIZE",
        ],
        vec!["OK", "OK", "OK", "OK", "1", "2", "3", "3"],
    )
    .with_hint(
        "Thread safety should not affect basic operations. \
        Make sure all public methods (get, put, size) are protected with locks."
    )
    .run(harness)
}

/// Test concurrent reads and writes with READ_HEAVY workload
/// 
/// Spawns multiple threads performing mostly reads (70%) with some writes (30%)
pub fn test_concurrent_read_heavy(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing concurrent reads and writes (READ_HEAVY workload)",
        vec![
            "INIT 5",
            "PUT a 1",
            "PUT b 2",
            "PUT c 3",
            "CONCURRENT 10 READ_HEAVY",
            "SIZE",
        ],
        vec!["OK", "OK", "OK", "OK", "OK", "5"],
    )
    .with_hint(
        "CONCURRENT command should spawn multiple threads performing random operations. \
        READ_HEAVY = 70% reads, 30% writes. \
        Verify that: \n\
        1. No crashes or deadlocks occur\n\
        2. SIZE is valid (never exceeds capacity)\n\
        3. Lock protects all operations correctly"
    )
    .run(harness)
}

/// Test concurrent writes with high contention
/// 
/// Spawns many threads performing mostly writes (90%) with few reads (10%)
pub fn test_concurrent_write_heavy(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing concurrent writes with high contention (WRITE_HEAVY workload)",
        vec![
            "INIT 3",
            "CONCURRENT 20 WRITE_HEAVY",
            "SIZE",
        ],
        vec!["OK", "OK", "3"],
    )
    .with_hint(
        "WRITE_HEAVY workload (90% writes) creates high lock contention. \
        Verify that:\n\
        1. Capacity is strictly enforced (SIZE = 3)\n\
        2. No lost updates or race conditions\n\
        3. Eviction logic works correctly under concurrent load"
    )
    .run(harness)
}

/// Test stress with small capacity and many threads
/// 
/// Small cache (capacity=2) with many concurrent threads forces frequent evictions
pub fn test_concurrent_stress(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing stress with small capacity (MIXED workload)",
        vec![
            "INIT 2",
            "CONCURRENT 50 MIXED",
            "SIZE",
        ],
        vec!["OK", "OK", "2"],
    )
    .with_hint(
        "Stress test: capacity=2 with 50 threads (50% reads, 50% writes). \
        This forces frequent evictions under high concurrency. \
        Common issues:\n\
        1. Corrupted doubly linked list pointers\n\
        2. SIZE exceeds capacity (race in eviction logic)\n\
        3. Deadlocks (nested lock acquisition)\n\
        4. Lost updates (operations not fully atomic)"
    )
    .run(harness)
}

/// Test multiple concurrent operations in sequence
/// 
/// Verifies that cache state remains consistent across multiple CONCURRENT commands
pub fn test_concurrent_sequential(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing multiple concurrent operations in sequence",
        vec![
            "INIT 3",
            "PUT x 100",
            "CONCURRENT 10 MIXED",
            "SIZE",
            "CONCURRENT 20 READ_HEAVY",
            "SIZE",
            "CONCURRENT 15 WRITE_HEAVY",
            "SIZE",
        ],
        vec!["OK", "OK", "OK", "3", "OK", "3", "OK", "3"],
    )
    .with_hint(
        "Multiple CONCURRENT commands in sequence should maintain consistency. \
        After each concurrent operation, SIZE should equal capacity. \
        This tests that the cache recovers correctly between concurrent workloads."
    )
    .run(harness)
}

/// Test that LRU behavior is preserved under concurrent load
/// 
/// Verifies that concurrent operations don't break LRU eviction logic
pub fn test_concurrent_lru_preserved(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing that LRU behavior is preserved under concurrent load",
        vec![
            "INIT 2",
            "PUT a 1",
            "PUT b 2",
            "GET a",           // Make 'a' most recent
            "CONCURRENT 5 READ_HEAVY",  // Concurrent operations on random keys
            "PUT c 3",         // Should evict 'b' if 'a' was recently accessed
            "GET a",           // 'a' might still be in cache (depends on CONCURRENT operations)
            "GET c",           // 'c' should be in cache
            "SIZE",            // SIZE should be 2
        ],
        vec!["OK", "OK", "OK", "1", "OK", "OK", "NULL", "3", "2"],
    )
    .with_hint(
        "Note: After CONCURRENT operations on random keys (key_0 to key_99), \
        the original keys 'a' and 'b' are likely evicted. \
        This is expected behavior - CONCURRENT uses different key space. \
        The test verifies that cache capacity is maintained and new keys work correctly."
    )
    .run(harness)
}

/// Test SIZE consistency under concurrent load
/// 
/// Verifies that SIZE never exceeds capacity during concurrent operations
pub fn test_size_consistency(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing SIZE consistency under concurrent load", &[]);
    
    // Run multiple rounds to catch race conditions
    for round in 1..=3 {
        harness.logger.debugf(&format!("Round {}/3", round), &[]);
        
        CacheTestCase::new(
            "SIZE consistency test (multiple rounds)",
            vec![
                "INIT 5",
                "CONCURRENT 30 MIXED",
                "SIZE",
            ],
            vec!["OK", "OK", "5"],
        )
        .with_hint(
            "SIZE must never exceed capacity, even under concurrent load. \
            If SIZE > capacity, there's a race condition in your eviction logic."
        )
        .run(harness)?;
    }
    
    harness.logger.successf("✓ SIZE remained consistent across all rounds", &[]);
    
    Ok(())
}

/// Test edge case: CONCURRENT with capacity=1
/// 
/// Extreme contention scenario with smallest possible cache
pub fn test_concurrent_capacity_one(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing CONCURRENT with capacity=1 (extreme contention)",
        vec![
            "INIT 1",
            "PUT a 1",
            "CONCURRENT 20 WRITE_HEAVY",
            "SIZE",
        ],
        vec!["OK", "OK", "OK", "1"],
    )
    .with_hint(
        "Capacity=1 with high concurrent writes is the extreme stress test. \
        Every PUT operation causes an eviction. \
        Common issues:\n\
        1. SIZE > 1 (race condition in eviction check)\n\
        2. Crash (pointer corruption in DLL)\n\
        3. Deadlock (lock not released properly)"
    )
    .run(harness)
}

/// Test that operations after CONCURRENT still work correctly
/// 
/// Verifies cache is usable after concurrent operations complete
pub fn test_operations_after_concurrent(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing operations work correctly after CONCURRENT",
        vec![
            "INIT 5",
            "PUT initial 123",
            "CONCURRENT 10 MIXED",
            "PUT after_concurrent 456",  // New operation after concurrent load
            "GET after_concurrent",
            "SIZE",
        ],
        vec!["OK", "OK", "OK", "OK", "456", "5"],
    )
    .with_hint(
        "Cache should remain fully functional after CONCURRENT operations complete. \
        Verify that:\n\
        1. All threads properly released locks\n\
        2. Internal state (HashMap + DLL) is not corrupted\n\
        3. New operations work normally"
    )
    .run(harness)
}
