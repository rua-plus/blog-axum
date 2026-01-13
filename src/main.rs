fn main() {
    // 获取 git 版本信息
    let git_version = option_env!("GIT_VERSION").unwrap_or("unknown");
    println!("Git Version: {}", git_version);
}
