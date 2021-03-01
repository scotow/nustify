use std::env::args;
use std::error::Error;

use nustify::notification::Notification;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = args().skip(1).next().ok_or("invalid ifttt key")?;
    let notification = Notification::new(None, "Hello from Rust".to_owned());
    nustify::send(&notification, "nustify", &key).await?;
    Ok(())
}
