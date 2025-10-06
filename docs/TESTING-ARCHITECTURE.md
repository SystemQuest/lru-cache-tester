# CodeCrafters/SystemQuest 测试架构总结

## 关键发现

### 1. 三个独立的仓库/目录

#### A. **课程仓库** (`build-your-own-lru-cache/`)
- `course-definition.yml` - 课程定义
- `solutions/` - **仅包含前几个 stage 的参考解决方案**（用于文档和示例）
- `starter_templates/` - 起始模板
- `compiled_starters/` - 编译后的起始代码（学员下载）

#### B. **Tester 仓库** (`lru-cache-tester/`)
```
lru-cache-tester/
├── Cargo.toml
├── Makefile
├── test.sh                    # 简单的包装器: exec "${TESTER_DIR}/tester"
├── src/
│   ├── bin/main.rs
│   ├── lib.rs
│   ├── helpers.rs
│   └── stage_1.rs
├── dist/
│   └── tester                 # 编译后的二进制文件
└── internal/
    └── test_helpers/
        └── pass_stage1/       # ⭐ 测试用例
            ├── systemquest.yml
            └── your_program.sh
```

#### C. **学员仓库**（独立项目，不在课程仓库中）
- 学员从平台获取 `compiled_starters/` 的代码
- 在**自己的独立仓库**中开发
- 使用 pipenv/venv 管理依赖
- 提交代码到平台测试

### 2. 测试执行流程

#### 开发阶段（本地测试 tester）

```bash
cd lru-cache-tester

# 方式1：使用 Makefile（推荐）
make build
make test_stage1

# 方式2：直接运行
SYSTEMQUEST_REPOSITORY_DIR=./internal/test_helpers/pass_stage1 \
SYSTEMQUEST_TEST_CASES_JSON='[{"slug":"s1-basic",...}]' \
./dist/tester
```

**关键点**：
- `SYSTEMQUEST_REPOSITORY_DIR` 指向 **tester 内部的 test_helpers**
- test_helpers 中的 `your_program.sh` 可以是：
  - 真实的参考实现（如 redis-server）
  - 指向 solutions 目录的包装脚本
  - Mock 实现

#### 生产阶段（平台测试学员代码）

```bash
# 平台设置环境变量
export TESTER_DIR=/path/to/tester/dist
export SYSTEMQUEST_REPOSITORY_DIR=/path/to/student/repo
export SYSTEMQUEST_TEST_CASES_JSON='[...]'

# 执行测试
/path/to/tester/test.sh
```

### 3. `your_program.sh` 的对齐问题

#### CodeCrafters 版本（正确）:
```bash
#!/bin/sh
set -e
exec pipenv run python3 -u -m app.main "$@"
```

#### 我们之前的版本（错误）:
```bash
exec python3 -m app.main "$@"  # ❌ 缺少 pipenv 和 -u 标志
```

**重要标志**：
- `pipenv run` - 使用项目的虚拟环境
- `-u` - Unbuffered mode（立即刷新输出，对交互式测试很关键）
- `-m app.main` - 以模块方式运行

### 4. 环境变量

| 变量名 | 来源 | 说明 |
|--------|------|------|
| `SYSTEMQUEST_REPOSITORY_DIR` | 测试命令 | 学员代码目录 |
| `SYSTEMQUEST_TEST_CASES_JSON` | 测试命令 | 要运行的测试用例 JSON |
| `SYSTEMQUEST_DEBUG` | ❌ 不存在 | debug 来自 `systemquest.yml` |
| `TESTER_DIR` | 平台/CI | tester 二进制所在目录（用于 test.sh） |

### 5. Debug 模式

**错误理解**：Debug 通过环境变量 `SYSTEMQUEST_DEBUG` 设置
**正确理解**：Debug 在学员仓库的 `systemquest.yml` 中设置

```yaml
# systemquest.yml（学员仓库中）
debug: true  # 启用详细日志
```

### 6. Test Helpers 的作用

`internal/test_helpers/pass_stage1/` 不是学员的代码，而是：

1. **验证 tester 本身是否正确**
   - 测试 tester 的测试逻辑
   - 确保测试用例设计合理

