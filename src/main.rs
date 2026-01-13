use tracing::info;

use crate::utils::init_tracing;

mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;

    // 获取 git 版本信息
    let git_version = option_env!("GIT_VERSION").unwrap_or("unknown");
    info!("Git Version: {}", git_version);
    Ok(())
}
