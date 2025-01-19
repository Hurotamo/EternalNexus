pub mod packets;

#[derive(Debug, Clone)]
pub struct NFT {
    pub id: u32,
    pub owner: Pubkey,
    pub metadata: String,
    pub rarity: String,
}

#[derive(Debug, Clone)]
pub struct UserTransaction {
    pub user_id: Pubkey,
    pub nft_id: u32,
    pub transaction_type: String, // "mint" or "trade"
    pub timestamp: u64,
}

mod action_bar;
pub use action_bar::*;

mod cuser;
pub use cuser::*;

mod cobject;
pub use cobject::*;

mod npc;
pub use npc::CNpc;

mod citem;
pub use citem::*;

mod cskill;
pub use cskill::*;

mod cquest;
pub use cquest::*;

mod crypto;
pub use crypto::*;

mod guild;
pub use guild::*;

mod critical_section;
pub use critical_section::*;

mod damage;
pub use damage::*;

mod exchange;
pub use exchange::*;

mod linked_list;
pub use linked_list::*;

mod svector;
pub use svector::*;

mod sconnection;
pub use sconnection::*;

mod shop;
pub use shop::*;

mod party;
pub use party::*;

mod friend;
pub use friend::*;

mod st_billing_item_info;
pub use st_billing_item_info::*;

mod minigame;
pub use minigame::*;
