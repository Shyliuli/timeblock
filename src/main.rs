// src/main.rs
//! 应用入口（仅框架）
//
// 模块引入，类似 C++ 的 include
mod data;
mod gui;
mod utils;

fn main() {
    utils::init_logging();
}