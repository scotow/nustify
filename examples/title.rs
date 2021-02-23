use std::error::Error;
use std::env::args;

use nustify::notification::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let event = Event::new("nustify");
    let key = Key::new(args().skip(1).next().ok_or("invalid ifttt key")?);
    let notification = Builder::new("Rust Content")
        .title("Rusty Notification")
        .build();
    futures::executor::block_on(async {
        nustify::send(&notification, &event, &key).await
    })?;
    Ok(())
}
