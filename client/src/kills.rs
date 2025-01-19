// Kill rank table
const KILL_RANK_TABLE: [u32; 32] = [
    1, 50, 300, 1000, 5000, 10000, 20000, 30000, 40000, 50000,
    70000, 90000, 110000, 130000, 150000, 200000, 250000, 300000,
    350000, 400000, 450000, 500000, 550000, 600000, 650000, 700000,
    750000, 800000, 850000, 900000, 1000000,
];

// Function to get the player's kill rank
pub fn get_player_kill_rank(player_kills: u32) -> usize {
    for (rank, &threshold) in KILL_RANK_TABLE.iter().enumerate() {
        if player_kills < threshold {
            return rank; // Return the rank if the player's kills are less than the threshold
        }
    }
    KILL_RANK_TABLE.len() - 1 // Return the highest rank if the player's kills exceed all thresholds
}
