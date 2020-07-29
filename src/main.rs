use std::error::Error;
use rust_mqtt_playground::*;

#[tokio::main]
async fn main() {
    async_stuff().await;
}
// 0:16.41
// fn main() {
    // match sync_stuff() {
        // Ok(_) => {},
        // Err(e) => println!("error {}", e)

    // };
// }
