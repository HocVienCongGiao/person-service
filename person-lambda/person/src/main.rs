type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use lambda_http::{handler, lambda_runtime};
use person::func;

#[tokio::main]
async fn main() -> Result<(), Error> {
    print!("Start handle person lambda");
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}
