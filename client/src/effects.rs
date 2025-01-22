pub fn toggle_effects_primary() -> Result<(), String> {
    if !is_effects_enabled() {
        return Ok(()); // Return early if effects are disabled
    }

    // Logic to render primary effects
    render_primary_effects();
    Ok(())
}

pub fn toggle_effects_secondary() -> Result<(), String> {
    if !is_effects_enabled() {
        return Ok(()); // Return early if effects are disabled
    }

    // Logic to render secondary effects
    render_secondary_effects();
    Ok(())
}

fn is_effects_enabled() -> bool {
    // Logic to check if effects are enabled
    // This could be a global state or configuration check
    true // Placeholder for actual implementation
}

fn render_primary_effects() -> Result<(), String> {
    // Implementation for rendering primary effects
}

fn render_secondary_effects() -> Result<(), String> {
    // Implementation for rendering primary effects
    // This would involve calling the appropriate graphics library functions
    Ok(())

}

fn render_secondary_effects() {
    // Implementation for rendering secondary effects
    // This would involve calling the appropriate graphics library functions
    Ok(())
}
