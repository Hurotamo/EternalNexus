pub mod additional_stats;
pub use additional_stats::*;

pub mod character_details;
pub use character_details::*;

pub mod character_list;
pub use character_list::*;

pub mod summon_request;
pub use summon_request::*;

// New packet types for NFT transactions and player actions
#[repr(C, packed)]
#[derive(Debug)]
pub struct NFTTransactionPacket {
    pub user_id: Pubkey,
    pub nft_id: u32,
    pub transaction_type: String, // "mint" or "trade"
    pub timestamp: u64,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct PlayerActionPacket {
    pub user_id: Pubkey,
    pub action_type: String, // e.g., "earn_tokens", "trade_nft"
    pub amount: u64, // Amount for actions like earning tokens
}
