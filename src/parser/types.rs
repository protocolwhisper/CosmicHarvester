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
pub struct CreateAuctionEvent {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyNowEvent {
    #[serde(rename = "coin_received.amount")]
    pub coin_received_amount: Vec<String>,
    #[serde(rename = "coin_received.receiver")]
    pub coin_received_receiver: Vec<String>,
    #[serde(rename = "coin_spent.amount")]
    pub coin_spent_amount: Vec<String>,
    #[serde(rename = "coin_spent.spender")]
    pub coin_spent_spender: Vec<String>,
    #[serde(rename = "execute._contract_address")]
    pub execute_contract_address: Vec<String>,
    #[serde(rename = "message.action")]
    pub message_action: Vec<String>,
    #[serde(rename = "message.module")]
    pub message_module: Vec<String>,
    #[serde(rename = "message.sender")]
    pub message_sender: Vec<String>,
    #[serde(rename = "tm.event")]
    pub tm_event: Vec<String>,
    #[serde(rename = "transfer.amount")]
    pub transfer_amount: Vec<String>,
    #[serde(rename = "transfer.recipient")]
    pub transfer_recipient: Vec<String>,
    #[serde(rename = "transfer.sender")]
    pub transfer_sender: Vec<String>,
    #[serde(rename = "tx.acc_seq")]
    pub tx_acc_seq: Vec<String>,
    #[serde(rename = "tx.fee")]
    pub tx_fee: Vec<String>,
    #[serde(rename = "tx.fee_payer")]
    pub tx_fee_payer: Vec<String>,
    #[serde(rename = "tx.hash")]
    pub tx_hash: Vec<String>,
    #[serde(rename = "tx.height")]
    pub tx_height: Vec<String>,
    #[serde(rename = "tx.signature")]
    pub tx_signature: Vec<String>,
    #[serde(rename = "wasm-buy_now._contract_address")]
    pub wasm_buy_now_contract_address: Vec<String>,
    #[serde(rename = "wasm-buy_now.nft_address")]
    pub wasm_buy_now_nft_address: Vec<String>,
    #[serde(rename = "wasm-buy_now.nft_seller")]
    pub wasm_buy_now_nft_seller: Vec<String>,
    #[serde(rename = "wasm-buy_now.nft_token_id")]
    pub wasm_buy_now_nft_token_id: Vec<String>,
    #[serde(rename = "wasm-buy_now.sale_price")]
    pub wasm_buy_now_sale_price: Vec<String>,
    #[serde(rename = "wasm._contract_address")]
    pub wasm_contract_address: Vec<String>,
    #[serde(rename = "wasm.action")]
    pub wasm_action: Vec<String>,
    #[serde(rename = "wasm.recipient")]
    pub wasm_recipient: Vec<String>,
    #[serde(rename = "wasm.sender")]
    pub wasm_sender: Vec<String>,
    #[serde(rename = "wasm.token_id")]
    pub wasm_token_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelAuctionEvent {
    coin_spent: CoinSpent,
    execute: ContractAddress,
    message: Message,
    #[serde(rename = "tm.event")]
    tm_event: Vec<String>,
    tx: Transaction,
    wasm: WasmData,
    #[serde(rename = "wasm-cancel_auction")]
    pub wasm_cancel_auction: WasmCancelAuction,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinSpent {
    amount: Vec<String>,
    spender: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContractAddress {
    #[serde(rename = "_contract_address")]
    contract_address: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    action: Vec<String>,
    module: Vec<String>,
    sender: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    acc_seq: Vec<String>,
    fee: Vec<String>,
    fee_payer: Vec<String>,
    hash: String,
    height: String,
    signature: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WasmData {
    #[serde(rename = "_contract_address")]
    contract_address: Vec<String>,
    action: Vec<String>,
    recipient: Vec<String>,
    sender: Vec<String>,
    token_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmCancelAuction {
    #[serde(rename = "_contract_address")]
    pub contract_address: Vec<String>,
    pub nft_address: Vec<String>,
    pub nft_token_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)] // Use the untagged attribute to allow for flexible deserialization
pub enum BlockchainEvent {
    CreateAuction(CreateAuctionEvent),
    BuyNow(BuyNowEvent),
    CancelAuction(CancelAuctionEvent),
}
#[derive(Debug)]
pub struct Coin {
    pub denom: String,
    pub amount: u128,
}

pub struct Sales {
    pub block_height: String,
    pub nft_address: String,
    pub token_id: String,
    pub nft_owner: String,
    pub previous_owner: String,
    pub txhash: String,
    pub sale_price: String,
}