2. **作为 tester 开发的测试目标**
   - 快速迭代 tester 功能
   - 不需要真实的学员代码

3. **示例实现**
   - 可以指向真实的参考实现
   - 可以是 mock 实现

### 7. Makefile 的标准结构

```makefile
.PHONY: build test test_stage1 test_starter clean

# 构建 tester
build:
	cargo build --release
	mkdir -p dist
	cp target/release/lru-cache-tester dist/tester

# 运行单元测试
test:
	cargo test

# 测试 Stage 1（应该通过）
test_stage1: build
	SYSTEMQUEST_REPOSITORY_DIR=$(shell pwd)/internal/test_helpers/pass_stage1 \
	SYSTEMQUEST_TEST_CASES_JSON='[...]' \
	./dist/tester

# 测试 compiled starter（应该失败）
test_starter: build
	SYSTEMQUEST_REPOSITORY_DIR=../build-your-own-lru-cache/compiled_starters/python \
	SYSTEMQUEST_TEST_CASES_JSON='[...]' \
	./dist/tester || true

# 清理
clean:
	cargo clean
	rm -rf dist
```

### 8. test.sh 的作用

```bash
#!/bin/sh
exec "${TESTER_DIR}/tester"
```

**为什么这么简单？**
- 平台负责设置所有环境变量
- test.sh 只是一个标准化的入口点
- 真正的逻辑在 tester 二进制中

**不要在 test.sh 中做**：
- ❌ 设置 `SYSTEMQUEST_REPOSITORY_DIR`
- ❌ 解析测试用例
- ❌ 任何业务逻辑

### 9. 常见错误

#### 错误 1：在 tester 目录运行 pipenv
```bash
# ❌ 错误
cd lru-cache-tester
SYSTEMQUEST_REPOSITORY_DIR=../solution/code ./dist/tester

# 问题：pipenv 在 lru-cache-tester 目录创建虚拟环境
# 结果：ModuleNotFoundError: No module named 'app'
```

**解决方案**：使用 test_helpers 作为独立的"学员仓库"

#### 错误 2：期望通过 SYSTEMQUEST_DEBUG 启用调试
```bash
# ❌ 不工作
SYSTEMQUEST_DEBUG=true ./dist/tester
```

**正确方式**：在 systemquest.yml 中设置 `debug: true`

#### 错误 3：在 course 仓库中维护完整的 solutions
```
❌ solutions/python/01-s1/
❌ solutions/python/02-s2/
❌ solutions/python/03-s3/
❌ solutions/python/04-s4/
❌ solutions/python/05-s5/
```

**正确方式**：只维护前 1-2 个 stage 的 solution 作为示例

## 最佳实践

### Tester 开发流程

1. **创建 test_helpers**
   ```bash
   mkdir -p internal/test_helpers/pass_stage1
   ```

2. **实现 your_program.sh**（指向真实实现或 mock）

3. **配置 systemquest.yml**
   ```yaml
   current_stage: 1
   debug: false  # 生产环境关闭
   ```

4. **在 Makefile 中添加测试目标**

5. **运行测试**
   ```bash
   make test_stage1
   ```

### 学员使用流程（模拟）

1. **获取起始代码**（从 compiled_starters）

2. **设置环境**
   ```bash
   pipenv install
   ```

3. **开发代码**

4. **本地测试**（如果有 tester）
   ```bash
   SYSTEMQUEST_REPOSITORY_DIR=$(pwd) \
   SYSTEMQUEST_TEST_CASES_JSON='[...]' \
   /path/to/tester
   ```

5. **提交到平台**（平台运行 tester）

## 9. Test Helpers 的三种类型 🆕

### 类型 A：真实系统二进制

**示例**: shell-tester, sqlite-tester

```bash
# shell-tester/internal/test_helpers/bash/your_shell.sh
#!/bin/sh
exec bash --norc -i

# sqlite-tester/internal/test_helpers/pass_all/your_sqlite3.sh
#!/bin/sh
exec sqlite3 "$@"
```

**特点**:
- 直接调用系统已安装的命令（bash, sqlite3）
- 无需编写任何实现代码
- 测试 tester 对标准输出格式的解析能力

