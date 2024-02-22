use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::parser::types::PalletListing;

pub async fn insert_listing(listing: PalletListing) -> Result<(), sqlx::Error> {
    // Load the DATABASE_URL from an environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection pool
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // Insert the listing into the database
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
    .execute(&pool)
    .await?;

    Ok(())
}
