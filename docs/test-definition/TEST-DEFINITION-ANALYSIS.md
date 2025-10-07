# Test Case Definition 方式深度分析

## 📋 背景

对比 CodeCrafters (Go) 和 SystemQuest (Rust) 的测试定义方式，评估哪种更合理。

---

## 🔍 当前实现对比

### CodeCrafters (Go) - 声明式静态定义

```go
// internal/tester_definition.go
var testerDefinition = tester_definition.TesterDefinition{
    ExecutableFileName: "your_program.sh",
    TestCases: []tester_definition.TestCase{
        {
            Slug:     "jm1",
            TestFunc: testBindToPort,
            Timeout:  15 * time.Second,
        },
        {
            Slug:     "rg2",
            TestFunc: testPingPongOnce,
        },
        // ... 89个测试
    },
    AntiCheatTestCases: []tester_definition.TestCase{
        {
            Slug:     "anti-cheat-1",
            TestFunc: antiCheatTest,
        },
    },
}
```

**特点：**
- ✅ 集中管理：所有测试在一个地方定义
- ✅ 元数据丰富：Timeout、AntiCheat 等直接可见
- ✅ 类型安全：Go 编译期检查
- ✅ 易于查看：一眼看清测试结构
- ⚠️ 修改成本：添加测试需要修改定义结构

---

### SystemQuest (Rust) - 命令式动态注册

```rust
// src/bin/main.rs (当前实现)
fn main() {
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    
    // Stage 1 测试
    definition.add_test_case(TestCase::new(
        "jq3".to_string(),
        lru_cache_tester::stage_1::test_basic_cache,
    ));
    
    definition.add_test_case(TestCase::new(
        "jq3-multiple-keys".to_string(),
        lru_cache_tester::stage_1::test_multiple_keys,
    ));
    
    // Stage 2 测试
    definition.add_test_case(TestCase::new(
        "ze6".to_string(),
        lru_cache_tester::stage_2::test_fifo_eviction,
    ));
    
    // ... 手动添加每个测试
    
    let exit_code = run_cli(env::vars().collect(), definition);
    process::exit(exit_code);
}
```

**特点：**
- ✅ 灵活性高：运行时动态添加
- ✅ 模块化：测试函数分散在各 stage
- ⚠️ 分散管理：难以一眼看清全局
- ⚠️ 易遗漏：容易忘记注册测试
- ⚠️ 缺少元数据：Timeout 需要显式设置

---

## 🎯 tester-utils-rs 的设计支持

### 当前 API 支持

```rust
// src/tester_definition.rs
pub struct TestCase {
    pub slug: String,
    pub test_func: TestFunc,
    pub timeout: Duration,  // ✅ 已支持
}

impl TestCase {
    // 基础构造
    pub fn new<F>(slug: String, test_func: F) -> Self
    
    // 带超时构造
    pub fn new_with_timeout<F>(slug: String, test_func: F, timeout: Duration) -> Self
}

pub struct TesterDefinition {
    pub executable_file_name: String,
    pub legacy_executable_file_name: Option<String>,
    pub test_cases: Vec<TestCase>,
    pub anti_cheat_test_cases: Vec<TestCase>,  // ✅ 已支持
}

impl TesterDefinition {
    pub fn add_test_case(&mut self, test_case: TestCase)
    pub fn add_anti_cheat_test_case(&mut self, test_case: TestCase)
}
```

**核心能力：**
- ✅ Timeout 支持：`new_with_timeout()`
- ✅ AntiCheat 支持：`add_anti_cheat_test_case()`
- ✅ 查询支持：`test_case_by_slug()`, `has_test_case()`
- ❌ 缺少元数据：Title、Description、Stage 等

---

## 💡 改进方案

### 方案 A：声明式宏（推荐 ⭐⭐⭐⭐⭐）

**实现：**

