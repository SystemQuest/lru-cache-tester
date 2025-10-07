use tester_utils::{TestCaseHarness, TesterError};
use crate::test_case::{CacheTestCase, MultiCacheTestCase};

/// Stage 0 扩展: 使用 MultiCacheTestCase 的示例测试
/// 
/// 这个文件展示如何使用 MultiCacheTestCase 来批量运行多个简单测试

/// 示例: 批量测试基本命令
/// 
/// 这个测试展示了 MultiCacheTestCase 的强大之处：
/// - 一个函数运行 3 个子测试
/// - 自动编号日志前缀 (test-1, test-2, test-3)
/// - 代码从 ~60 行减少到 ~15 行
pub fn test_basic_commands_batch(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    MultiCacheTestCase::new(vec![
        CacheTestCase::new(
            "Testing INIT command",
            vec!["INIT 5"],
            vec!["OK"],
        ),
        
        CacheTestCase::new(
            "Testing PUT command",
            vec!["INIT 5", "PUT a 1"],
            vec!["OK", "OK"],
        ),
        
        CacheTestCase::new(
            "Testing GET command",
            vec!["INIT 5", "PUT a 1", "GET a"],
            vec!["OK", "OK", "1"],
        ),
    ]).run_all(harness)
}

/// 示例: 批量测试不同容量
pub fn test_various_capacities(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    MultiCacheTestCase::new(vec![
        CacheTestCase::new(
            "Capacity 1",
            vec!["INIT 1", "PUT a 1", "GET a"],
            vec!["OK", "OK", "1"],
        ),
        
        CacheTestCase::new(
            "Capacity 5",
            vec!["INIT 5", "PUT a 1", "GET a"],
            vec!["OK", "OK", "1"],
        ),
        
        CacheTestCase::new(
            "Capacity 10",
            vec!["INIT 10", "PUT a 1", "GET a"],
            vec!["OK", "OK", "1"],
        ),
    ]).run_all(harness)
}

/// 示例: 批量测试错误情况（带 Hint）
pub fn test_error_cases_batch(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    MultiCacheTestCase::new(vec![
        CacheTestCase::new(
            "GET non-existent key",
            vec!["INIT 5", "GET missing"],
            vec!["OK", "NULL"],
        ).with_hint("Non-existent keys should return NULL"),
        
        CacheTestCase::new(
            "GET after DELETE",
            vec!["INIT 5", "PUT a 1", "DELETE a", "GET a"],
            vec!["OK", "OK", "OK", "NULL"],
        ).with_hint("Deleted keys should return NULL"),
    ]).run_all(harness)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // 这些测试无法直接运行（需要 TestCaseHarness），
    // 但展示了 MultiCacheTestCase 的用法
    
    #[test]
    fn test_multi_cache_structure() {
        // 验证可以创建 MultiCacheTestCase
        let _multi = MultiCacheTestCase::new(vec![
            CacheTestCase::new("Test 1", vec!["INIT 5"], vec!["OK"]),
            CacheTestCase::new("Test 2", vec!["PUT a 1"], vec!["OK"]),
        ]);
    }
}
