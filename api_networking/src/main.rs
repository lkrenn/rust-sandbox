// //use reqwest::Error;
// use tokio_tungstenite::{connect_async, tungstenite::Error as WsError};
// use url::Url;

// #[tokio::main]
// async fn main()-> Result<(), WsError> {// -> Result<(), Error> {

//     // REST

//     // let response = reqwest::get("https://api.kraken.com/0/public/Time")
//     //     .await?
//     //     .text()
//     //     .await?;

//     // let response = reqwest::get("https://api.kraken.com/0/public/Depth?pair=XBTUSD")
//     //     .await?
//     //     .text()
//     //     .await?;

//     // println!("Response: {}", response);


use futures_util::{SinkExt, StreamExt}; // Make sure these traits are in scope
use tokio;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("wss://ws.kraken.com/")?;

    // Connect to the WebSocket server
    let (ws_stream, _response) = connect_async(url).await?;

    // Now, correctly split ws_stream into a writer and reader parts
    let (mut write, read) = ws_stream.split();

    // Proceed to send messages and read responses
    // For example, to send a subscription message:
    let subscribe_command = serde_json::json!({
        "event": "subscribe",
        "pair": ["XBT/USD"],
        "subscription": {
            "name": "book",
            "depth": 10,
        }
    }).to_string();

    // Send the subscription message
    write.send(Message::Text(subscribe_command)).await?;

    // Process incoming messages
    let mut read = read;
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received: {}", msg),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
