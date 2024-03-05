CREATE TABLE IF NOT EXISTS PalletListings (
    listing_id BIGSERIAL PRIMARY KEY,
    nft_owner VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    min_price BIGINT NOT NULL CHECK (min_price >= 0 AND min_price < 10000000000),
    block_height VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    listed BOOLEAN NOT NULL DEFAULT true
);

-- Corrected Indexes for optimizing queries
CREATE INDEX IF NOT EXISTS idx_nft_owner ON PalletListings(nft_owner);
CREATE INDEX IF NOT EXISTS idx_nft ON PalletListings(nft_address, token_id);
CREATE INDEX IF NOT EXISTS idx_txhash ON PalletListings(txhash);

-- Corrected Sales table with missing comma
CREATE TABLE IF NOT EXISTS Sales (
    sale_id BIGSERIAL PRIMARY KEY,
    block_height VARCHAR(255) NOT NULL , 
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255), -- Corrected: Added missing comma
    nft_owner VARCHAR(255) NOT NULL,
    previous_owner VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    sale_price BIGINT NOT NULL CHECK (sale_price >= 0 AND sale_price < 10000000000)
);

-- Corrected table name and column typo
CREATE TABLE IF NOT EXISTS WithdrawListings (
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL, -- Corrected: Typo fixed from 'toke_id'
    withdraw_listing_id BIGSERIAL PRIMARY KEY, -- Assuming 'withdraw_listings' was a typo
    transaction_hash VARCHAR(255) NOT NULL,
    withdraw_listing_price BIGINT NOT NULL CHECK (withdraw_listing_price >= 0 AND withdraw_listing_price < 10000000000)
);
