// Function to prepare the cooldown text
pub fn prepare_cooldown_text(remaining_time: u32) -> String {
    let minutes = remaining_time / 60;
    let seconds = remaining_time % 60;
    format!("{}:{:02}", minutes, seconds)
}

// Function to draw the cooldown text
pub fn draw_cooldown_text(x: usize, y: usize, text: &str) {
    // Implementation for drawing the text at the specified coordinates
    // This would involve calling the appropriate graphics library functions
}
