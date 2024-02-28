use super::types::{AuctionDetails, Coin};
use std::{error::Error, fs::read_to_string};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use jsonpath_rust::{JsonPathFinder, JsonPathQuery, JsonPathInst, JsonPathValue};


pub fn parse_line(line: &str) -> Vec<AuctionDetails>{
    let mut result = Vec::new();
    let json: Value = serde_json::from_str(line).unwrap();
    let logs = json.path("$.txs_results[*].log").unwrap();
    let logs_array = logs.as_array().unwrap();
    for log in logs_array {
        let log_str = log.to_string();
        let log_unescaped: String = serde_json::from_str(&log_str).unwrap();
        let log_json: Value = serde_json::from_str(&log_unescaped).unwrap();        
        let events = log_json.path("$..events[*]").unwrap();
        let events_array = events.as_array().unwrap();
        for ev in events_array {
            let _type: String = serde_json::from_str(&ev["type"].to_string()).unwrap();
            if _type == "wasm-create_auction" {
                let mut create_auction = AuctionDetails {
                    contract_address: String::from(""),
                    min_price: String::from(""),
                    nft_address: String::from(""),
                    nft_token_id: String::from("")
                };        
                let attributes = &ev["attributes"];
                let attributes_array = attributes.as_array().unwrap();
                for attribute in attributes_array {
                    let key: String = serde_json::from_str(&attribute["key"].to_string()).unwrap();
                    match key.as_str() {
                        "_contract_address" => create_auction.contract_address = serde_json::from_str(&attribute["value"].to_string()).unwrap(),
                        "min_price" => create_auction.min_price = serde_json::from_str(&attribute["value"].to_string()).unwrap(),
                        "nft_address" => create_auction.nft_address = serde_json::from_str(&attribute["value"].to_string()).unwrap(),
                        "nft_token_id" => create_auction.nft_token_id = serde_json::from_str(&attribute["value"].to_string()).unwrap(),
                        _ => {}
                    }
                }
                result.push(create_auction);
            }
        }
    }
    return result;
}

pub fn parse_lines(filename: &str) {
    for line in read_to_string(filename).unwrap().lines() {
        //print!("{}", line);
        for create_auction in parse_line(line) {
            print!("create auction: contract_address={}, min_price={}, nft_address={}, nft_token_id={}", 
                create_auction.contract_address, 
                create_auction.min_price, 
                create_auction.nft_address,
                create_auction.nft_token_id
            );
        }
    }
}



pub fn parse_coin(input: &str) -> std::result::Result<Coin, &'static str> {
    // Trim the leading and trailing brackets and quotes
    let trimmed = input.trim_matches(|c| c == '[' || c == ']' || c == '\"');

    // Directly trimming the known structure
    if let Some(start) = trimmed.find("denom: \"") {
        let remainder = &trimmed[start + 8..]; // Skip past "denom: \""
        if let Some(end) = remainder.find("\",") {
            let denom = &remainder[..end];
            if let Some(amount_start) = remainder.find("Uint128(") {
                let amount_part = &remainder[amount_start + 8..]; // Skip past "Uint128("
                if let Some(amount_end) = amount_part.find(")") {
                    let amount_str = &amount_part[..amount_end];
                    if let Ok(amount) = amount_str.parse::<u128>() {
                        return Ok(Coin {
                            denom: denom.to_string(),
                            amount,
                        });
                    }
                }
            }
        }
    }
    Err("Failed to parse amount")
}



