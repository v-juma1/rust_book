# Rust book 学习笔记

- **《The Rust Programming Language》或中文版《Rust权威指南》**

## 说明

- **每个知识点对应一个binary crate,即src下的一个文件，用cargo workspace管理**
- **用cargo run --bin 文件夹名称（即binary crate name）的方式运行**

## 第2章

- ​	添加crate 依赖：
  - 在Cargo.toml文件[dependencies]下可按照如下的格式添加依赖
  - rand = "0.7.0"
  - 上面的格式其实是rand = "^0.7.0"的简写，表示依赖在0.7.0以上，但小于0.8.0的版本，cargo build 第一次运行时会创建Cargo.lock文件，并将依赖版本写入，只有在Cargo.toml更新了0.8.0以上的版本时，Cargo.lock文件才会更新
