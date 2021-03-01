use std::env::args;
use std::error::Error;

use nustify::notification::Builder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<_>>();
    let key = args.get(0).ok_or("invalid ifttt key")?;
    let image = args.get(1).unwrap_or(&"https://i.imgur.com/SFmiPRo.png".to_owned()).clone();
    let notification = Builder::new("A nice image".to_owned())
        .image_url(image)
        .build();
    nustify::send(&notification, "nustify", &key).await?;
    Ok(())
}
