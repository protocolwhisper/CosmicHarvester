use reqwest::Client;
use serde_json::{json, Value};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block_height = 57940547; // Specify your block height here
    let rpc_url = "https://rpc.ankr.com/sei"; // The Tendermint RPC endpoint

    // Construct the JSON RPC request payload
    let request_payload = json!({
        "jsonrpc": "2.0",
        "id": "block_results",
        "method": "block_results",
        "params": {
            "height": block_height
        }
    });

    let client = Client::new();
    let response = client
        .post(rpc_url)
        .json(&request_payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Assuming the structure of the response and extracting the events
    // You'll need to adjust the path according to the actual response structure
    if let Some(events) = response["result"]["events"].as_array() {
        for event in events {
            println!("Event: {:?}", event);
            // Further process each event as needed
        }
    } else {
        println!("No events found or different structure in response.");
    }

    Ok(())
}
