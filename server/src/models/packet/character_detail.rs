use crate::util::types::FixedLengthArray;

#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct CharacterDetails {
    opcode: u16,
    pub statpoints: u16,
    pub skillpoints: u16,
    pub max_hitpoints: u32,
    pub max_mana: u32,
    pub max_stamina: u32,
    pub direction: u16,
    pub prev_exp: u32,
    pub next_exp: u32,
    pub current_exp: u32,
    pub gold: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub kills: u32,
    pub deaths: u32,
    pub victories: u32,
    pub defeats: u32,
    pub guild_name: FixedLengthArray<25>,
    
    // New fields for NFT ownership and Eterna Token balance
    pub nft_ownership: Vec<u32>, // List of owned NFT IDs
    pub eterna_token_balance: u64, // Balance of Eterna Tokens
}

impl CharacterDetails {
    /// Default initializes the packet.
    pub fn new() -> Self {
        Self {
            opcode: 0x0105,
            nft_ownership: Vec::new(),
            eterna_token_balance: 0,
            ..Default::default()
        }
    }
}
