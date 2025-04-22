use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
}
