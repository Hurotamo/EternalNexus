#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graphic_options() {
        assert!(load_graphic_options().is_ok());
    }

    #[test]
    fn test_prepare_cooldown_text() {
        let result = prepare_cooldown_text(123456); // 123456 milliseconds
        assert_eq!(result, "02:03"); // Expecting "02:03"
    }

    #[test]
    fn test_draw_cooldown_text() {
        // Assuming draw_text returns Result
        let result = draw_cooldown_text(100, 100, "Cooldown");
        assert!(result.is_ok());
    }

    #[test]
    fn test_flag_window_focused() {
        assert!(flag_window_focused());
    }

    #[test]
    fn test_get_window_title() {
        let title = get_window_title("abc123");
        assert_eq!(title, "ETERNAL NEXUS (commit #abc123)");
    }

    #[test]
    fn test_write_login_ip_addr() {
        assert!(write_login_ip_addr("127.0.0.1").is_ok());
    }

    #[test]
    fn test_toggle_effects_secondary() {
        let config = Config::default(); // Assuming a default implementation exists
        assert!(toggle_effects_secondary(&config).is_ok());
    }

    #[test]
    fn test_startup() {
        assert!(startup().is_ok());
    }

    #[test]
    fn test_reset_window_title() {
        assert!(reset_window_title("abc123").is_ok());
    }

    #[test]
    fn test_allocate_stat_points() {
        let result = allocate_stat_points(5, 3);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_render_buffs() {
        assert!(render_buffs(1, 1, 100, 100).is_ok());
    }

    #[test]
    fn test_update_window_title_char_name() {
        let title = update_window_title_char_name("John Doe", "abc123");
        assert_eq!(title, "ETERNAL NEXUS - Playing as John Doe (commit #abc123)");
    }

    #[test]
    fn test_toggle_effects_primary() {
        assert!(toggle_effects_primary().is_ok());
    }

    #[test]
    fn test_toggle_effects_secondary() {
        assert!(toggle_effects_secondary().is_ok());
    }

    #[test]
    fn test_render_primary_effects() {
        assert!(render_primary_effects().is_ok());
    }

    #[test]
    fn test_render_secondary_effects() {
        assert!(render_secondary_effects().is_ok());
    }
}




