use mandos::error::Result;
use mandos::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