**优势**:
- 无需维护参考实现
- 自动支持所有功能
- 跨平台兼容（bash, zsh, ash）
- 可测试复杂交互（PTY, 信号, 转义序列）

**适用场景**: 当存在成熟的系统命令时

### 类型 B：Python 部分实现

**示例**: sqlite-tester/internal/test_helpers/stages/

```
stages/
├── init/           # 只实现 Stage 1（.dbinfo）
├── table_count/    # 实现到 Stage 2
└── table_names/    # 实现到 Stage 3
```

```python
# stages/init/app.py（只实现 Stage 1）
if command == ".dbinfo":
    # 只实现了最基本的功能
    print(f"database page size: {page_size}")
else:
    print(f"Invalid command: {command}")
```

**特点**:
- 每个 stage 目录是一个"断点实现"
- 用于测试**渐进式实现的判断逻辑**
- 验证 tester 能正确识别未完成的实现

**测试策略**:
```go
// 测试 Stage 1 应该通过
"init_success": {
    UntilStageSlug: "dr6",
    CodePath:       "./test_helpers/stages/init",
    ExpectedExitCode: 0,  // ✅ Pass
}

// 用同一个实现测试 Stage 2 应该失败
"table_count_failure": {
    UntilStageSlug: "ce0",  // Stage 2
    CodePath:       "./test_helpers/stages/init",  // 只实现了 Stage 1
    ExpectedExitCode: 1,  // ❌ Fail
}
```

**适用场景**: 需要测试多个 stage 的渐进式实现

### 类型 C：外部完整实现

**示例**: interpreter-tester

```bash
# stages_test.go
CodePath: "../../craftinginterpreters/build/gen/chap04_scanning"
CodePath: "../../craftinginterpreters/build/gen/chap13_inheritance"
```

**特点**:
- 指向**仓库外部**的实现（来自其他项目）
- jlox 是 Java 编译的二进制文件
- 通过包装脚本调用外部程序

```bash
# test_helpers/jlox04/your_program.sh
#!/bin/bash
script_dir=$(dirname "$0")
${script_dir}/jlox "$filename"  # 调用预编译的 jlox
```

**适用场景**: 
- 已有权威的参考实现
- 实现复杂度很高（如完整的解释器）
- 需要跟随外部项目更新

### 对比总结

| 类型 | 维护成本 | 完整度 | 适用场景 | 示例 |
|------|---------|--------|---------|------|
| 真实二进制 | ✅ 零成本 | 100% | 系统命令存在 | bash, sqlite3 |
| 部分实现 | 🟡 中等 | 分阶段 | 渐进式测试 | SQLite stages |
| 外部实现 | 🟡 依赖外部 | 100% | 权威参考 | jlox |

## 10. 推荐的目录结构 🆕

### 完整的 test_helpers 结构

```
internal/test_helpers/
├── course_definition.yml       # 课程定义（用于测试）
│
├── pass_all/                   # ✅ 完整实现（所有 stage 通过）
│   ├── systemquest.yml         #    必须：配置文件
│   └── your_program.sh         #    必须：可执行入口
│
├── stages/                     # 🔄 渐进式实现（可选，推荐）
│   ├── stage1/                 #    Stage 1 only
│   │   ├── systemquest.yml
│   │   └── your_program.sh
│   ├── stage2/                 #    Stage 1-2
│   └── stage3/                 #    Stage 1-3
│
├── scenarios/                  # ❌ 特殊测试场景（可选）
│   ├── empty_key/              #    测试空 key 的错误处理
│   ├── invalid_capacity/       #    测试无效 capacity
│   └── segfault/               #    测试崩溃场景
│
└── fixtures/                   # 📊 期望输出（必须）
    ├── stage1/
    │   ├── pass                #    Stage 1 通过的输出
    │   └── fail                #    Stage 1 失败的输出
    └── scenarios/
        └── empty_key           #    特殊场景的期望输出
```

### 我们当前的结构

```
lru-cache-tester/internal/test_helpers/
└── pass_stage1/                # ⚠️ 命名不够清晰
    ├── systemquest.yml
    └── your_program.sh
```

### 推荐改进（Week 2）

