use std::str::FromStr;

//use cosmrs::tendermint-rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::tx::Tx; // We don't need this yet
use futures::StreamExt;

use tendermint_rpc::query::Query;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the WebSocket endpoint of the Cosmos node
    let (client_ws, driver) =
        WebSocketClient::new("wss://rpc.ankr.com/sei/ws/c41b0a71a36f5853b6ef3868e1b04d42b9705940faef80d5f40dd34986319351")
            .await
            .expect("Failed to connect to WebSocket");
    tokio::spawn(driver.run());

    // Query for new blocks
    let query = "tm.event = 'NewBlock'";
    let mut subs = client_ws.subscribe(Query::from_str(query).unwrap()).await?;

    while let Some(msg) = subs.next().await {
        match msg {
            Ok(event) => {
                println!("Received new block: {:?}", event.data);
                // Here you can add additional processing for the block data
                println!("Finish processing the block :)");
            }
            Err(e) => println!("Error processing new block: {:?}", e),
        }
    }

    Ok(())
}
