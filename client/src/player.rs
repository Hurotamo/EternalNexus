// Constants for player attributes
pub const USER_STATUS: usize = 0x8BDA5C; // User's GM status
pub const PLAYER_MAP: usize = 0x8BEB64; // User's current map ID
pub const POS_X: usize = 0x8B5444; // Player's x position
pub const POS_Y: usize = 0x8B5448; // Player's y position
pub const POS_Z: usize = 0x8B544C; // Player's z position
pub const GET_PLAYER: usize = 0x4498C0; // Address for getting a player by ID
pub const PLAYER_ID: usize = 0x8BEB78; // Player's own ID
pub const PLAYER_PTR: usize = 0x8B541C; // Current player's pointer
pub const PLAYER_KILLS: usize = 0x22563CC; // Player's kills
pub const PLAYER_LEVEL: usize = 0x8BDA68; // Player's level
pub const CLIENT_BASE: usize = 0x775550; // Base address for getting player instances

// Additional functions for managing player attributes can be added here
