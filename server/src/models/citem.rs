#[repr(C)]
#[repr(packed)]
#[derive(Debug)]
pub struct CItem {
    pub item_id: u32,
    pub item_type: u8,
    pub item_name: FixedLengthArray<32>,
    pub rarity: u8,
    pub attributes: [u32; 5], // Example attributes for the item
    pub owner: Pubkey, // Address of the item owner
    pub metadata: String, // Metadata for the NFT
}

impl CItem {
    pub fn new(item_id: u32, item_type: u8, name: &str, rarity: u8, attributes: [u32; 5], owner: Pubkey, metadata: String) -> Self {
        let mut item_name = FixedLengthArray::default();
        item_name.copy_from_slice(&name.as_bytes()[..name.len().min(32)]);
        
        CItem {
            item_id,
            item_type,
            item_name,
            rarity,
            attributes,
            owner,
            metadata,
        }
    }
}
