mod builder;
use builder::Builder;

mod images;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Builder = Default::default();
    let image = client.image();
    let images = image.pull_image("busybox", "latest").await;

    println!("{:?}", images);
    Ok(())
}
