mod checker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    checker::run().await
}