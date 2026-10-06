mod app;
mod ui;

#[tokio::main]
async fn main() {
    // Keep startup and window setup inside the application module.
    app::PixtacioApp::run();
}
