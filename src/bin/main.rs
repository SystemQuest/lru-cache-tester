use std::collections::HashMap;
use std::env;
use std::process;
use tester_utils::{register_tests, run_cli, TesterDefinition};

// 声明式测试注册 - 所有测试在一处集中定义
register_tests! {
    stage 0, "Edge Cases & Error Handling" => {
        "edge-capacity-1" => lru_cache_tester::stage_0::test_capacity_one,
        "edge-empty-values" => lru_cache_tester::stage_0::test_empty_values,
        "error-no-init" => lru_cache_tester::stage_0::test_no_init,
        "error-double-init" => lru_cache_tester::stage_0::test_double_init,
    },
    
    stage 1, "Basic Cache Operations" => {
        "jq3" => lru_cache_tester::stage_1::test_basic_cache,
        "jq3-multiple-keys" => lru_cache_tester::stage_1::test_multiple_keys,
        "jq3-update" => lru_cache_tester::stage_1::test_key_update,
    },
    
    stage 2, "FIFO Eviction" => {
        "ze6" => lru_cache_tester::stage_2::test_fifo_eviction,
        "ze6-update" => lru_cache_tester::stage_2::test_fifo_update_no_reorder,
        "ze6-size" => lru_cache_tester::stage_2::test_fifo_size,
    },
    
    stage 3, "LRU Eviction" => {
        "ch7" => lru_cache_tester::stage_3::test_lru_eviction,
        "ch7-vs-fifo" => lru_cache_tester::stage_3::test_lru_vs_fifo,
        "ch7-multiple" => lru_cache_tester::stage_3::test_lru_multiple_access,
        "ch7-sequential" => lru_cache_tester::stage_3::test_lru_sequential_evictions,
    },
    
    stage 4, "Custom Doubly Linked List" => {
        "vh5" => lru_cache_tester::stage_4::test_lru_eviction,
        "vh5-vs-fifo" => lru_cache_tester::stage_4::test_lru_vs_fifo,
        "vh5-multiple" => lru_cache_tester::stage_4::test_lru_multiple_access,
        "vh5-sequential" => lru_cache_tester::stage_4::test_lru_sequential_evictions,
        "vh5-capacity-one" => lru_cache_tester::stage_4::test_capacity_one,
        "vh5-empty-cache" => lru_cache_tester::stage_4::test_empty_cache,
        "vh5-repeated-ops" => lru_cache_tester::stage_4::test_repeated_operations,
        "vh5-eviction-cycle" => lru_cache_tester::stage_4::test_full_eviction_cycle,
    },
    
    stage 5, "Thread Safety" => {
        "ba6" => lru_cache_tester::stage_5::test_thread_safe_basic,
        "ba6-read-heavy" => lru_cache_tester::stage_5::test_concurrent_read_heavy,
        "ba6-write-heavy" => lru_cache_tester::stage_5::test_concurrent_write_heavy,
        "ba6-stress" => lru_cache_tester::stage_5::test_concurrent_stress,
        "ba6-sequential" => lru_cache_tester::stage_5::test_concurrent_sequential,
        "ba6-lru-preserved" => lru_cache_tester::stage_5::test_concurrent_lru_preserved,
        "ba6-size-consistency" => lru_cache_tester::stage_5::test_size_consistency,
        "ba6-capacity-one" => lru_cache_tester::stage_5::test_concurrent_capacity_one,
        "ba6-after-concurrent" => lru_cache_tester::stage_5::test_operations_after_concurrent,
    },
}

fn main() {
    // 获取环境变量
    let env_vars: HashMap<String, String> = env::vars().collect();
    
    // 创建测试定义
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    
    // 使用生成的函数注册所有测试
    register_all_tests(&mut definition);
    
    // 运行 CLI
    let exit_code = run_cli(env_vars, definition);
    process::exit(exit_code);
}
