use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::CacheTestCase;

/// Stage 1: Basic Cache Operations
/// 
/// 测试基本的缓存操作：
/// - INIT: 初始化缓存
/// - PUT: 插入键值对
/// - GET: 获取键值
/// - 不存在的键返回 NULL
pub fn test_basic_cache(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing basic cache operations",
        vec![
            "INIT 10",
            "PUT name Alice",
            "GET name",
            "GET age",        // 不存在，应返回 NULL
            "PUT name Bob",   // 更新已存在的键
            "GET name",
        ],
        vec!["OK", "OK", "Alice", "NULL", "OK", "Bob"],
    )
    .with_hint("Basic cache operations: INIT, PUT, GET should work correctly. Non-existent keys should return NULL.")
    .run(harness)
}

/// Stage 1: Multiple Keys Test
/// 
/// 测试多个键值对的操作
pub fn test_multiple_keys(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing multiple keys operations",
        vec![
            "INIT 5",
            "PUT key1 value1",
            "PUT key2 value2",
            "PUT key3 value3",
            "GET key1",
            "GET key2",
            "GET key3",
            "GET key4",  // 不存在
        ],
        vec!["OK", "OK", "OK", "OK", "value1", "value2", "value3", "NULL"],
    )
    .with_hint("Cache should handle multiple different keys independently.")
    .run(harness)
}

/// Stage 1: Key Update Test
/// 
/// 测试更新已存在的键值对
pub fn test_key_update(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing key update operations",
        vec![
            "INIT 10",
            "PUT name Alice",
            "GET name",
            "PUT name Bob",      // 更新
            "GET name",
            "PUT name Charlie",  // 再次更新
            "GET name",
        ],
        vec!["OK", "OK", "Alice", "OK", "Bob", "OK", "Charlie"],
    )
    .with_hint("Updating an existing key should replace its value.")
    .run(harness)
}
