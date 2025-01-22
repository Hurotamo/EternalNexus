// Constants for player attributes
pub const PLAYER_ATTRIBUTES: [usize; 9] = [
    0x8BDA5C, // User's GM status
    0x8BEB64, // User's current map ID
    0x8B5444, // Player's x position
    0x8B5448, // Player's y position
    0x8B544C, // Player's z position
    0x4498C0, // Address for getting a player by ID
    0x8BEB78, // Player's own ID
    0x8B541C, // Current player's pointer
    0x8BDA68, // Player's level
];

pub const PLAYER_KILLS: usize = 0x22563CC; // Player's kills
pub const CLIENT_BASE: usize = 0x775550; // Base address for getting player instances

// Enum for player attributes
#[derive(Debug, PartialEq, Eq)]
pub enum PlayerAttribute {
    UserStatus,
    PlayerMap,
    PosX,
    PosY,
    PosZ,
    GetPlayer,
    PlayerId,
    PlayerPtr,
    PlayerLevel,
    PlayerKills,
}

// Function to get player attribute
pub fn get_player_attribute(attribute: PlayerAttribute) -> usize {
    match attribute {
        PlayerAttribute::UserStatus => PLAYER_ATTRIBUTES[0],
        PlayerAttribute::PlayerMap => PLAYER_ATTRIBUTES[1],
        PlayerAttribute::PosX => PLAYER_ATTRIBUTES[2],
        PlayerAttribute::PosY => PLAYER_ATTRIBUTES[3],
        PlayerAttribute::PosZ => PLAYER_ATTRIBUTES[4],
        PlayerAttribute::GetPlayer => PLAYER_ATTRIBUTES[5],
        PlayerAttribute::PlayerId => PLAYER_ATTRIBUTES[6],
        PlayerAttribute::PlayerPtr => PLAYER_ATTRIBUTES[7],
        PlayerAttribute::PlayerLevel => PLAYER_ATTRIBUTES[8],
        PlayerAttribute::PlayerKills => PLAYER_KILLS,
    }
}