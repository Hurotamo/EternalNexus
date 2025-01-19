#[repr(C, packed)]
pub struct CGuild {
    pub id: u32,
    pub name: FixedLengthArray<32>,
    pub members: Vec<u32>, // List of member IDs
    pub wars: Vec<u32>, // List of guild war IDs
}

#[repr(C, packed)]
pub struct CGuildCreate {
    pub name: FixedLengthArray<32>,
    pub leader_id: u32,
}

impl CGuild {
    pub fn new(id: u32, name: &str) -> Self {
        let mut guild_name = FixedLengthArray::default();
        guild_name.copy_from_slice(&name.as_bytes()[..name.len().min(32)]);
        
        CGuild {
            id,
            name: guild_name,
            members: Vec::new(),
            wars: Vec::new(),
        }
    }
}
</create_file>
