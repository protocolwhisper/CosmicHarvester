use serde::{Deserialize, Serialize};
pub struct AuctionDetails {
    pub contract_address: String,
    pub min_price: String, // Assuming this could be simplified as a String for demonstration
    pub nft_address: String,
    pub nft_token_id: String,
}

pub struct PalletListing {
    pub owner: String,
    pub nft_address: String,
    pub token_id: String,
    pub min_price: String,
    pub block_height: String,
    pub txhash: String,
    pub listed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventData {
    #[serde(rename = "coin_spent.amount")]
    coin_spent_amount: Vec<String>,
    #[serde(rename = "coin_spent.spender")]
    coin_spent_spender: Vec<String>,
    #[serde(rename = "execute._contract_address")]
    execute_contract_address: Vec<String>,
    #[serde(rename = "message.action")]
    message_action: Vec<String>,
    #[serde(rename = "message.module")]
    message_module: Vec<String>,
    #[serde(rename = "message.sender")]
    pub message_sender: Vec<String>,
    #[serde(rename = "tm.event")]
    tm_event: Vec<String>,
    #[serde(rename = "tx.acc_seq")]
    tx_acc_seq: Vec<String>,
    #[serde(rename = "tx.fee")]
    tx_fee: Vec<String>,
    #[serde(rename = "tx.fee_payer")]
    tx_fee_payer: Vec<String>,
    #[serde(rename = "tx.hash")]
    pub tx_hash: Vec<String>,
    #[serde(rename = "tx.height")]
    pub tx_height: Vec<String>,
    #[serde(rename = "tx.signature")]
    tx_signature: Vec<String>,
    #[serde(rename = "wasm-create_auction._contract_address")]
    pub wasm_create_auction_contract_address: Vec<String>,
    #[serde(rename = "wasm-create_auction.min_price")]
    pub wasm_create_auction_min_price: Vec<String>,
    #[serde(rename = "wasm-create_auction.nft_address")]
    pub wasm_create_auction_nft_address: Vec<String>,
    #[serde(rename = "wasm-create_auction.nft_token_id")]
    pub wasm_create_auction_nft_token_id: Vec<String>,
    #[serde(rename = "wasm._contract_address")]
    pub wasm_contract_address: Vec<String>,
    #[serde(rename = "wasm.action")]
    wasm_action: Vec<String>,
    #[serde(rename = "wasm.recipient")]
    wasm_recipient: Vec<String>,
    #[serde(rename = "wasm.sender")]
    wasm_sender: Vec<String>,
    #[serde(rename = "wasm.spender")]
    wasm_spender: Vec<String>,
    #[serde(rename = "wasm.token_id")]
    wasm_token_id: Vec<String>,
}
