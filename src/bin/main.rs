use std::collections::HashMap;
use std::env;
use std::process;
use tester_utils::{run_cli, TestCase, TesterDefinition};

fn main() {
    // 获取环境变量
    let env_vars: HashMap<String, String> = env::vars().collect();
    
    // 创建测试定义
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    
    // 添加 Stage 1 测试用例
    definition.add_test_case(TestCase::new(
        "jq3".to_string(),
        lru_cache_tester::stage_1::test_basic_cache,
    ));
    
    definition.add_test_case(TestCase::new(
        "jq3-multiple-keys".to_string(),
        lru_cache_tester::stage_1::test_multiple_keys,
    ));
    
    definition.add_test_case(TestCase::new(
        "jq3-update".to_string(),
        lru_cache_tester::stage_1::test_key_update,
    ));
    
    // 运行 CLI
    let exit_code = run_cli(env_vars, definition);
    process::exit(exit_code);
}
