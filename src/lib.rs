use serde_json::json;
use std::error::Error;
use mqtt_async_client::client::{Client, Publish};
use tokio::{time::Duration};
use chrono::prelude::Utc;
use std::io::prelude::*;
use std::net::{ TcpStream, SocketAddr, IpAddr, Ipv4Addr, Shutdown };
use mqtt::packet::connect::ConnectPacket;
use mqtt::{Encodable, Decodable};
use mqtt::packet::{VariablePacket, PublishPacket, QoSWithPacketIdentifier};
use mqtt::TopicName;

pub async fn async_stuff () -> Result<(), Box<dyn Error>> {
    // Initialize the logger from the environment
    env_logger::init();

    let mut client = Client::builder()
        .set_host("localhost".to_owned())
        .set_port(1883)
        .set_connect_retry_delay(Duration::from_secs(1))
        .build()?;

    client.connect().await?;

    for i in 0..60_000 {
        let now = Utc::now();

        let msg = json!({
            "serial": "123456",
            "data": {
                "timestamp": now.to_rfc3339(),
            }
        });

        let p = Publish::new(String::from("the_topic"), msg.to_string().as_bytes().to_vec());

        match client.publish(&p).await {
            Err(err) => println!("err {}", err),
            Ok(()) => println!("written: {}", msg.to_string()),
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}

pub fn sync_stuff() -> std::io::Result<()> {
    fn get_stream() -> std::io::Result<TcpStream>{
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1883);

        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(2))?;

        let conn_packet = ConnectPacket::new("MQTT", "id-123");

        let mut buf = Vec::new();
        conn_packet.encode(&mut buf).unwrap();

        stream.write_all(&buf)?;
        Ok(stream)
    }

    let mut stream = get_stream()?;

    for i in 0..60_000 {
        let now = Utc::now();
        let msg = json!({
            "serial": "123456",
            "data": {
                "timestamp": now.to_rfc3339(),
            }
        });

        let packet = PublishPacket::new(
            TopicName::new("the_topic").unwrap(),
            QoSWithPacketIdentifier::Level1(10),
            msg.to_string().as_bytes().to_vec()
        );

        let mut buf = Vec::new();
        packet.encode(&mut buf).unwrap();

        match stream.write_all(&buf) {
            Err(err) => {
                println!("Could not write! {}", err);
                match stream.shutdown(Shutdown::Both) {
                    Ok(_) => println!("disconnected tcp stream"),
                    Err(e) => println!("error disconnecting {}", e)
                };
                stream = get_stream()?;
            },
            Ok(()) => println!("written: {}", msg.to_string()),
        };

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}
