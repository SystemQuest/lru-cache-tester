/// Integration tests for stage validation
/// 
/// These tests verify that the tester correctly identifies
/// incomplete implementations (progressive testing)

use std::process::Command;
use std::path::PathBuf;

fn get_tester_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist/tester")
}

fn get_test_helpers_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("internal/test_helpers")
}

#[test]
fn test_stage1_with_pass_all() {
    let tester = get_tester_path();
    let repo_dir = get_test_helpers_dir().join("pass_all");
    
    let output = Command::new(&tester)
        .arg("jq3")
        .arg("jq3-multiple-keys")
        .arg("jq3-update")
        .env("SYSTEMQUEST_REPOSITORY_DIR", repo_dir)
        .output()
        .expect("Failed to run tester");
    
    assert_eq!(
        output.status.code(),
        Some(0),
        "Stage 1 tests with pass_all should pass.\nStdout: {}\nStderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_stage1_with_stage1_impl() {
    let tester = get_tester_path();
    let repo_dir = get_test_helpers_dir().join("stages/stage1");
    
    let output = Command::new(&tester)
        .arg("jq3")
        .arg("jq3-multiple-keys")
        .arg("jq3-update")
        .env("SYSTEMQUEST_REPOSITORY_DIR", repo_dir)
        .output()
        .expect("Failed to run tester");
    
    assert_eq!(
        output.status.code(),
        Some(0),
        "Stage 1 tests with stage1 impl should pass.\nStdout: {}\nStderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// TODO: Enable when Stage 2 is implemented
// #[test]
// fn test_stage2_with_stage1_impl_should_fail() {
//     let tester = get_tester_path();
//     let repo_dir = get_test_helpers_dir().join("stages/stage1");
//     
//     let output = Command::new(&tester)
//         .env("SYSTEMQUEST_REPOSITORY_DIR", repo_dir)
//         .env("SYSTEMQUEST_TEST_CASES_JSON", r#"[{"slug":"s2-fifo"}]"#)
//         .output()
//         .expect("Failed to run tester");
//     
//     assert_ne!(
//         output.status.code(),
//         Some(0),
//         "Stage 2 test with stage1 impl should fail (progressive testing)"
//     );
// }
