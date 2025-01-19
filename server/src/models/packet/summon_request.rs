#[repr(C, packed)]
#[derive(Default)]
pub struct SummonRequest {
    opcode: u16,
    pub id: u32,
    pub map: u16,
    
    // New fields for NFT-related requests
    pub nft_id: Option<u32>, // Optional NFT ID for summoning
}

impl SummonRequest {
    /// Default initializes the packet.
    pub fn new() -> Self {
        Self {
            opcode: 0x0223,
            nft_id: None, // Default to no NFT ID
            ..Default::default()
        }
    }
}
