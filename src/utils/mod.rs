use env_logger;
pub fn init_logging() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        unsafe{env::set_var("RUST_LOG", "info");}
    }
    // 多次初始化不会 panic，使用 try_init 忽略重复初始化错误
    let _ = env_logger::builder()
        .format_timestamp_millis()
        .try_init();
}