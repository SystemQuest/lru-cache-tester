use tester_utils::{TestCaseHarness, TesterError};
use crate::helpers::CommandRunner;

/// Stage 1: Basic Cache Operations
/// 
/// 测试基本的缓存操作：
/// - INIT: 初始化缓存
/// - PUT: 插入键值对
/// - GET: 获取键值
/// - 不存在的键返回 NULL
pub fn test_basic_cache(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing basic cache operations", &[]);
    
    // 创建 CommandRunner
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    // Test 1: 初始化缓存
    harness.logger.debugf("Test 1: Initialize cache with capacity 10", &[]);
    
    // Test 2: 基本的 PUT/GET 操作
    harness.logger.debugf("Test 2: PUT and GET operations", &[]);
    
    let responses = runner.send_commands(&[
        "INIT 10",
        "PUT name Alice",
        "GET name",
        "GET age",        // 不存在，应返回 NULL
        "PUT name Bob",   // 更新已存在的键
        "GET name",
    ])?;
    
    // 验证响应
    let expected = vec!["OK", "OK", "Alice", "NULL", "OK", "Bob"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'",
                i + 1, expected, actual
            ).into()));
        }
        harness.logger.debugf(&format!("✓ Command {}: {} = {}", i + 1, 
            match i {
                0 => "INIT 10",
                1 => "PUT name Alice",
                2 => "GET name",
                3 => "GET age",
                4 => "PUT name Bob",
                5 => "GET name",
                _ => "",
            },
            actual
        ), &[]);
    }
    
    harness.logger.successf("✓ All basic cache operations passed", &[]);
    
    Ok(())
}

/// Stage 1: Multiple Keys Test
/// 
/// 测试多个键值对的操作
pub fn test_multiple_keys(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing multiple keys operations", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 5",
        "PUT key1 value1",
        "PUT key2 value2",
        "PUT key3 value3",
        "GET key1",
        "GET key2",
        "GET key3",
        "GET key4",  // 不存在
    ])?;
    
    let expected = vec!["OK", "OK", "OK", "OK", "value1", "value2", "value3", "NULL"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'",
                i + 1, expected, actual
            ).into()));
        }
    }
    
    harness.logger.successf("✓ Multiple keys test passed", &[]);
    
    Ok(())
}

/// Stage 1: Key Update Test
/// 
/// 测试更新已存在的键值对
pub fn test_key_update(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
    harness.logger.infof("Testing key update operations", &[]);
    
    let mut runner = CommandRunner::new(harness.executable.clone_executable());
    
    let responses = runner.send_commands(&[
        "INIT 10",
        "PUT name Alice",
        "GET name",
        "PUT name Bob",      // 更新
        "GET name",
        "PUT name Charlie",  // 再次更新
        "GET name",
    ])?;
    
    let expected = vec!["OK", "OK", "Alice", "OK", "Bob", "OK", "Charlie"];
    
    for (i, (actual, expected)) in responses.iter().zip(expected.iter()).enumerate() {
        if actual != expected {
            return Err(TesterError::User(format!(
                "Command {} failed: expected '{}', got '{}'",
                i + 1, expected, actual
            ).into()));
        }
    }
    
    harness.logger.successf("✓ Key update test passed", &[]);
    
    Ok(())
}
