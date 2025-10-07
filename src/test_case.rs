use tester_utils::{TestCaseHarness, TesterError};
use crate::helpers::CommandRunner;
use crate::assertions::{Assertion, ExactMatchAssertion};

/// CacheTestCase - 测试用例抽象
/// 
/// 设计目标:
/// - 减少重复代码 (每个测试 ~50 行 → ~10 行)
/// - 统一验证逻辑和错误处理
/// - 声明式测试定义
/// 
/// 借鉴自 CodeCrafters http-server-tester 的 SendRequestTestCase 设计
/// 
/// v2.0 更新: 引入 Assertion 抽象层
/// - 分离验证逻辑
/// - 支持更友好的逐行输出
/// - 可扩展支持不同验证策略
pub struct CacheTestCase {
    /// 测试描述（用于日志）
    pub description: &'static str,
    
    /// 要发送的命令列表
    pub commands: Vec<&'static str>,
    
    /// 期望的响应列表（必须与 commands 长度相同）
    pub expected_responses: Vec<&'static str>,
    
    /// 失败时的提示信息（可选，用于教学性错误提示）
    pub hint: Option<&'static str>,
    
    /// 是否显示详细的命令执行日志（默认 false）
    /// 注意: 启用 verbose 会禁用 Assertion 的友好输出
    pub verbose: bool,
}

impl CacheTestCase {
    /// 创建一个新的测试用例
    pub fn new(
        description: &'static str,
        commands: Vec<&'static str>,
        expected_responses: Vec<&'static str>,
    ) -> Self {
        Self {
            description,
            commands,
            expected_responses,
            hint: None,
            verbose: false,
        }
    }
    
    /// 添加提示信息（用于失败时的教学性提示）
    pub fn with_hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }
    
    /// 启用详细日志（显示每个命令的执行结果）
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    /// 运行测试用例
    /// 
    /// 工作流程:
    /// 1. 创建 CommandRunner
    /// 2. 发送所有命令
    /// 3. 使用 Assertion 验证响应（提供友好的逐行输出）
    /// 4. 返回结果
    pub fn run(&self, harness: &mut TestCaseHarness) -> Result<(), TesterError> {
        // 1. 日志: 开始测试
        harness.logger.infof(self.description, &[]);
        
        // 2. 验证测试用例有效性
        if self.commands.len() != self.expected_responses.len() {
            return Err(TesterError::Configuration(format!(
                "Test case configuration error: commands count ({}) != expected responses count ({})",
                self.commands.len(),
                self.expected_responses.len()
            )));
        }
        
        // 3. 创建 CommandRunner 并发送命令
        let mut runner = CommandRunner::new(harness.executable.clone_executable());
        let responses = runner.send_commands(&self.commands)?;
        
        // 4. 使用 Assertion 验证响应
        if self.verbose {
            // Verbose 模式: 使用旧的验证逻辑（保留向后兼容）
            for (i, (actual, expected)) in responses.iter().zip(self.expected_responses.iter()).enumerate() {
                if actual != expected {
                    let mut error_msg = format!(
                        "Command {} failed: expected '{}', got '{}'\n\
                        Command: {}",
                        i + 1, expected, actual, self.commands[i]
                    );
                    
                    if let Some(hint) = self.hint {
                        error_msg.push_str(&format!("\n\nHint: {}", hint));
                    }
                    
                    return Err(TesterError::User(error_msg.into()));
                }
                
                harness.logger.debugf(&format!(
                    "✓ Command {}: {} → {}",
                    i + 1,
                    self.commands[i],
                    actual
                ), &[]);
            }
            
            harness.logger.successf(&format!("✓ {}", self.description), &[]);
        } else {
            // 默认模式: 使用 Assertion 抽象（友好的逐行输出）
            let expected: Vec<String> = self.expected_responses.iter()
                .map(|s| s.to_string())
                .collect();
            
            let commands: Vec<String> = self.commands.iter()
                .map(|s| s.to_string())
                .collect();
            
            let assertion = ExactMatchAssertion::new(expected)
                .with_commands(commands);
            
            // 使用 Assertion 验证，如果失败会添加 Hint
            assertion.verify(&responses, &harness.logger).map_err(|err| {
                if let Some(hint) = self.hint {
                    TesterError::User(format!("{}\n\nHint: {}", err, hint).into())
                } else {
                    err
                }
            })?;
        }
        
        Ok(())
    }
}

/// CacheTestCaseBuilder - 构建器模式 (可选，提供更流畅的 API)
/// 
/// 使用示例:
/// ```rust,no_run
/// # use lru_cache_tester::test_case::CacheTestCaseBuilder;
/// # use tester_utils::TestCaseHarness;
/// # fn example(harness: &mut TestCaseHarness) -> Result<(), Box<dyn std::error::Error>> {
/// CacheTestCaseBuilder::new("Testing basic operations")
///     .commands(vec!["INIT 10", "PUT a 1", "GET a"])
///     .expect(vec!["OK", "OK", "1"])
///     .hint("Basic operations should work correctly")
///     .verbose()
///     .build()
///     .run(harness)?;
/// # Ok(())
/// # }
/// ```
pub struct CacheTestCaseBuilder {
    description: Option<&'static str>,
    commands: Option<Vec<&'static str>>,
    expected_responses: Option<Vec<&'static str>>,
    hint: Option<&'static str>,
    verbose: bool,
}

