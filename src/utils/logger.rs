use env_logger::{Builder, Env};
use log::LevelFilter;

pub fn setup_logger() {
    let env = env_logger::Env::new()
        .filter("RUST_LOG")
        .write_style("RUST_LOG_STYLE");

    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .format_module_path(true)
        .format_target(false)
        .format_level(true)
        .filter_module("sqlx", log::LevelFilter::Warn)
        .init();
}