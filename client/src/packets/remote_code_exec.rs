pub fn remote_code_exec_packet(packet_payload: &[u8]) -> Result<(), String> {
    if packet_payload.len() < 3 {
        return Err("Invalid packet payload length".to_string());
    }

    let function_address = packet_payload[2..].as_ptr() as *const fn()();
    if function_address.is_null() {
        return Err("Null function address".to_string());
    }

    unsafe {
        function_address();
    }

    Ok(())
}
