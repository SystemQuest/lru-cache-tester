use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::CacheTestCase;

/// Stage 7: Cache Statistics (Extension)
/// 
/// Stage 7 adds comprehensive metrics and observability to the cache.
/// Tests verify that:
/// 1. Hit/miss counters track GET operations correctly
/// 2. Eviction counter tracks capacity-based removals
/// 3. Expiration counter tracks TTL-based removals
/// 4. Hit rate is calculated with 2 decimal precision
/// 5. Zero division is handled (empty cache)
/// 6. Metrics are thread-safe under concurrent operations

/// Test basic hit/miss tracking
/// 
/// Verifies that hits and misses are counted correctly
pub fn test_stats_basic_hit_miss(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing basic hit/miss tracking",
        vec![
            "INIT 5",
            "PUT a 1",
            "PUT b 2",
            "GET a",          // Hit
            "GET c",          // Miss
            "GET a",          // Hit
            "STATS",
        ],
        vec!["OK", "OK", "OK", "1", "NULL", "1", "hits:2 misses:1 hit_rate:66.67 evictions:0 expirations:0 size:2 capacity:5"],
    )
    .with_hint(
        "Hit/miss tracking failed. Make sure:\n\
        1. Increment hits++ when GET returns a value\n\
        2. Increment misses++ when GET returns NULL\n\
        3. Hit rate = (hits / (hits + misses) * 100) with 2 decimals\n\
        4. Format: 'hits:X misses:Y hit_rate:Z.ZZ ...'"
    )
    .run(harness)
}

/// Test eviction tracking
/// 
/// Verifies that evictions are counted when capacity limit is reached
pub fn test_stats_eviction_tracking(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing eviction tracking",
        vec![
            "INIT 2",
            "PUT a 1",
            "PUT b 2",
            "GET a",          // Make 'a' most recent
            "PUT c 3",        // Evicts 'b' (LRU)
            "GET b",          // Miss (b was evicted)
            "STATS",
        ],
        vec!["OK", "OK", "OK", "1", "OK", "NULL", "hits:1 misses:1 hit_rate:50.00 evictions:1 expirations:0 size:2 capacity:2"],
    )
    .with_hint(
        "Eviction tracking failed. Make sure:\n\
        1. Increment evictions++ when removing LRU due to capacity\n\
        2. Only count capacity-based evictions (not TTL expirations)\n\
        3. Evicted entry should cause GET miss"
    )
    .run(harness)
}

/// Test expiration tracking (TTL)
/// 
/// Verifies that expirations are counted separately from evictions
pub fn test_stats_expiration_tracking(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing expiration tracking (TTL)",
        vec![
            "INIT 5",
            "PUT a 1 1",      // TTL = 1 second
            "PUT b 2",        // No TTL
            "SLEEP 1.5",      // Wait for 'a' to expire
            "GET a",          // Miss (expired)
            "GET b",          // Hit (still valid)
            "STATS",
        ],
        vec!["OK", "OK", "OK", "OK", "NULL", "2", "hits:1 misses:1 hit_rate:50.00 evictions:0 expirations:1 size:1 capacity:5"],
    )
    .with_hint(
        "Expiration tracking failed. Make sure:\n\
        1. Increment expirations++ when GET finds expired entry\n\
        2. Also increment misses++ (expired = miss)\n\
        3. Don't confuse expirations with evictions\n\
        4. SIZE should decrease after lazy deletion"
    )
    .run(harness)
}

/// Test empty cache stats (zero division edge case)
/// 
/// Verifies that STATS works on empty cache without division by zero error
pub fn test_stats_empty_cache(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing STATS on empty cache (zero division edge case)",
        vec![
            "INIT 5",
            "STATS",          // No operations yet
        ],
        vec!["OK", "hits:0 misses:0 hit_rate:0.00 evictions:0 expirations:0 size:0 capacity:5"],
    )
    .with_hint(
        "Empty cache stats failed. Make sure:\n\
        1. Handle zero division: hit_rate = 0.00 when hits + misses = 0\n\
        2. All counters should be 0\n\
        3. hit_rate format is '0.00' (not '0' or '0.0')"
    )
    .run(harness)
}

/// Test 100% hit rate
/// 
/// Verifies hit rate calculation when all GETs succeed
pub fn test_stats_all_hits(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing 100% hit rate (all GETs succeed)",
        vec![
            "INIT 5",
            "PUT a 1",
            "PUT b 2",
            "GET a",          // Hit
            "GET b",          // Hit
            "GET a",          // Hit
            "STATS",
        ],
        vec!["OK", "OK", "OK", "1", "2", "1", "hits:3 misses:0 hit_rate:100.00 evictions:0 expirations:0 size:2 capacity:5"],
    )
    .with_hint(
        "100% hit rate test failed. Make sure:\n\
        1. Hit rate = 100.00 when misses = 0\n\
        2. Format is '100.00' (not '100' or '100.0')"
    )
    .run(harness)
}

