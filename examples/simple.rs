use std::error::Error;
use std::env::args;

use nustify::notification::Notification;

fn main() -> Result<(), Box<dyn Error>> {
    let key = args().skip(1).next().ok_or("invalid ifttt key")?;
    let notification = Notification::new(None, "Hello from Rust".to_owned());
    futures::executor::block_on(async {
        nustify::send(&notification, "nustify", &key).await
    })?;
    Ok(())
}
