use realworld::{run, error::MainError};


#[tokio::main]
async fn main() -> Result<(), MainError> {
    run().await
}
