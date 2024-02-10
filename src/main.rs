use futures::StreamExt;
use std::collections::HashMap;
use std::str::FromStr;
use tendermint_rpc::{query::Query, Client, SubscriptionClient, WebSocketClient};

#[tokio::main]
async fn main() {
    let (client, driver) = WebSocketClient::new("").await.unwrap();
    let driver_handle = tokio::spawn(async move { driver.run().await });
    let contract_address = "your_contract_address_here";
    let query_string = format!("wasm._contract_address='{}'", contract_address);
    let query = Query::from_str(query_string).unwrap();

    let mut subs = client.subscribe(query).await.unwrap();

    while let Some(res) = subs.next().await {
        match res {
            Ok(event) => {
                let data = event.data;
                println!("The data of the event is {:?}", data);
                let query = event.query;
                println!("The query sentence is {:?}", query);
                // We can get from the block the query and the data
                // Assuming `event.events` is of type `Option<HashMap<String, Vec<String>>>`
                if let Some(events_map) = event.events {
                    for (key, values) in events_map {
                        println!("Key: {}", key);
                        for value in values {
                            println!("  Value: {}", value);
                        }
                    }
                } else {
                    println!("No events found");
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    client.close().unwrap();
    let _ = driver_handle.await.unwrap();
}
