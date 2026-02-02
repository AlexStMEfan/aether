fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Aether CLI v{}", aether_core::VERSION);
}