```rust
// src/test_registry.rs
use std::time::Duration;
use crate::tester_definition::{TestCase, TesterDefinition};

macro_rules! test_cases {
    (
        $(
            $slug:expr => $func:path $(, timeout: $timeout:expr)?
        ),+ $(,)?
    ) => {
        vec![
            $(
                {
                    let test_case = TestCase::new($slug.to_string(), $func);
                    $(
                        let test_case = TestCase::new_with_timeout(
                            $slug.to_string(), 
                            $func, 
                            $timeout
                        );
                    )?
                    test_case
                }
            ),+
        ]
    };
}

// 使用示例
pub fn register_all_tests(definition: &mut TesterDefinition) {
    // Stage 1 tests
    for test in test_cases! {
        "jq3" => stage_1::test_basic_cache,
        "jq3-multiple-keys" => stage_1::test_multiple_keys,
        "jq3-update" => stage_1::test_key_update,
    } {
        definition.add_test_case(test);
    }
    
    // Stage 2 tests
    for test in test_cases! {
        "ze6" => stage_2::test_fifo_eviction, timeout: Duration::from_secs(15),
        "ze6-update" => stage_2::test_fifo_update_no_reorder,
        "ze6-size" => stage_2::test_fifo_size,
    } {
        definition.add_test_case(test);
    }
    
    // Stage 3 tests
    for test in test_cases! {
        "ch7" => stage_3::test_lru_eviction,
        "ch7-vs-fifo" => stage_3::test_lru_vs_fifo,
        "ch7-multiple" => stage_3::test_lru_multiple_access,
        "ch7-sequential" => stage_3::test_lru_sequential_evictions,
    } {
        definition.add_test_case(test);
    }
}

// main.rs
fn main() {
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    register_all_tests(&mut definition);
    let exit_code = run_cli(env::vars().collect(), definition);
    process::exit(exit_code);
}
```

**优势：**
- ✅ 集中视图：所有测试清晰可见
- ✅ 编译期检查：函数名错误会编译失败
- ✅ 类型安全：Timeout 类型检查
- ✅ 可选参数：Timeout 可选，默认使用 10s
- ✅ 分组管理：按 Stage 分组
- ✅ 易于维护：添加测试只需一行

---

### 方案 B：Builder 模式增强

**实现：**

```rust
// src/tester_definition.rs
impl TestCase {
    pub fn builder(slug: impl Into<String>) -> TestCaseBuilder {
        TestCaseBuilder::new(slug)
    }
}

pub struct TestCaseBuilder {
    slug: String,
    test_func: Option<TestFunc>,
    timeout: Option<Duration>,
    title: Option<String>,
    description: Option<String>,
}

impl TestCaseBuilder {
    pub fn new(slug: impl Into<String>) -> Self {
        Self {
            slug: slug.into(),
            test_func: None,
            timeout: None,
            title: None,
            description: None,
        }
    }
    
    pub fn test_func<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut TestCaseHarness) -> Result<(), TesterError> + Send + Sync + 'static,
    {
        self.test_func = Some(Box::new(func));
        self
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    pub fn build(self) -> TestCase {
        TestCase {
            slug: self.slug,
            test_func: self.test_func.expect("test_func is required"),
            timeout: self.timeout.unwrap_or(Duration::from_secs(10)),
            title: self.title,
            description: self.description,
        }
    }
}

// 使用示例
fn main() {
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    
    definition.add_test_case(
        TestCase::builder("ch7")
            .test_func(stage_3::test_lru_eviction)
            .timeout(Duration::from_secs(15))
            .title("LRU Eviction")
            .description("Test basic LRU eviction behavior")
            .build()
    );
    
    // 简化版本（使用默认值）
    definition.add_test_case(
        TestCase::builder("ch7-vs-fifo")
            .test_func(stage_3::test_lru_vs_fifo)
            .build()
    );
}
```

**优势：**
- ✅ 元数据丰富：Title、Description 等
- ✅ 可选参数：链式调用，清晰易读
- ✅ 扩展性强：易于添加新字段
- ⚠️ 代码冗长：每个测试需要多行

---

### 方案 C：混合方案（最佳 ⭐⭐⭐⭐⭐）

**结合声明式宏 + Builder 增强**

