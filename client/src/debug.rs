pub fn format_debug_info(mouse_x: usize, mouse_y: usize, fps: f32) -> String {
    format!("fps={:.2}, mx={}, my={}", fps, mouse_x, mouse_y)
}

pub fn format_debug_info_with_additional_details(mouse_x: usize, mouse_y: usize, fps: f32, additional_info: &str) -> String {
    format!("FPS: {:.2}, Mouse X: {}, Mouse Y: {}, Additional Info: {}", fps, mouse_x, mouse_y, additional_info)
}
