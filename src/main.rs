mod db;
mod parser;
use std::str::FromStr;

//use cosmrs::tendermint-rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::rpc::{Client, SubscriptionClient, WebSocketClient};
use cosmrs::tx::Tx;
use futures::io::Window;
// We don't need this yet
use futures::StreamExt;
use sqlx::postgres::PgPoolOptions;
use std::{env, fs};
use tendermint_rpc::query::Query;
//Dependencies for writing the file :
use std::fs::OpenOptions;
use std::io::Write;

use crate::db::pallet::{insert_listing, insert_sales, update_listing_to_unlisted, update_owner};
use crate::parser::auctiondetails::parse_coin;
use crate::parser::types::{BlockchainEvent, PalletListing, Sales, Withdraw_listings};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Establish a connection pool
    let pool = PgPoolOptions::new().connect(&database_url).await?;
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
                println!("Received new event: {:?}", event.events);
                if let Ok(event_json_str) = serde_json::to_string(&event.events) {
                    match serde_json::from_str::<BlockchainEvent>(&event_json_str) {
                        Ok(blockchain_event) => {
                            match blockchain_event {
                                BlockchainEvent::CreateAuction(event_data) => {
                                    // Handle CreateAuctionEvent
                                    println!("Handling CreateAuctionEvent");
                                    println!("Parsing values");
                                    let parsed_min_price = parse_coin(
                                        &event_data.wasm_create_auction_min_price[0].to_string(),
                                    );
                                    //Should check if exist so it can just update the value
                                    let listing = PalletListing {
                                        owner: event_data.message_sender[0].to_string(),
                                        nft_address: event_data.wasm_create_auction_nft_address[0]
                                            .to_string(),
                                        token_id: event_data.wasm_create_auction_nft_token_id[0]
                                            .to_string(),
                                        min_price: parsed_min_price.unwrap().amount.to_string(),
                                        block_height: event_data.tx_height[0].to_string(),
                                        txhash: event_data.tx_hash[0].to_string(),
                                        listed: true,
                                    };

                                    println!("This is the min_price {:?}", listing.min_price);

                                    if let Err(e) = insert_listing(&pool, listing).await {
                                        eprintln!("Failed to insert listing: {}", e);
                                    }
                                }
                                BlockchainEvent::BuyNow(event_data) => {
                                    let new_owner = event_data.wasm_sender[0].to_string();
                                    let nft_contract_address =
                                        event_data.wasm_buy_now_nft_address[0].to_string();
                                    let token_id =
                                        event_data.wasm_buy_now_sale_price[0].to_string();
                                    let block_height = event_data.tx_height[0].to_string();

                                    // Need to the conversion

                                    let buynow = Sales {
                                        block_height: event_data.tx_height[0].to_string(),
                                        nft_address: event_data.wasm_buy_now_nft_address[0]
                                            .to_string(),
                                        token_id: event_data.wasm_token_id[0].to_string(),
                                        nft_owner: event_data.wasm_buy_now_nft_seller[0]
                                            .to_string(),
                                        previous_owner: event_data.transfer_recipient[0]
                                            .to_string(),
                                        txhash: event_data.tx_hash[0].to_string(),
                                        sale_price: event_data.wasm_buy_now_sale_price[0]
                                            .to_string(),
                                    };

                                    /* if let Err(e) = update_owner(
                                        &pool,
                                        &new_owner,
                                        &nft_contract_address,
                                        &token_id,
                                    )
                                    .await
                                    {
                                        eprintln!("Failed to update owner for BuyNow method {}", e);
                                    } */

                                    if let Err(e) = insert_sales(&pool, buynow).await {
                                        eprintln!("Failed to insert Sales: {}", e);
                                    }

                                    if let Err(e) = update_listing_to_unlisted(
                                        &pool,
                                        &nft_contract_address,
                                        &token_id,
                                    )
                                    .await
                                    {
                                        eprint!("Failed to update listing : {}", e);
                                    }

                                    //Should we keep current owners in a different table?
                                }
                                BlockchainEvent::CancelAuction(event_data) => {
                                    let nft_contract_address =
                                        event_data.wasm_cancel_auction_nft_address[0].to_string();
                                    let token_id =
                                        event_data.wasm_cancel_auction_nft_token_id[0].to_string();
                                    let w_listings = Withdraw_listings {
                                        block_height: event_data.tx_height[0].to_string(),
                                        nft_address: event_data.wasm_cancel_auction_nft_address[0]
                                            .to_string(),
                                        token_id: event_data.wasm_cancel_auction_nft_token_id[0]
                                            .to_string(),
                                        transaction_hash: event_data.tx_hash[0].to_string(),
                                        withdraw_listing_price: event_data.coin_spent_amount[0]
                                            .to_string(),
                                    };

                                    if let Err(e) = update_listing_to_unlisted(
                                        &pool,
                                        &nft_contract_address,
                                        &token_id,
                                    )
                                    .await
                                    {
                                        eprint!("Failed to update listing : {}", e);
                                    }

                                    // Handle CancelAuctionEvent

                                    /* if let Err(e) = update_listing_to_unlisted(
                                        &pool,
                                        &nft_contract_address[0].to_string(),
                                        &nft_token_id[0].to_string(),
                                    )
                                    .await
                                    {
                                        eprintln!("Failed to update listing: {}", e);
                                    }
                                    println!("Handling CancelAuctionEvent"); */
                                }
                            }
                        }
                        Err(e) => println!("Could not deserialize event: {:?}", e),
                    }
                } else {
                    println!("Failed to serialize event.events to a JSON string");
                }
            }
            Err(e) => println!("Error processing new event: {:?}", e),
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
