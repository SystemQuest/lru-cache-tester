use tester_utils::{TestCaseHarness, TesterError};
use crate::helpers::CommandRunner;
use crate::test_case::CacheTestCase;

/// Stage 0: Edge Cases and Error Handling
/// 
/// 测试边界情况和错误处理：
/// - 边界容量（capacity = 1）
/// - 空键和空值
/// - 错误操作顺序（未 INIT 就操作）

/// 测试容量为 1 的边界情况
pub fn test_capacity_one(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    CacheTestCase::new(
        "Testing edge case: capacity = 1",
        vec!["INIT 1", "PUT a 1", "GET a", "PUT b 2", "GET a", "GET b"],
        vec!["OK", "OK", "1", "OK", "NULL", "2"],
    )
    .with_hint("With capacity=1, inserting a new key should immediately evict the existing key.")
    .run(harness)
}

/// 测试空值处理
pub fn test_empty_values(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing edge case: empty values", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 5",
        "PUT key ",     // 空值（注意有一个空格）
        "GET key",      // 应该返回空字符串或处理空值
        "PUT empty",    // 缺少值 - 可能是错误格式
    ])?;
    
    // 基础验证：至少不应该崩溃
    harness.logger.debugf(&format!("Responses: {:?}", responses), &[]);
    
    harness.logger.successf("✓ Empty values handled without crash", &[]);
    Ok(())
}

/// 测试错误操作顺序：未 INIT 就操作
pub fn test_no_init(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing error handling: operations before INIT", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    // 尝试在 INIT 前执行操作
    let result = runner.send_commands(&[
        "PUT name Alice",  // 应该失败或返回错误
        "GET name",
    ]);
    
    // 我们期望要么返回错误响应，要么程序报错
    match result {
        Ok(responses) => {
            // 如果返回了响应，检查是否是错误标识
            harness.logger.debugf(&format!("Responses: {:?}", responses), &[]);
            
            // 检查是否有错误标识（ERROR/FAIL/等）
            let has_error = responses.iter().any(|r| 
                r.contains("ERROR") || r.contains("FAIL") || r == "NULL"
            );
            
            if !has_error {
                harness.logger.debugf(
                    &format!("Warning: Operations before INIT should return errors, got: {:?}", responses),
                    &[]
                );
            }
        }
        Err(_) => {
            // 程序退出也是合理的错误处理
            harness.logger.debugf("Program exited (expected behavior)", &[]);
        }
    }
    
    harness.logger.successf("✓ Error handling test completed", &[]);
    Ok(())
}

/// 测试重复 INIT
pub fn test_double_init(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing error handling: double INIT", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 10",
        "PUT a 1",
        "INIT 5",      // 第二次 INIT - 应该重置缓存或返回错误
        "GET a",       // 如果重置了，应该是 NULL
    ])?;
    
    harness.logger.debugf(&format!("Responses: {:?}", responses), &[]);
    
    // 基础验证：记录行为但不强制要求特定实现
    if responses.len() >= 4 {
        let final_get = &responses[3];
        if final_get == "NULL" {
            harness.logger.debugf("Double INIT resets cache (NULL returned)", &[]);
        } else {
            harness.logger.debugf("Double INIT preserves state or returns error", &[]);
        }
    }
    
    harness.logger.successf("✓ Double INIT handled", &[]);
    Ok(())
}
