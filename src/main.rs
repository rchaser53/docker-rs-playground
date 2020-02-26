mod builder;
use builder::Builder;

#[macro_use]
mod containers;
use containers::GetContainerOption;

mod images;
mod request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Builder = Default::default();
    let container = client.container();
    let query = create_query_by_struct!(GetContainerOption, (all, Some(false))).unwrap();
    let containers = container.get_containers(&query).await;
    // ("rchaser53/redis", "latest").await;
    // let images = image.get_images().await;

    // url::form_urlencoded::Serializer::new(String::new())
    //   .extend_pairs(&GetContainerOption{
    //     all: false
    //   })
    //   .finish();

    println!("{:?}", containers);
    Ok(())
}
