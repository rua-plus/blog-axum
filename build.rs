use std::process::Command;

fn main() {
    // 收集 git commit hash 信息
    let commit_hash = match Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::from("unknown"),
    };

    // 检查 git 是否是 dirty 状态
    let is_dirty = match Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
    {
        Ok(output) => !output.stdout.is_empty(),
        Err(_) => false,
    };

    // 构建完整的版本字符串
    let version = if is_dirty {
        format!("{}-dirty", commit_hash)
    } else {
        commit_hash
    };

    // 将版本信息设置为环境变量，以便在代码中使用
    println!("cargo:rustc-env=GIT_VERSION={}", version);

    // 确保每次构建都重新运行此脚本
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}
