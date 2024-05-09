#[tokio::main]
async fn main() -> Result<(), service_kit::Error> {
    service_kit::run().await
}
