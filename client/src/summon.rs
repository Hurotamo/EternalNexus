// Constants for summon functionality
const SUMMON_CHAR_NAME: &str = "PlayerName"; // Placeholder for the player's name
const DIALOGUE_TEXT: &str = "{} is summoning you to {}. Do you wish to accept?"; // Summon dialogue text
const DIALOGUE_TEXT_BOX_SIZE: usize = 128; // Size of the dialogue text box

// Function to write the destination map in the summon dialogue box
pub fn write_summon_destination(summon_dest_map_id: u32, summon_char_name: &str) -> String {
    let summon_dest_map_name = get_map_name(summon_dest_map_id); // Get the map name
    format!(DIALOGUE_TEXT, summon_char_name, summon_dest_map_name) // Format the output string
}

// Function to get the map name based on the ID
fn get_map_name(map_id: u32) -> String {
    match map_id {
        1 => "Map1".to_string(),
        2 => "Map2".to_string(),
        _ => "Unknown Map".to_string(),
    }
}

// Function to read the extra data in the summon packet
pub fn read_summon_packet(packet_payload: &[u8]) -> Result<(u32, u32), String> {
    if packet_payload.len() < 6 {
        return Err("Invalid packet payload length".to_string());
    }
    let character_id = read_packet_data(packet_payload, 0, 4); // Read character ID
    let summon_dest_map_id = read_packet_data(packet_payload, 4, 2); // Read destination map ID
    Ok((character_id, summon_dest_map_id))
}

// Function to read data from the packet
fn read_packet_data(packet_payload: &[u8], offset: usize, size: usize) -> u32 {
    let mut data = 0;
    for i in 0..size {
        data = (data << 8) | packet_payload[offset + i] as u32;
    }
    data
}