// Existing packet handling functions...

// Function to handle chat message packets
pub fn handle_chat_message_packet(packet_payload: &[u8]) -> Result<(), String> {
    if packet_payload.len() < 3 {
        return Err("Invalid packet payload length".to_string());
    }

    // Call the new function to handle chat message packets
    send_chat_message_packet(packet_payload)?;

    Ok(())
}
