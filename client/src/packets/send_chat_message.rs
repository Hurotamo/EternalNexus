pub fn send_chat_message_packet(packet_payload: &[u8]) -> Result<(), String> {
    if packet_payload.len() < 3 {
        return Err("Invalid packet payload length".to_string());
    }

    let (effect_code, message_string) = packet_payload.split_at(3);

    if message_string.is_empty() {
        return Err("Message string is empty".to_string());
    }

    let effect_code = effect_code[2];

    let message = match String::from_utf8(message_string.to_vec()) {
        Ok(message) => message,
        Err(_) => return Err("Failed to convert message bytes to string".to_string()),
    };

    write_client_chat_text(&message, effect_code as u32)?;

    Ok(())
}
