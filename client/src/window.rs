pub const COPYRIGHT_X_OFFSET: usize = 78; // The horizontal offset of the copyright text.
pub const COPYRIGHT_TEXT_FORMAT: &str = "Running on commit #%s."; // The copyright footer format.

const WINDOW_TITLE_FORMAT: &str = "ETERNAL NEXUS (commit #{})";
const WINDOW_TITLE_FORMAT_CHAR_NAME: &str = "ETERNAL NEXUS - Playing as {} (commit #{})";
const WINDOW_CLASS_NAME: &str = "GAME";
const WINDOW_TITLE_SIZE: usize = 128;

// Function to get the window title
pub fn get_window_title(git_hash: &str) -> String {
    WINDOW_TITLE_FORMAT.replace("#{}", git_hash)
}

// Function to reset the window title to the default
pub fn reset_window_title(git_hash: &str) {
    let title = get_window_title(git_hash);
    set_window_text(title);
}

// Function to update the window title to include the character name
pub fn update_window_title_char_name(character_name: &str, git_hash: &str) -> String {
    WINDOW_TITLE_FORMAT_CHAR_NAME.replace("{}", character_name).replace("#{}", git_hash)
}

pub fn display_copyright_info() {
    let copyright_info = crate::copyright::generate_copyright_footer();
    println!("{}", copyright_info);
}

// Function to set the window text
pub fn set_window_text(title: String) {
    // Implementation for setting the window text
}

    // Example usage
    pub fn main() {
        // Display copyright information
        display_copyright_info();

    let git_hash = "abc123";
    let character_name = "John Doe";

    let default_title = get_window_title(git_hash);
    let title_with_character = update_window_title_char_name(character_name, git_hash);

    println!("Default title: {}", default_title);
    println!("Title with character: {}", title_with_character);

    set_window_text(default_title);
    set_window_text(title_with_character);
}