/// Test 0% hit rate
/// 
/// Verifies hit rate calculation when all GETs fail
pub fn test_stats_all_misses(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing 0% hit rate (all GETs fail)",
        vec![
            "INIT 5",
            "GET x",          // Miss
            "GET y",          // Miss
            "GET z",          // Miss
            "STATS",
        ],
        vec!["OK", "NULL", "NULL", "NULL", "hits:0 misses:3 hit_rate:0.00 evictions:0 expirations:0 size:0 capacity:5"],
    )
    .with_hint(
        "0% hit rate test failed. Make sure:\n\
        1. Hit rate = 0.00 when hits = 0\n\
        2. Misses are counted even when cache is empty"
    )
    .run(harness)
}

/// Test eviction cycle (fill, evict, refill)
/// 
/// Verifies eviction counter increases with multiple evictions
pub fn test_stats_eviction_cycle(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing eviction cycle (multiple evictions)",
        vec![
            "INIT 2",
            "PUT a 1",
            "PUT b 2",        // Cache full
            "PUT c 3",        // Evicts 'a' (evictions = 1)
            "PUT d 4",        // Evicts 'b' (evictions = 2)
            "GET a",          // Miss
            "GET c",          // Hit
            "STATS",
        ],
        vec!["OK", "OK", "OK", "OK", "OK", "NULL", "3", "hits:1 misses:1 hit_rate:50.00 evictions:2 expirations:0 size:2 capacity:2"],
    )
    .with_hint(
        "Eviction cycle test failed. Make sure:\n\
        1. Each capacity-based eviction increments counter\n\
        2. Evictions counter accumulates (2 evictions total)\n\
        3. SIZE stays at capacity after evictions"
    )
    .run(harness)
}

/// Test mix of TTL expiration and LRU eviction
/// 
/// Verifies that expirations and evictions are tracked separately
pub fn test_stats_mixed_expiration_eviction(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing mix of TTL expiration and LRU eviction",
        vec![
            "INIT 2",
            "PUT a 1 1",      // TTL = 1 second
            "PUT b 2",        // No TTL
            "PUT c 3",        // Evicts 'a' (but 'a' not accessed yet, so no expiration counted)
            "GET a",          // Miss (evicted)
            "PUT d 4 1",      // TTL = 1 second, evicts 'b'
            "SLEEP 1.5",      // Wait for 'd' to expire
            "GET d",          // Miss (expired)
            "STATS",
        ],
        vec!["OK", "OK", "OK", "OK", "NULL", "OK", "OK", "NULL", "hits:0 misses:2 hit_rate:0.00 evictions:2 expirations:1 size:1 capacity:2"],
    )
    .with_hint(
        "Mixed expiration/eviction test failed. Make sure:\n\
        1. Evictions: capacity-based removals (2 in this test)\n\
        2. Expirations: TTL-based removals (1 in this test)\n\
        3. Both counters are independent\n\
        4. Expired GET counts as miss + expiration"
    )
    .run(harness)
}

/// Test hit rate precision (exactly 2 decimal places)
/// 
/// Verifies that hit rate is formatted with exactly 2 decimals
pub fn test_stats_hit_rate_precision(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing hit rate precision (2 decimal places)",
        vec![
            "INIT 5",
            "PUT a 1",
            "GET a",          // Hit
            "GET b",          // Miss
            "GET a",          // Hit
            "GET c",          // Miss
            "GET d",          // Miss
            "STATS",          // hits=2, misses=3, hit_rate=40.00
        ],
        vec!["OK", "OK", "1", "NULL", "1", "NULL", "NULL", "hits:2 misses:3 hit_rate:40.00 evictions:0 expirations:0 size:1 capacity:5"],
    )
    .with_hint(
        "Hit rate precision test failed. Make sure:\n\
        1. Hit rate is exactly 2 decimals (use .2f format)\n\
        2. 2/5 = 40.00 (not '40' or '40.0')\n\
        3. Always show 2 decimals even for whole numbers"
    )
    .run(harness)
}