```rust
// src/test_registry.rs
macro_rules! register_tests {
    (
        $(
            stage $stage:expr, $stage_name:expr => {
                $(
                    $slug:expr => $func:path 
                    $(, timeout: $timeout:expr)? 
                    $(, title: $title:expr)?
                ),+ $(,)?
            }
        ),+ $(,)?
    ) => {
        pub fn register_all_tests(definition: &mut TesterDefinition) {
            $(
                // Stage 注释
                $(
                    let mut test = TestCase::new($slug.to_string(), $func);
                    $(
                        test = TestCase::new_with_timeout($slug.to_string(), $func, $timeout);
                    )?
                    definition.add_test_case(test);
                )+
            )+
        }
        
        pub fn get_stage_info() -> Vec<StageInfo> {
            vec![
                $(
                    StageInfo {
                        stage: $stage,
                        name: $stage_name.to_string(),
                        test_count: count!($($slug),+),
                    }
                ),+
            ]
        }
    };
}

// 使用示例
register_tests! {
    stage 1, "Basic Cache" => {
        "jq3" => stage_1::test_basic_cache, title: "Basic operations",
        "jq3-multiple-keys" => stage_1::test_multiple_keys,
        "jq3-update" => stage_1::test_key_update,
    },
    
    stage 2, "FIFO Eviction" => {
        "ze6" => stage_2::test_fifo_eviction, timeout: Duration::from_secs(15),
        "ze6-update" => stage_2::test_fifo_update_no_reorder,
        "ze6-size" => stage_2::test_fifo_size,
    },
    
    stage 3, "LRU Eviction" => {
        "ch7" => stage_3::test_lru_eviction,
        "ch7-vs-fifo" => stage_3::test_lru_vs_fifo, title: "LRU vs FIFO comparison",
        "ch7-multiple" => stage_3::test_lru_multiple_access,
        "ch7-sequential" => stage_3::test_lru_sequential_evictions,
    },
}

// main.rs
fn main() {
    let mut definition = TesterDefinition::new("your_program.sh".to_string());
    test_registry::register_all_tests(&mut definition);
    let exit_code = run_cli(env::vars().collect(), definition);
    process::exit(exit_code);
}
```

**优势：**
- ✅ 集中管理：所有测试结构一目了然
- ✅ 分组清晰：按 Stage 自动分组
- ✅ 编译期检查：函数名、类型错误会编译失败
- ✅ 元数据支持：Timeout、Title 可选
- ✅ 代码生成：自动生成 `get_stage_info()` 等辅助函数
- ✅ 类似 CodeCrafters：保持声明式风格

---

## 📊 对比总结

| 特性 | CodeCrafters (Go) | SystemQuest 当前 | 方案 A 宏 | 方案 B Builder | 方案 C 混合 |
|------|------------------|-----------------|---------|--------------|-----------|
| 集中管理 | ✅ | ❌ | ✅ | ❌ | ✅ |
| 类型安全 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Timeout 支持 | ✅ | ⚠️ 需显式 | ✅ | ✅ | ✅ |
| 元数据丰富 | ⚠️ 有限 | ❌ | ❌ | ✅ | ⚠️ 可选 |
| 易于维护 | ✅ | ⚠️ | ✅ | ⚠️ | ✅ |
| Stage 分组 | ❌ | ⚠️ 手动 | ⚠️ 手动 | ❌ | ✅ |
| 代码简洁 | ✅ | ⚠️ | ✅ | ❌ | ✅ |
| 扩展性 | ⚠️ | ✅ | ⚠️ | ✅ | ✅ |

---

## 🎯 最终建议

### 立即实施：方案 C（混合方案）

**理由：**
1. ✅ **最接近 CodeCrafters 风格**：保持声明式、集中管理
2. ✅ **Rust 最佳实践**：利用宏的编译期能力
3. ✅ **易于维护**：添加测试只需一行，难以遗漏
4. ✅ **自动分组**：Stage 信息自动生成
5. ✅ **元数据可选**：Timeout、Title 可按需添加

### 实施步骤

1. **Week 1**: 创建 `test_registry.rs` 和基础宏
2. **Week 2**: 迁移现有测试到宏定义
3. **Week 3**: 增强元数据支持（Title、Description）
4. **Week 4**: 文档和示例完善

---

## 💭 关键洞察

### CodeCrafters 的优势在哪里？

1. **声明式优于命令式**：一眼看清测试结构
2. **集中管理优于分散**：减少心智负担
3. **元数据内联**：Timeout 等配置直接可见

### SystemQuest 如何借鉴？

1. **采用声明式宏**：保持 Rust 习惯，获得 Go 的便利
2. **Stage 自动分组**：利用宏生成辅助函数
3. **可选元数据**：灵活性和清晰性平衡

### 为什么不直接用 Builder？

- Builder 模式适合复杂对象构建
- 但测试定义本质是简单声明，不需要复杂流程
- 声明式宏更接近 CodeCrafters 的"一行一测试"风格

---

## ✅ 结论

**CodeCrafters 的方式更合理**，SystemQuest 应该借鉴其声明式、集中管理的优势。

**最佳实践：采用方案 C（混合宏方案）**
- 保持 Rust 的类型安全和编译期检查
- 获得 Go 声明式定义的清晰性
- 支持可选元数据和扩展
- 易于维护和避免遗漏

这样既保持了 Rust 生态的习惯，又吸收了 CodeCrafters 经过验证的设计智慧！
