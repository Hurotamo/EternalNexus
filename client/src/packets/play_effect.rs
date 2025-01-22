pub fn play_effect_packet(packet_payload: &[u8]) -> Result<(), String> {
    if packet_payload.len() < 8 {
        return Err("Invalid packet payload length".to_string());
    }

    let player_id = packet_payload[2]; // Extract player ID from the packet

    // Get the player by their ID
    if let Some(player) = get_player(player_id) {
        // Play the effect
        let player_position = player.position(); // Assuming a method to get the player's position
        let scene = packet_payload[7]; // Extract scene from the packet
        let effect = packet_payload[6]; // Extract effect from the packet

        // Call to play_graphical_effect with player's position, scene, and effect
        play_graphical_effect(player_position, scene, effect)?;
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

fn get_player(player_id: u8) -> Option<Player> {
    // Retrieve the player by ID using CLIENT_BASE
    let player_address = CLIENT_BASE + (player_id as usize * std::mem::size_of::<Player>());
    unsafe {
        // Assuming a way to dereference the player address
        let player: &Player = &*(player_address as *const Player);
        Some(player.clone()) // Return a clone of the player
    }
}

fn play_graphical_effect(position: Position, scene: u8, effect: u8) -> Result<(), String> {
    // Implementation for playing the graphical effect
    // For example:
    // if let Err(e) = effect.play(position, scene) {
    //     return Err(e.to_string());
    // }
    Ok(())
}
