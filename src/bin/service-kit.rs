#[tokio::main]
async fn main() -> Result<(), Errors> {
    service_kit::run().await
}
