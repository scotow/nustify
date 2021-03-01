use std::env::args;
use std::error::Error;

use nustify::notification::Builder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = args().skip(1).next().ok_or("invalid ifttt key")?;
    let notification = Builder::new("Rusty Content".to_owned())
        .title("Hello from Rust".to_owned())
        .build();
    nustify::send(&notification, "nustify", &key).await?;
    Ok(())
}
