#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rupa_cli::run().await
}
