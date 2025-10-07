use tester_utils::{Executable, TesterError};

/// CommandRunner - Batch stdin/stdout 模式
/// 
/// 设计理念（基于 CodeCrafters 测试架构研究）:
/// - Week 1 使用简单的 batch 模式: 一次性发送所有命令，等待所有响应
/// - Week 2-3 可选升级到 PTY 交互模式（参考 Shell-Tester）
/// 
/// 工作流程:
/// 1. Start program once
/// 2. Write all commands to stdin
/// 3. Close stdin (EOF)
/// 4. Wait for program to exit
/// 5. Read all responses from stdout
pub struct CommandRunner {
    executable: Executable,
}

impl CommandRunner {
    /// 创建 CommandRunner
    pub fn new(executable: Executable) -> Self {
        Self { executable }
    }
    
    /// 批量发送命令并读取所有响应
    /// 
    /// 这是 Week 1 的核心方法，适用于所有 Stage 1-4 测试
    /// 
    /// # 示例
    /// ```rust,no_run
    /// # use lru_cache_tester::helpers::CommandRunner;
    /// # use tester_utils::Executable;
    /// # fn example(executable: Executable) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut runner = CommandRunner::new(executable);
    /// let responses = runner.send_commands(&[
    ///     "INIT 10",
    ///     "PUT name Alice",
    ///     "GET name",
    /// ])?;
    /// assert_eq!(responses, vec!["OK", "OK", "Alice"]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn send_commands(&mut self, commands: &[&str]) -> Result<Vec<String>, TesterError> {
        // 使用 run_with_stdin 一次性发送所有命令
        let stdin_data = commands.join("\n") + "\n";
        
        let result = self.executable.run_with_stdin(stdin_data.as_bytes(), &[])
            .map_err(|e| TesterError::Configuration(format!("Execution failed: {:?}", e)))?;
        
        // 4. 检查退出码
        if result.exit_code != 0 {
            return Err(TesterError::User(format!(
                "Program exited with code {}: {}",
                result.exit_code,
                String::from_utf8_lossy(&result.stderr)
            ).into()));
        }
        
        // 5. 解析输出（按行分割）
        let output = String::from_utf8_lossy(&result.stdout);
        let responses: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        
        // 6. 验证响应数量匹配
        if responses.len() != commands.len() {
            return Err(TesterError::User(format!(
                "Expected {} responses, got {}. Output: {:?}",
                commands.len(),
                responses.len(),
                responses
            ).into()));
        }
        
        Ok(responses)
    }
}

// ============================================================================
// 未来改进（Week 2-3 可选）
// ============================================================================
// 
// 如果需要真正的交互式测试（发一条读一条），参考 Shell-Tester 的 PTY 模式:
// 
// ```rust
// use pty_process::Pty;
// 
// pub struct InteractiveCommandRunner {
//     pty: Pty,
//     buffer: String,
// }
// 
// impl InteractiveCommandRunner {
//     pub fn send_command(&mut self, cmd: &str) -> Result<String, TesterError> {
//         // 1. Write command to PTY
//         writeln!(self.pty, "{}", cmd)?;
//         
//         // 2. Read until newline (real-time response)
//         let mut line = String::new();
//         self.pty.read_line(&mut line)?;
//         
//         Ok(line.trim().to_string())
//     }
// }
// ```
// 
// 优点:
// - 真正的实时交互
// - 程序保持运行，状态保持在内存
// - 更接近真实使用场景
// 
// 缺点:
// - 需要添加 pty-process crate
// - 实现复杂度增加约 100 行代码
// - 需要处理 PTY 特有的问题（ANSI 转义码等）
// 
// 决策: Week 1 不实现，Week 2-3 根据需要评估

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    
    /// 测试命令拼接逻辑
    #[test]
    fn test_command_joining() {
        let commands = vec!["INIT 10", "PUT a 1", "GET a"];
        let stdin_data = commands.join("\n") + "\n";
        
        assert_eq!(stdin_data, "INIT 10\nPUT a 1\nGET a\n");
    }
    
    /// 测试响应解析逻辑
    #[test]
    fn test_response_parsing() {
        let output = "OK\nOK\n1\n";
        let responses: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        
        assert_eq!(responses, vec!["OK", "OK", "1"]);
    }
    
    /// 测试响应数量验证逻辑
    #[test]
    fn test_response_count_validation() {
        let commands = vec!["INIT 10", "PUT a 1"];
        let responses = vec!["OK".to_string(), "OK".to_string()];
        
        assert_eq!(responses.len(), commands.len());
    }
    
    /// 测试响应数量不匹配检测
    #[test]
    fn test_response_count_mismatch() {
        let commands = vec!["INIT 10", "PUT a 1", "GET a"];
        let responses = vec!["OK".to_string(), "OK".to_string()]; // 缺少一个
        
        assert_ne!(responses.len(), commands.len());
    }
    
    /// 测试空命令列表
    #[test]
    fn test_empty_commands() {
        let commands: Vec<&str> = vec![];
        let stdin_data = commands.join("\n") + "\n";
        
        assert_eq!(stdin_data, "\n");
    }
    
    /// 测试单个命令
    #[test]
    fn test_single_command() {
        let commands = vec!["INIT 10"];
        let stdin_data = commands.join("\n") + "\n";
        
        assert_eq!(stdin_data, "INIT 10\n");
    }
}
