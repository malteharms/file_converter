use std::io::Write;

pub fn initialize_logging() {
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("debug"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
