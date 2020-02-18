mod builder;
use builder::Builder;

mod images;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Builder = Default::default();
    let image = client.image();
    let images = image.get_images().await;

    println!("{:?}", images);
    Ok(())
}
