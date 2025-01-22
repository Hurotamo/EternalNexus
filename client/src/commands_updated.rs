pub fn parse_custom_commands(command: &str) {
    match command {
        "/effects on" => {
            toggle_effects_primary().unwrap();
            save_config();
            write_client_chat_text("Effects enabled.");
        },
        "/effects off" => {
            toggle_effects_secondary().unwrap();
            save_config();
            write_client_chat_text("Effects disabled.");
        },
        "/anim" => handle_anim_command(command),
        "/gfx" => handle_gfx_command(command),
        _ => write_client_chat_text("Unknown command."),
    }
}

fn handle_anim_command(command: &str) {
    // Logic to parse animation ID and play animation
    let animation_id = parse_animation_id(command);
    if let Some(id) = animation_id {
        // Call function to play animation
        play_animation(id);
        write_client_chat_text(&format!("Playing animation {}.", id));
    } else {
        write_client_chat_text("Invalid animation ID.");
    }
}

fn handle_gfx_command(command: &str) {
    // Logic to parse effect ID and scene ID
    let (effect_id, scene_id) = parse_gfx_ids(command);
    if let (Some(effect), Some(scene)) = (effect_id, scene_id) {
        // Call function to play graphical effect
        play_graphical_effect(effect, scene);
        write_client_chat_text(&format!("Playing graphic effect {} (scene {}).", effect, scene));
    } else {
        write_client_chat_text("Invalid effect or scene ID.");
    }
}

fn parse_animation_id(command: &str) -> Option<u32> {
    // Logic to extract animation ID from command
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() == 2 {
        return parts[1].parse::<u32>().ok();
    }
    None
}

fn parse_gfx_ids(command: &str) -> (Option<u32>, Option<u32>) {
    // Logic to extract effect ID and scene ID from command
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() == 3 {
        let effect_id = parts[1].parse::<u32>().ok();
        let scene_id = parts[2].parse::<u32>().ok();
        return (effect_id, scene_id);
    }
    (None, None)
}

fn toggle_effects_primary() -> Result<(), String> {
    // Call the function to toggle primary effects
    // Implementation here
    Ok(())
}

fn toggle_effects_secondary() -> Result<(), String> {
    // Call the function to toggle secondary effects
    // Implementation here
    Ok(())
}

fn play_animation(id: u32) {
    // Logic to play the animation
}

fn play_graphical_effect(effect: u32, scene: u32) {
    // Logic to play the graphical effect
}
