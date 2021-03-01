use std::env::args;
use std::error::Error;

use nustify::notification::Builder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<_>>();
    let key = args.get(0).ok_or("invalid ifttt key")?;
    let imgur_key = args.get(1).ok_or("invalid imgur key")?;
    let image_path = args.get(2).unwrap_or(&"examples/crab.png".to_owned()).clone();
    let image_data = std::fs::read(image_path)?;

    let notification = Builder::new("A nice image".to_owned())
        .imgur_image(imgur_key, image_data).await?
        .build();
    nustify::send(&notification, "nustify", &key).await?;
    Ok(())
}
