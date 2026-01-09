use box_system::webui;

#[tokio::main]
async fn main() {
    webui::run().await;
}
