/// Configuration settings for the Eternal Nexus game.

pub struct GameConfig {
    pub blockchain: String,
    pub game_mechanics: String,
    pub user_settings: String,
    pub nft_minting_address: String, // Address for NFT minting
    pub eterna_token_address: String, // Address for Eterna Token
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            blockchain: String::from("Solana and Core"),
            game_mechanics: String::from("MMORPG with NFT integration"),
            user_settings: String::from("Default user settings"),
        }
    }
}
</create_file>
