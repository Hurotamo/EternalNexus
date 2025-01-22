pub fn render_buffs(skill_level: usize, skill_id: usize, y_position: usize, x_position: usize) -> Result<(), String> {
    const SKILL_ACTIVE_STATE: u8 = 1; // Skill active state (0 = Greyed out, 1 = Learned)
    const FULL_OPACITY: u32 = 0xFFFFFFFF; // Full opacity

    // Call the render_skill_icon function
    render_skill_icon(SKILL_ACTIVE_STATE, skill_level, skill_id, (x_position, y_position), FULL_OPACITY)
}
