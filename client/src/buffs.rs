/// Renders the player's own buffs on the screen.
pub fn render_buffs(skill_level: usize, skill_id: usize, y_position: usize, x_position: usize) {
    let skill_active_state = 1; // Skill active state (0 = Greyed out, 1 = Learned)
    let opacity = 0xFFFFFFFF; // Full opacity
    render_skill_icon(skill_active_state, skill_level, skill_id, y_position, x_position, opacity);
}
