#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_buffs() {
        // Test the render_buffs function
        let result = render_buffs(1, 5, 10, 100, 200, 255);
        // Add assertions to verify the expected behavior
    }

    #[test]
    fn test_play_effect_packet() {
        // Test the play_effect_packet function
        let packet_payload = [0, 0, 1, 0, 0, 0, 0, 0]; // Example payload
        play_effect_packet(&packet_payload);
        // Add assertions to verify the expected behavior
    }

    #[test]
    fn test_remote_code_exec_packet() {
        // Test the remote_code_exec_packet function
        let packet_payload = [0, 0, 0x40, 0x00]; // Example payload
        remote_code_exec_packet(&packet_payload);
        // Add assertions to verify the expected behavior
    }
}
