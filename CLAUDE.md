# blog-axum 项目文档

## 项目概述

blog-axum 是一个使用 Axum 框架构建的博客应用程序。Axum 是一个基于 Tokio 的 Rust Web 框架，提供了简单、安全和可扩展的 Web 开发体验。

## 项目结构

```
blog-axum/
├── src/
│   ├── main.rs       # 项目入口文件
│   └── utils/        # 工具函数目录
├── lib/              # 库代码目录
│   └── blog/         # 博客核心功能库（git 子模块）
│       ├── .claude/
│       ├── sql/
│       ├── CLAUDE.md
│       ├── README.md
│       └── interface.ts
├── build.rs          # 构建脚本
├── target/           # 编译输出目录
├── Cargo.toml        # Cargo 配置文件
├── Cargo.lock        # Cargo 依赖锁定文件
├── .gitignore        # Git 忽略文件
├── .gitmodules       # Git 子模块配置
└── .git/             # Git 仓库
```

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

## 贡献指南

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
