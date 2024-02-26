use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::parser::types::PalletListing;

pub async fn insert_listing(pool: &sqlx::Pool<sqlx::Postgres>, listing: PalletListing) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO PalletListings (owner, nft_address, token_id, min_price, block_height, txhash, listed) VALUES ($1, $2, $3, $4, $5, $6, $7)",
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
    nft_token_id: &str
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE PalletListings SET listed = $1 WHERE nft_address = $2 AND token_id = $3",
        false, // Set listed to false
        nft_contract_address, // Use the NFT contract address to identify the listing
        nft_token_id // Use the NFT token ID to identify the listing
    )
    .execute(pool)
    .await?;

    Ok(())
}