pub fn init() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().pretty().init();
}
