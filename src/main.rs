use std::error::Error;
use mqtt_async_client::client::{Client, Publish};
use tokio::{time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger from the environment
    env_logger::init();

    let mut client = Client::builder()
        .set_host("localhost".to_owned())
        .set_port(1883)
        .set_connect_retry_delay(Duration::from_secs(1))
        .build()?;

    client.connect().await?;


    let p = Publish::new(String::from("the_topic"), "❤".as_bytes().to_vec());
    client.publish(&p).await?;

    Ok(())
}
