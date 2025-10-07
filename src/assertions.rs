use tester_utils::{Logger, TesterError};

/// Assertion trait for verifying command responses
/// 
/// This trait abstracts the verification logic, allowing different
/// validation strategies to be composed and reused.
pub trait Assertion {
    /// Verify the actual responses against expected criteria
    fn verify(&self, actual: &[String], logger: &Logger) -> Result<(), TesterError>;
}

/// ExactMatchAssertion verifies that responses match exactly line-by-line
/// 
/// This is the default assertion used for most cache test cases.
/// It provides friendly output showing each line's verification status.
pub struct ExactMatchAssertion {
    expected: Vec<String>,
    command_hints: Option<Vec<String>>,
}

impl ExactMatchAssertion {
    /// Create a new ExactMatchAssertion with expected responses
    pub fn new(expected: Vec<String>) -> Self {
        Self {
            expected,
            command_hints: None,
        }
    }

    /// Add command hints for better error messages
    /// 
    /// When provided, commands are shown alongside responses:
    /// `✓ OK          (INIT 5)`
    pub fn with_commands(mut self, commands: Vec<String>) -> Self {
        self.command_hints = Some(commands);
        self
    }
}

impl Assertion for ExactMatchAssertion {
    fn verify(&self, actual: &[String], logger: &Logger) -> Result<(), TesterError> {
        let mut success_logs: Vec<String> = Vec::new();

        // Check each expected line
        for (i, expected_line) in self.expected.iter().enumerate() {
            if i >= actual.len() {
                // Log all successful lines before showing the error
                for log in &success_logs {
                    logger.successf(log, &[]);
                }
                
                let hint = self.command_hints.as_ref()
                    .and_then(|cmds| cmds.get(i))
                    .map(|cmd| format!(" ({})", cmd))
                    .unwrap_or_default();
                
                logger.errorf(&format!("? {}{}", expected_line, hint), &[]);
                return Err(TesterError::User(
                    format!(
                        "Expected response #{} to be '{}', but didn't receive enough responses (got {} response(s))",
                        i + 1,
                        expected_line,
                        actual.len()
                    ).into()
                ));
            }

            let actual_line = &actual[i];

            if actual_line != expected_line {
                // Log all successful lines before showing the error
                for log in &success_logs {
                    logger.successf(log, &[]);
                }
                
                let hint = self.command_hints.as_ref()
                    .and_then(|cmds| cmds.get(i))
                    .map(|cmd| format!(" ({})", cmd))
                    .unwrap_or_default();
                
                logger.errorf(&format!("𐄂 {}{}", actual_line, hint), &[]);
                return Err(TesterError::User(
                    format!(
                        "Response #{} mismatch: expected '{}', got '{}'",
                        i + 1,
                        expected_line,
                        actual_line
                    ).into()
                ));
            } else {
                let hint = self.command_hints.as_ref()
                    .and_then(|cmds| cmds.get(i))
                    .map(|cmd| format!(" ({})", cmd))
                    .unwrap_or_default();
                
                success_logs.push(format!("✓ {}{}", actual_line, hint));
            }
        }

        // Check for extra responses
        if actual.len() > self.expected.len() {
            // Log all successful lines before showing the error
            for log in &success_logs {
                logger.successf(log, &[]);
            }
            
            logger.errorf(&format!("! {}", actual[self.expected.len()]), &[]);
            return Err(TesterError::User(
                format!(
                    "Expected {} response(s), but got {} (extra response: '{}')",
                    self.expected.len(),
                    actual.len(),
                    actual[self.expected.len()]
                ).into()
            ));
        }

        // All lines match - show summary instead of repeating all lines
        logger.successf(&format!("✓ {} response(s) match", self.expected.len()), &[]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tester_utils::Logger;

    fn create_test_logger() -> Logger {
        Logger::get_quiet_logger("test")
    }

    #[test]
    fn test_exact_match_success() {
        let assertion = ExactMatchAssertion::new(vec!["OK".to_string(), "1".to_string()]);
        let actual = vec!["OK".to_string(), "1".to_string()];
        let logger = create_test_logger();

        let result = assertion.verify(&actual, &logger);
        assert!(result.is_ok());
    }

    #[test]
    fn test_exact_match_mismatch() {
        let assertion = ExactMatchAssertion::new(vec!["OK".to_string(), "1".to_string()]);
        let actual = vec!["OK".to_string(), "2".to_string()];
        let logger = create_test_logger();

        let result = assertion.verify(&actual, &logger);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Response #2 mismatch"));
    }

    #[test]
    fn test_exact_match_missing_response() {
        let assertion = ExactMatchAssertion::new(vec!["OK".to_string(), "1".to_string()]);
        let actual = vec!["OK".to_string()];
        let logger = create_test_logger();

        let result = assertion.verify(&actual, &logger);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("didn't receive enough responses"));
    }

    #[test]
    fn test_exact_match_extra_response() {
        let assertion = ExactMatchAssertion::new(vec!["OK".to_string()]);
        let actual = vec!["OK".to_string(), "EXTRA".to_string()];
        let logger = create_test_logger();

        let result = assertion.verify(&actual, &logger);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("extra response"));
    }

    #[test]
    fn test_exact_match_with_commands() {
        let assertion = ExactMatchAssertion::new(vec!["OK".to_string()])
            .with_commands(vec!["INIT 5".to_string()]);
        let actual = vec!["OK".to_string()];
        let logger = create_test_logger();

        let result = assertion.verify(&actual, &logger);
        assert!(result.is_ok());
    }
}