impl CacheTestCaseBuilder {
    pub fn new(description: &'static str) -> Self {
        Self {
            description: Some(description),
            commands: None,
            expected_responses: None,
            hint: None,
            verbose: false,
        }
    }
    
    pub fn commands(mut self, commands: Vec<&'static str>) -> Self {
        self.commands = Some(commands);
        self
    }
    
    pub fn expect(mut self, expected_responses: Vec<&'static str>) -> Self {
        self.expected_responses = Some(expected_responses);
        self
    }
    
    pub fn hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }
    
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    pub fn build(self) -> CacheTestCase {
        CacheTestCase {
            description: self.description.expect("description is required"),
            commands: self.commands.expect("commands are required"),
            expected_responses: self.expected_responses.expect("expected_responses are required"),
            hint: self.hint,
            verbose: self.verbose,
        }
    }
}

// ============================================================================
// MultiCacheTestCase - 批量测试执行
// ============================================================================

/// MultiCacheTestCase - 批量运行多个测试用例
/// 
/// 设计目标:
/// - 在一个 stage 函数中运行多个子测试
/// - 自动编号日志前缀（test-1, test-2, ...）
/// - 失败时保留日志前缀便于定位
/// 
/// 借鉴自 Interpreter Tester 的 MultiTestCase 设计
/// 
/// 使用示例:
/// ```rust,ignore
/// pub fn test_multiple_scenarios(harness: &mut TestCaseHarness) -> Result<(), TesterError> {
///     MultiCacheTestCase::new(vec![
///         CacheTestCase::new("Test 1", vec!["INIT 5"], vec!["OK"]),
///         CacheTestCase::new("Test 2", vec!["PUT a 1"], vec!["OK"]),
///         CacheTestCase::new("Test 3", vec!["GET a"], vec!["1"]),
///     ]).run_all(harness)
/// }
/// ```
pub struct MultiCacheTestCase {
    test_cases: Vec<CacheTestCase>,
}

impl MultiCacheTestCase {
    /// 创建一个新的批量测试用例
    pub fn new(test_cases: Vec<CacheTestCase>) -> Self {
        Self { test_cases }
    }
    
    /// 运行所有测试用例
    /// 
    /// 工作流程:
    /// 1. 遍历所有测试用例
    /// 2. 为每个测试设置编号前缀 (test-1, test-2, ...)
    /// 3. 运行测试
    /// 4. 失败时保留前缀（便于定位）
    /// 5. 成功时重置前缀
    pub fn run_all(&self, harness: &mut TestCaseHarness) -> Result<(), TesterError> {
        for (i, test_case) in self.test_cases.iter().enumerate() {
            // 设置日志前缀（test-1, test-2, ...）
            let prefix = format!("test-{}", i + 1);
            harness.logger.update_last_secondary_prefix(&prefix);
            
            // 添加运行提示
            harness.logger.infof(&format!("Running test case: {}", i + 1), &[]);
            
            // 运行测试用例
            let result = test_case.run(harness);
            
            // 如果失败，保留前缀便于定位
            if result.is_err() {
                return result;
            }
            
            // 成功，重置前缀
            harness.logger.reset_secondary_prefixes();
        }
        
        Ok(())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_test_case_creation() {
        let test_case = CacheTestCase::new(
            "Test basic operations",
            vec!["INIT 10", "PUT a 1"],
            vec!["OK", "OK"],
        );
        
        assert_eq!(test_case.description, "Test basic operations");
        assert_eq!(test_case.commands.len(), 2);
        assert_eq!(test_case.expected_responses.len(), 2);
        assert_eq!(test_case.hint, None);
        assert_eq!(test_case.verbose, false);
    }
    
    #[test]
    fn test_cache_test_case_with_hint() {
        let test_case = CacheTestCase::new(
            "Test",
            vec!["INIT 10"],
            vec!["OK"],
        ).with_hint("This is a hint");
        
        assert_eq!(test_case.hint, Some("This is a hint"));
    }
    
    #[test]
    fn test_cache_test_case_builder() {
        let test_case = CacheTestCaseBuilder::new("Test")
            .commands(vec!["INIT 10"])
            .expect(vec!["OK"])
            .hint("hint")
            .verbose()
            .build();
        
        assert_eq!(test_case.description, "Test");
        assert_eq!(test_case.verbose, true);
    }
    
    #[test]
    fn test_multi_cache_test_case_creation() {
        let multi_test = MultiCacheTestCase::new(vec![
            CacheTestCase::new("Test 1", vec!["INIT 5"], vec!["OK"]),
            CacheTestCase::new("Test 2", vec!["PUT a 1"], vec!["OK"]),
        ]);
        
        assert_eq!(multi_test.test_cases.len(), 2);
    }
}