```
lru-cache-tester/internal/test_helpers/
├── pass_all/                   # 重命名，表示通过所有 stage
│   ├── systemquest.yml
│   └── your_program.sh
│
├── stages/                     # 新增：渐进式测试
│   ├── stage1/                 # 只实现 INIT, PUT, GET
│   ├── stage2/                 # 添加 LRU eviction
│   └── stage3/                 # 添加 TTL
│
├── scenarios/                  # 新增：边缘情况
│   ├── empty_key/
│   └── negative_capacity/
│
└── fixtures/                   # 新增：期望输出
    ├── stage1/
    │   ├── pass
    │   └── fail
    └── scenarios/
```

## 11. 渐进式测试的价值 🆕

### 为什么需要 stages/ 目录？

#### 问题：如何验证 tester 能正确判断未完成的实现？

**场景 1**: 学员只实现了 Stage 1，提交 Stage 2 测试
- 期望: Tester 应该标记为 ❌ 失败
- 风险: 如果 tester 有 bug，可能误判为 ✅ 通过

**场景 2**: 学员复制了别人的完整实现
- 期望: 平台可以检测作弊行为
- 需要: Anti-cheat 测试用例

### stages/ 目录的作用

```go
// 测试用例矩阵
{
    "stage1_impl + stage1_test": ✅ Pass,
    "stage1_impl + stage2_test": ❌ Fail,  // ⭐ 关键测试
    "stage2_impl + stage2_test": ✅ Pass,
}
```

**关键价值**:
1. ✅ 验证 tester 能识别未完成的功能
2. ✅ 测试错误消息是否清晰
3. ✅ 确保不会漏判或误判

### 实现示例（Week 2）

```rust
// lru-cache-tester/tests/stages_test.rs
#[test]
fn test_stage1_with_stage1_impl() {
    let result = run_tester_test(
        "./internal/test_helpers/stages/stage1",
        vec!["s1-basic"],
    );
    assert_eq!(result.exit_code, 0);  // ✅ 应该通过
}

#[test]
fn test_stage2_with_stage1_impl() {
    let result = run_tester_test(
        "./internal/test_helpers/stages/stage1",  // ⚠️ 只实现了 Stage 1
        vec!["s2-eviction"],                       // 测试 Stage 2
    );
    assert_eq!(result.exit_code, 1);  // ❌ 应该失败
    assert!(result.stderr.contains("eviction not implemented"));
}
```

## 12. 环境变量完整列表 🆕

| 变量名 | 必需 | 说明 | 示例 |
|--------|------|------|------|
| `SYSTEMQUEST_REPOSITORY_DIR` | ✅ | 学员代码目录 | `./internal/test_helpers/pass_all` |
| `SYSTEMQUEST_TEST_CASES_JSON` | ✅ | 测试用例 JSON | `'[{"slug":"s1-basic",...}]'` |
| `SYSTEMQUEST_RANDOM_SEED` | 🟡 | 随机种子（可重现测试） | `"1234567890"` |
| `SYSTEMQUEST_SUBMISSION_DIR` | ❌ | 提交目录（可能不同于 REPOSITORY_DIR） | `/tmp/submission` |
| `TESTER_DIR` | 🟡 | tester 二进制目录（用于 test.sh） | `/path/to/tester/dist` |

**新发现**:
- `SYSTEMQUEST_RANDOM_SEED` 在所有 CodeCrafters testers 中设置
- 确保测试结果可重现（特别是涉及随机数据的测试）

```go
// stages_test.go
func TestStages(t *testing.T) {
    os.Setenv("CODECRAFTERS_RANDOM_SEED", "1234567890")  // ⭐ 固定随机种子
    // ...
}
```

## 总结

关键认知转变：
- ✅ Solutions 不是完整的实现，只是示例
- ✅ Test helpers 才是 tester 开发的测试目标
- ✅ 学员代码在**独立的仓库**中
- ✅ Debug 模式通过 YAML 配置，不是环境变量
- ✅ Pipenv 环境在学员仓库中，不是 tester 仓库中
- 🆕 Test helpers 有三种类型：真实二进制、部分实现、外部实现
- 🆕 stages/ 目录用于测试渐进式实现的判断逻辑
- 🆕 fixtures/ 目录存储期望输出用于断言
- 🆕 RANDOM_SEED 环境变量确保测试可重现
