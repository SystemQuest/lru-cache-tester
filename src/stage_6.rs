use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::CacheTestCase;

/// Stage 6: TTL Expiration
/// 
/// Stage 6 adds Time-To-Live (TTL) expiration to cache entries.
/// Tests verify that:
/// 1. Entries with TTL expire after specified time
/// 2. Expired entries return NULL and are removed
/// 3. TTL interacts correctly with LRU eviction
/// 4. GET performs lazy deletion of expired entries
/// 5. Entries without TTL never expire

/// Test basic TTL expiration
/// 
/// Verifies that an entry expires after its TTL and is removed from cache
pub fn test_ttl_basic(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing basic TTL expiration",
        vec![
            "INIT 5",
            "PUT a 1 1",      // TTL = 1 second
            "GET a",          // Should return 1 (not expired yet)
            "SLEEP 1.5",      // Wait for expiration
            "GET a",          // Should return NULL (expired)
            "SIZE",           // Should be 0 (expired entry removed)
        ],
        vec!["OK", "OK", "1", "OK", "NULL", "0"],
    )
    .with_hint(
        "TTL expiration test failed. Make sure:\n\
        1. PUT accepts optional TTL parameter (in seconds)\n\
        2. GET checks if entry expired (time.time() >= expire_at)\n\
        3. Expired entries are removed and NULL is returned\n\
        4. SIZE reflects removal of expired entries"
    )
    .run(harness)
}

/// Test immediate access (entry not expired yet)
/// 
/// Verifies that entry is accessible before TTL expires
pub fn test_ttl_immediate_access(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing immediate access before TTL expiration",
        vec![
            "INIT 5",
            "PUT a 1 10",     // TTL = 10 seconds
            "GET a",          // Immediate access (should succeed)
            "PUT b 2 5",      // TTL = 5 seconds
            "GET b",          // Immediate access (should succeed)
            "SIZE",           // Both entries present
        ],
        vec!["OK", "OK", "1", "OK", "2", "2"],
    )
    .with_hint(
        "Entries should be accessible immediately after PUT. \
        Check that expiration check doesn't trigger false positives."
    )
    .run(harness)
}

/// Test multiple entries with different TTLs
/// 
/// Verifies that entries expire independently based on their individual TTLs
pub fn test_ttl_multiple_different(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing multiple entries with different TTLs",
        vec![
            "INIT 5",
            "PUT short 1 1",   // Expires in 1 second
            "PUT medium 2 3",  // Expires in 3 seconds
            "PUT long 3 10",   // Expires in 10 seconds
            "SLEEP 1.5",       // After 1.5 seconds
            "GET short",       // Expired
            "GET medium",      // Still valid
            "GET long",        // Still valid
            "SIZE",            // 2 entries remain
        ],
        vec!["OK", "OK", "OK", "OK", "OK", "NULL", "2", "3", "2"],
    )
    .with_hint(
        "Each entry should expire based on its own TTL. \
        After 1.5 seconds:\n\
        - 'short' (TTL=1) should be expired\n\
        - 'medium' (TTL=3) and 'long' (TTL=10) should still be valid"
    )
    .run(harness)
}

/// Test TTL with LRU eviction
/// 
/// Verifies that LRU eviction works correctly alongside TTL
pub fn test_ttl_with_eviction(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing TTL with LRU eviction",
        vec![
            "INIT 2",          // Small capacity
            "PUT a 1 10",      // TTL = 10 seconds
            "PUT b 2 10",      // TTL = 10 seconds
            "PUT c 3",         // No TTL, causes eviction
            "GET a",           // Should be NULL (evicted by LRU, not expired)
            "GET c",           // Should return 3
            "SIZE",            // Should be 2
        ],
        vec!["OK", "OK", "OK", "OK", "NULL", "3", "2"],
    )
    .with_hint(
        "LRU eviction should work normally with TTL entries. \
        'a' is evicted because cache is full, not because TTL expired."
    )
    .run(harness)
}

/// Test no TTL (entries never expire)
/// 
/// Verifies that entries without TTL persist indefinitely
pub fn test_ttl_no_expiration(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing entries without TTL (never expire)",
        vec![
            "INIT 5",
            "PUT persistent1 100",  // No TTL
            "PUT persistent2 200",  // No TTL
            "SLEEP 2",              // Wait 2 seconds
            "GET persistent1",      // Should still be valid
            "GET persistent2",      // Should still be valid
            "SIZE",                 // Both entries present
        ],
        vec!["OK", "OK", "OK", "OK", "100", "200", "2"],
    )
    .with_hint(
        "Entries without TTL should never expire. \
        They can only be removed by LRU eviction or explicit deletion."
    )
    .run(harness)
}

