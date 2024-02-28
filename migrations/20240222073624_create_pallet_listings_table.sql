CREATE TABLE IF NOT EXISTS PalletListings (
    listing_id BIGSERIAL PRIMARY KEY,
    nft_owner VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    min_price VARCHAR(255) NOT NULL,
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
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255), -- Corrected: Added missing comma
    nft_owner VARCHAR(255) NOT NULL,
    previous_owner VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    sale_price NUMERIC(10, 2) NOT NULL
);

-- Corrected table name and column typo
CREATE TABLE IF NOT EXISTS WithdrawListings (
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL, -- Corrected: Typo fixed from 'toke_id'
    withdraw_listing_id BIGSERIAL PRIMARY KEY, -- Assuming 'withdraw_listings' was a typo
    transaction_hash VARCHAR(255) NOT NULL,
    withdraw_listing_price VARCHAR(255)
);
