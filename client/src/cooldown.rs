pub const COOLDOWN_COLOR: u32 = 0xF0000000; // The colour of the cooldown dial.
pub const COOLDOWN_RETN: u32 = 0x4A9FD0;    // The address to return to.
pub const COOLDOWN_DRAW_RETN: u32 = 0x4AA08D; // The address to return to after drawing the text.

pub fn prepare_cooldown_text(remaining_time: u32) -> String {
    let minutes = remaining_time / 1000 / 60; // Convert milliseconds to minutes
    let seconds = (remaining_time / 1000) % 60; // Convert milliseconds to seconds
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn draw_cooldown_text(x: usize, y: usize, text: &str) {
    // Set color for the cooldown text
    let alpha = 0x00;
    let blue = 0x00;
    let green = 0xEC;
    let red = 0xFF;

    // Check if coordinates are out of bounds
    if y >= 5000 || x >= 5000 {
        // Handle inventory rendering
        let inventory_y = y + 6; // Move the text down by 6 pixels
        let inventory_x = x + 6; // Move the text 6 pixels to the right
        // Call the function to draw the text at inventory coordinates
        draw_text(inventory_x, inventory_y, text, alpha, blue, green, red);
    } else {
        // Draw the text at the specified coordinates
        draw_text(x, y, text, alpha, blue, green, red);
    }
}