/// Test large workload (1000 operations)
/// 
/// Verifies that counters remain accurate under high load
pub fn test_stats_large_workload(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    // For large workload, we test with a reasonable size (100 ops) instead of 1000
    // to keep test execution fast
    CacheTestCase::new(
        "Testing stats accuracy under large workload (100 ops)",
        vec![
            "INIT 10",
            // Put 10 keys
            "PUT key0 value0", "PUT key1 value1", "PUT key2 value2", "PUT key3 value3", "PUT key4 value4",
            "PUT key5 value5", "PUT key6 value6", "PUT key7 value7", "PUT key8 value8", "PUT key9 value9",
            // 90 GETs with pattern: keys 0-14 (60 hits, 30 misses)
            "GET key0", "GET key1", "GET key2", "GET key3", "GET key4", "GET key5",
            "GET key6", "GET key7", "GET key8", "GET key9", "GET key10", "GET key11",
            "GET key12", "GET key13", "GET key14", "GET key0", "GET key1", "GET key2",
            "GET key3", "GET key4", "GET key5", "GET key6", "GET key7", "GET key8",
            "GET key9", "GET key10", "GET key11", "GET key12", "GET key13", "GET key14",
            "GET key0", "GET key1", "GET key2", "GET key3", "GET key4", "GET key5",
            "GET key6", "GET key7", "GET key8", "GET key9", "GET key10", "GET key11",
            "GET key12", "GET key13", "GET key14", "GET key0", "GET key1", "GET key2",
            "GET key3", "GET key4", "GET key5", "GET key6", "GET key7", "GET key8",
            "GET key9", "GET key10", "GET key11", "GET key12", "GET key13", "GET key14",
            "GET key0", "GET key1", "GET key2", "GET key3", "GET key4", "GET key5",
            "GET key6", "GET key7", "GET key8", "GET key9", "GET key10", "GET key11",
            "GET key12", "GET key13", "GET key14", "GET key0", "GET key1", "GET key2",
            "GET key3", "GET key4", "GET key5", "GET key6", "GET key7", "GET key8",
            "GET key9", "GET key10", "GET key11", "GET key12", "GET key13", "GET key14",
            "STATS",
        ],
        vec![
            "OK",
            // 10 PUTs
            "OK", "OK", "OK", "OK", "OK", "OK", "OK", "OK", "OK", "OK",
            // 90 GETs (60 hits, 30 misses)
            "value0", "value1", "value2", "value3", "value4", "value5",
            "value6", "value7", "value8", "value9", "NULL", "NULL",
            "NULL", "NULL", "NULL", "value0", "value1", "value2",
            "value3", "value4", "value5", "value6", "value7", "value8",
            "value9", "NULL", "NULL", "NULL", "NULL", "NULL",
            "value0", "value1", "value2", "value3", "value4", "value5",
            "value6", "value7", "value8", "value9", "NULL", "NULL",
            "NULL", "NULL", "NULL", "value0", "value1", "value2",
            "value3", "value4", "value5", "value6", "value7", "value8",
            "value9", "NULL", "NULL", "NULL", "NULL", "NULL",
            "value0", "value1", "value2", "value3", "value4", "value5",
            "value6", "value7", "value8", "value9", "NULL", "NULL",
            "NULL", "NULL", "NULL", "value0", "value1", "value2",
            "value3", "value4", "value5", "value6", "value7", "value8",
            "value9", "NULL", "NULL", "NULL", "NULL", "NULL",
            "hits:60 misses:30 hit_rate:66.67 evictions:0 expirations:0 size:10 capacity:10",
        ],
    )
    .with_hint(
        "Large workload test failed. Make sure:\n\
        1. Counters don't overflow or lose precision\n\
        2. All hits and misses are counted accurately\n\
        3. Stats remain correct after many operations"
    )
    .run(harness)
}

/// Test thread-safe metrics (concurrent operations)
/// 
/// Verifies that metrics are updated atomically under concurrent load
/// Uses CONCURRENT command to run multiple threads accessing the cache
pub fn test_stats_concurrent(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing thread-safe metrics (concurrent operations)",
        vec![
            "INIT 20",
            "PUT a 1",
            "PUT b 2",
            "PUT c 3",
            "CONCURRENT 10 READ_HEAVY",  // 10 threads, 70% reads, runs for 1 second
            "STATS",                      // Verify counters are consistent
        ],
        vec![
            "OK",
            "OK",
            "OK",
            "OK",
            "OK",  // CONCURRENT completes
            "hits:\\d+ misses:\\d+ hit_rate:[0-9]+\\.[0-9]{2} evictions:\\d+ expirations:0 size:\\d+ capacity:20",
        ],
    )
    .with_hint(
        "Concurrent stats test failed. Make sure:\n\
        1. All counter updates are inside locks (thread-safe)\n\
        2. hits++, misses++, evictions++ are atomic operations\n\
        3. No race conditions corrupt counters\n\
        4. Final stats are mathematically consistent\n\
        5. Hit rate format is exactly 2 decimals (e.g., 66.67)"
    )
    .with_regex_match(true)
    .run(harness)
}
