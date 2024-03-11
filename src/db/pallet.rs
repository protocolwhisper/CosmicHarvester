use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::parser::types::{PalletListing, Sales, Withdraw_listings};

pub async fn insert_listing(
    pool: &sqlx::Pool<sqlx::Postgres>,
    listing: PalletListing,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO PalletListings (nft_owner, nft_address, token_id, min_price, block_height, txhash, listed) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        listing.owner,
        listing.nft_address,
        listing.token_id,
        listing.min_price,
        listing.block_height,
        listing.txhash,
        listing.listed
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_listing_to_unlisted(
    pool: &sqlx::Pool<sqlx::Postgres>,
    nft_contract_address: &str,
    nft_token_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE PalletListings SET listed = $1 WHERE nft_address = $2 AND token_id = $3",
        false,                // Set listed to false
        nft_contract_address, // Use the NFT contract address to identify the listing
        nft_token_id          // Use the NFT token ID to identify the listing
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn eliminate_listing(
    pool: &sqlx::Pool<sqlx::Postgres>,
    nft_contract_address: &str,
    nft_token_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM PalletListings WHERE nft_address = $1 and token_id = $2",
        nft_contract_address,
        nft_token_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_owner() {
    // Function to get owner before updating mostly used for the sale thingy
    // For getting previous owner then
}

pub async fn update_owner(
    pool: &sqlx::Pool<sqlx::Postgres>,
    new_owner: &str,
    nft_contract_address: &str,
    nft_token_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE PalletListings SET nft_owner = $1 where nft_address = $2 AND token_id= $3",
        new_owner,
        nft_contract_address,
        nft_token_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_sales(
    pool: &sqlx::Pool<sqlx::Postgres>,
    sales: Sales,
) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO Sales (block_height , nft_address, token_id , nft_owner , previous_owner , txhash , sale_price) VALUES ($1, $2, $3, $4, $5, $6 , $7)",
    sales.block_height,
    sales.nft_address,
    sales.token_id,
    sales.nft_owner,
    sales.previous_owner,
    sales.txhash,
    sales.sale_price).execute(pool).await?;
    Ok(())
}

pub async fn insert_withdraw(
    pool: &sqlx::Pool<sqlx::Postgres>,
    withdraw: Withdraw_listings,
) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO WithdrawListings (block_height , nft_address , token_id , transaction_hash , withdraw_listing_price) VALUES ($1 , $2 , $3 , $4 , $5)",
    withdraw.block_height,
    withdraw.nft_address,
    withdraw.token_id,
    withdraw.transaction_hash,
    withdraw.withdraw_listing_price,
    ).execute(pool).await?;
    Ok(())
}