/// Test mixed TTL and no-TTL entries
/// 
/// Verifies that entries with and without TTL coexist correctly
pub fn test_ttl_mixed(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing mixed TTL and no-TTL entries",
        vec![
            "INIT 5",
            "PUT persistent 1",     // No TTL
            "PUT temporary 2 1",    // TTL = 1 second
            "SLEEP 1.5",            // Wait for temporary to expire
            "GET persistent",       // Should return 1
            "GET temporary",        // Should return NULL (expired)
            "SIZE",                 // Only persistent remains
        ],
        vec!["OK", "OK", "OK", "OK", "1", "NULL", "1"],
    )
    .with_hint(
        "Entries with TTL should expire, while entries without TTL persist. \
        Check that expire_at is None for entries without TTL."
    )
    .run(harness)
}

/// Test PUT update resets TTL
/// 
/// Verifies that updating an existing key also updates its TTL
pub fn test_ttl_update(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing PUT update resets TTL",
        vec![
            "INIT 5",
            "PUT a 1 2",        // TTL = 2 seconds
            "SLEEP 1",          // Wait 1 second (half-way to expiration)
            "PUT a 2 5",        // Update with new TTL = 5 seconds (resets timer)
            "SLEEP 2.5",        // Wait another 2.5 seconds (total 3.5 from first PUT)
            "GET a",            // Should return 2 (new TTL hasn't expired yet)
            "SIZE",             // Should be 1
        ],
        vec!["OK", "OK", "OK", "OK", "OK", "2", "1"],
    )
    .with_hint(
        "When updating an existing key, the TTL should be reset. \
        In this test:\n\
        - First PUT: a=1 with TTL=2s at time T=0\n\
        - At T=1s: Update to a=2 with TTL=5s (expires at T=6s)\n\
        - At T=3.5s: GET should succeed (not expired yet)"
    )
    .run(harness)
}

/// Test TTL with SIZE consistency
/// 
/// Verifies that SIZE is consistent after lazy deletion
pub fn test_ttl_size_consistency(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing SIZE consistency with TTL expiration",
        vec![
            "INIT 5",
            "PUT a 1 1",
            "PUT b 2 1",
            "PUT c 3 1",
            "SIZE",             // Should be 3 (before expiration)
            "SLEEP 1.5",        // All expire
            "SIZE",             // Should still be 3 (lazy deletion - not accessed yet)
            "GET a",            // NULL (expired and removed)
            "SIZE",             // Should be 2 (a removed)
            "GET b",            // NULL (expired and removed)
            "SIZE",             // Should be 1 (b removed)
            "GET c",            // NULL (expired and removed)
            "SIZE",             // Should be 0 (c removed)
        ],
        vec!["OK", "OK", "OK", "OK", "3", "OK", "3", "NULL", "2", "NULL", "1", "NULL", "0"],
    )
    .with_hint(
        "SIZE reflects actual cache state. \
        With lazy deletion, expired entries remain in cache until accessed. \
        SIZE decreases as expired entries are removed via GET."
    )
    .run(harness)
}

/// Test TTL with concurrent operations
/// 
/// Verifies that TTL works correctly under concurrent access
pub fn test_ttl_concurrent(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing TTL with concurrent operations",
        vec![
            "INIT 10",
            "PUT a 1 5",        // TTL = 5 seconds
            "PUT b 2 5",        // TTL = 5 seconds
            "PUT c 3 5",        // TTL = 5 seconds
            "CONCURRENT 10 READ_HEAVY",  // Concurrent reads/writes
            "SIZE",             // Should be <= 10 (capacity enforced)
            "GET a",            // May or may not exist (depends on CONCURRENT)
            "GET b",            // May or may not exist (depends on CONCURRENT)
            "GET c",            // May or may not exist (depends on CONCURRENT)
        ],
        vec!["OK", "OK", "OK", "OK", "OK", "10", "NULL", "NULL", "NULL"],
    )
    .with_hint(
        "TTL should work correctly with concurrent operations. \
        CONCURRENT command may overwrite a/b/c with random keys. \
        Check that expired entries are removed safely under concurrent access."
    )
    .run(harness)
}
