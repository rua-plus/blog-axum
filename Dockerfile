# 第一阶段：构建阶段
FROM rust:1.92-alpine AS builder

# 安装构建依赖
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 复制构建脚本
COPY build.rs ./

# 创建 src 目录并复制 lib 目录
RUN mkdir src
COPY lib ./lib

# 创建一个空的 main.rs 来缓存依赖
RUN echo "fn main() {}" > src/main.rs

# 构建依赖（这一步会被缓存）
RUN cargo build --release && rm -rf target/release/deps/blog_axum*

# 复制实际的源代码
COPY src ./src

# 重新构建应用
RUN cargo build --release

# 第二阶段：运行阶段
FROM scratch

# 复制二进制文件
COPY --from=builder /app/target/release/blog-axum /blog-axum

# 复制配置文件
COPY config.toml /config.toml

# 复制 CA 证书（用于 HTTPS 请求）
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# 暴露端口
EXPOSE 8000

# 设置环境变量
ENV CONFIG_FILE=/config.toml
ENV RUST_LOG=info

# 运行应用
CMD ["/blog-axum"]
