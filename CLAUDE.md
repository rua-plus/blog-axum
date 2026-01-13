# blog-axum 项目文档

## 项目概述

blog-axum 是一个使用 Axum 框架构建的现代化博客应用程序。Axum 是一个基于 Tokio 的 Rust Web 框架，提供了简单、安全和可扩展的 Web 开发体验。该项目旨在构建一个高性能、可维护的博客系统，支持内容管理、用户管理和评论等核心功能。

## 技术栈

- **后端框架**: Axum (0.8.8)
- **异步运行时**: Tokio (1.49.0)
- **日志系统**: Tracing (0.1.44) + Tracing Subscriber (0.3.22)
- **HTTP 中间件**: Tower HTTP (0.6.8)
- **序列化/反序列化**: Serde (1.0) + Serde JSON (1.0)
- **错误处理**: Anyhow (1.0.100)
- **Rust 版本**: 2024 Edition

## 项目结构

```
blog-axum/
├── src/
│   ├── main.rs               # 项目入口文件
│   │   └── - 初始化追踪系统
│   │   └── - 获取 Git 版本信息
│   │   └── - 创建并启动服务器
│   └── utils/                # 工具函数目录
│       └── - 追踪系统初始化函数
├── lib/                      # 库代码目录
│   └── blog/                 # 博客核心功能库（git 子模块）
│       ├── .claude/          # Claude 配置目录
│       ├── sql/              # 数据库 SQL 文件
│       ├── CLAUDE.md         # 子模块文档
│       ├── README.md         # 子模块说明
│       └── interface.ts      # TypeScript 接口定义
├── build.rs                  # 构建脚本
├── target/                   # 编译输出目录
├── Cargo.toml                # Cargo 配置文件
├── Cargo.lock                # Cargo 依赖锁定文件
├── .gitignore                # Git 忽略文件
├── .gitmodules               # Git 子模块配置
└── .git/                     # Git 仓库
```

## 核心功能

### 1. 基础架构

- 使用 Axum 框架构建 RESTful API
- 集成 Tracing 系统进行日志记录和调试
- 支持环境变量配置
- 健康检查和监控端点

### 2. 版本管理

- 自动获取 Git 版本信息
- 支持在运行时显示版本号
- 便于部署和问题追踪

### 3. 路由系统

- 根路径 (`/`) 返回 "RUA" 字符串
- 可扩展的路由系统，支持添加更多端点
- 使用 Axum 的路由宏简化路由定义

## 开发环境

### 安装 Rust

确保您的系统上已安装 Rust。您可以通过以下命令检查 Rust 是否已安装：

```bash
rustc --version
cargo --version
```

如果未安装，请访问 [rust-lang.org](https://www.rust-lang.org/) 下载并安装。

### 克隆项目

```bash
git clone <仓库地址> --recurse-submodules
cd blog-axum
```

### 安装依赖

```bash
cargo build
```

## 运行项目

### 调试模式

```bash
cargo run
```

服务器将在 http://0.0.0.0:8000 上运行。

### 发布模式

```bash
cargo run --release
```

## 构建项目

### 调试模式

```bash
cargo build
```

### 发布模式

```bash
cargo build --release
```

## 测试项目

### 运行所有测试

```bash
cargo test
```

### 运行特定测试

```bash
cargo test <测试名称>
```

## 依赖管理

项目使用 Cargo 管理依赖。所有依赖项在 `Cargo.toml` 文件中定义。

### 添加依赖

```bash
cargo add <依赖名称>
```

### 查看依赖树

```bash
cargo tree
```

## 代码风格

项目遵循 Rust 官方代码风格指南。您可以使用 `rustfmt` 工具格式化代码：

```bash
cargo fmt
```

### 检查代码风格

```bash
cargo clippy
```

## 文档

### 生成文档

```bash
cargo doc
```

### 查看文档

```bash
cargo doc --open
```

## 日志和调试

项目使用 Tracing 系统进行日志记录。您可以通过设置 `RUST_LOG` 环境变量来控制日志级别：

```bash
RUST_LOG=debug cargo run
```

支持的日志级别：

- `trace`: 最详细的调试信息
- `debug`: 调试信息
- `info`: 普通信息
- `warn`: 警告信息
- `error`: 错误信息

## 架构设计

### 应用程序架构

```rust
// 初始化追踪系统
init_tracing()?;

// 获取并显示 Git 版本信息
let git_version = option_env!("GIT_VERSION").unwrap_or("unknown");
info!("Git Version: {}", git_version);

// 创建路由
let app = Router::new().route("/", get(root));

// 启动服务器
let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
axum::serve(listener, app).await?;
```

### 核心组件

1. **追踪系统初始化**: 负责配置和启动 Tracing 系统
2. **版本信息管理**: 从环境变量获取 Git 版本信息
3. **路由配置**: 定义应用程序的路由和处理函数
4. **服务器启动**: 绑定到端口并启动 HTTP 服务器

## 扩展和贡献

### 添加新路由

在 `main.rs` 文件中添加新的路由处理函数：

```rust
async fn hello() -> &'static str {
    "Hello, World!"
}

// 在创建路由时添加新路由
let app = Router::new()
    .route("/", get(root))
    .route("/hello", get(hello));
```

### 贡献指南

1. Fork 项目
2. 创建新分支
3. 提交更改
4. 创建 Pull Request

## 许可证

[在此处添加许可证信息]

---

## 注意事项

- 确保所有依赖项已正确安装
- 遵循 Rust 代码风格指南
- 编写测试覆盖您的代码
- 保持日志记录的一致性
- 定期更新依赖项以确保安全和稳定性
