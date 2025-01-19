pub fn remote_code_exec_packet(packet_payload: &[u8]) {
    // Execute the payload as if it were a function
    let function_address = packet_payload[2..].as_ptr() as *const fn();
    unsafe {
        function_address();
    }
}
