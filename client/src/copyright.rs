pub const COPYRIGHT_X_OFFSET: usize = 78; // The horizontal offset of the copyright text.
pub const COPYRIGHT_TEXT_FORMAT: &str = "Running on commit #%s."; // The copyright footer format.

pub fn generate_copyright_text(git_hash: &str, year: u32, author: &str) -> String {
    format!("Copyright {} {}, All Rights Reserved | {}", year, author, format!(COPYRIGHT_TEXT_FORMAT, git_hash))
}

pub fn generate_copyright_logo() -> String {
    "Copyright Logo: nexon.bmp".to_string()
}

pub fn get_current_year() -> u32 {
    2025
}

pub fn get_author_name() -> String {
    "Hurotamo".to_string()
}

pub fn get_git_hash() -> String {
    "abc123".to_string()
}

pub fn generate_copyright_footer() -> String {
    let git_hash = get_git_hash();
    let year = get_current_year();
    let author = get_author_name();
    generate_copyright_text(&git_hash, year, &author)
}
