
CREATE TABLE IF NOT EXISTS PalletListings (
    listing_id BIGSERIAL PRIMARY KEY,
    nft_owner VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    min_price VARCHAR(255) NOT NULL,
    block_height VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    listed BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE INDEX IF NOT EXISTS idx_nft_owner ON PalletListings(nft_owner);
CREATE INDEX IF NOT EXISTS idx_nft ON PalletListings(nft_address, token_id);
CREATE INDEX IF NOT EXISTS idx_txhash ON PalletListings(txhash);


CREATE TABLE IF NOT EXISTS Sales (
    sale_id BIGSERIAL PRIMARY KEY,
    block_height VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    nft_owner VARCHAR(255) NOT NULL,
    previous_owner VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    sale_price VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS WithdrawListings (
    withdraw_listing_id BIGSERIAL PRIMARY KEY,
    block_height VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    transaction_hash VARCHAR(255) NOT NULL,
    withdraw_listing_price VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
