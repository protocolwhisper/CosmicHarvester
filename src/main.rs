mod parser;
use std::str::FromStr;

//use cosmrs::tendermint-rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::tx::Tx; // We don't need this yet
use futures::StreamExt;
use std::fs;
use tendermint_rpc::query::Query;
//Dependencies for writing the file :
use std::fs::OpenOptions;
use std::io::Write;

use crate::parser::types::{EventData, PalletListing};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the WebSocket endpoint of the Cosmos node
    let (client_ws, driver) =
        WebSocketClient::new("wss://rpc.ankr.com/sei/ws/c41b0a71a36f5853b6ef3868e1b04d42b9705940faef80d5f40dd34986319351")
            .await
            .expect("Failed to connect to WebSocket");
    tokio::spawn(driver.run());

    // Query for new blocks
    let query = "tm.event='Tx' AND wasm EXISTS";
    let mut subs = client_ws.subscribe(Query::from_str(query).unwrap()).await?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("newquery2.txt")
        .expect("Failed to open or create file");

    while let Some(msg) = subs.next().await {
        match msg {
            Ok(event) => {
                println!("Received new block: {:?}", event.events);
                // Attempt to serialize event.events to a JSON string
                if let Ok(event_json_str) = serde_json::to_string(&event.events) {
                    // Now attempt to deserialize event_json_str into EventData
                    match serde_json::from_str::<EventData>(&event_json_str) {
                        Ok(event_data) => {
                            // Successful deserialization, proceed with event_data
                            // Let's do a check to see if it's from the pallet contract

                            let listing = PalletListing {
                                owner: event_data.message_sender[0].to_string(),
                                nft_address: event_data.wasm_create_auction_nft_address[0]
                                    .to_string(),
                                token_id: event_data.wasm_create_auction_nft_token_id[0]
                                    .to_string(),
                                min_price: event_data.wasm_create_auction_min_price[0].to_string(),
                                block_height: event_data.tx_height[0].to_string(),
                                txhash: event_data.tx_hash[0].to_string(),
                                listed: true,
                            };

                            println!(
                                "The contract address is: {:?}",
                                event_data.wasm_create_auction_contract_address
                            );

                            println!(
                                "The address of the nft is: {:?}",
                                event_data.wasm_create_auction_nft_address
                            );
                            println!(
                                "The token is: {:?}",
                                event_data.wasm_create_auction_nft_token_id
                            );

                            println!("The owner is: {:?}", event_data.message_sender);
                        }

                        Err(_e) => {
                            // Failed to deserialize, event may not match expected structure
                            println!("This is not a pallet event.");
                        }
                    }
                } else {
                    println!("Failed to serialize event.events to a JSON string");
                }

                // Additional processing...
            }
            Err(e) => println!("Error processing new block: {:?}", e),
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {

    use jsonpath_rust::parser;

    use crate::parser::auctiondetails::parse_lines;

    #[test]
    fn parsing_http_block() {
        let buynow_event = parse_lines("./example-logs/historicalBlock.txt");
    }
}
