use std::error::Error;
use std::env::args;

use nustify::notification::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<_>>();
    let key = args.get(0).ok_or("invalid ifttt key")?;
    let image = args.get(1).unwrap_or(&"https://i.imgur.com/SFmiPRo.png".to_owned()).clone();
    let notification = Builder::new("A nice image".to_owned())
        .image_url(image)
        .build();
    futures::executor::block_on(async {
        nustify::send(&notification, "nustify", &key).await
    })?;
    Ok(())
}
