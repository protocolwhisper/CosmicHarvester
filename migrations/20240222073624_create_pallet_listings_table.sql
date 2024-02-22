CREATE TABLE IF NOT EXISTS PalletListings (
    listing_id BIGSERIAL PRIMARY KEY,
    owner VARCHAR(255) NOT NULL,
    nft_address VARCHAR(255) NOT NULL,
    token_id VARCHAR(255) NOT NULL,
    min_price VARCHAR(255) NOT NULL,
    block_height VARCHAR(255) NOT NULL,
    txhash VARCHAR(255) NOT NULL,
    listed BOOLEAN NOT NULL DEFAULT true
);

-- Indexes for optimizing queries
CREATE INDEX idx_owner ON PalletListings(owner);
CREATE INDEX idx_nft ON PalletListings(nft_address, token_id);
CREATE INDEX idx_txhash ON PalletListings(txhash);
