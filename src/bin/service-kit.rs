#[tokio::main]
async fn main() -> Result<(), service_kit::Errors> {
    service_kit::run().await
}
