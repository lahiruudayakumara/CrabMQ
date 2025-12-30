pub fn init_with_level(level: &str) {
    // Use env_logger but allow overriding level via config when RUST_LOG not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", level);
    }
    let _ = env_logger::try_init();
}
