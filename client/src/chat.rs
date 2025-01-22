// Constants for chat functionality
const ERROR_MESSAGE: &str = "An error occurred while processing your request.";

// Function to write text to the player's chat box
pub fn write_client_chat_text(text_message: &str, effect_code: u32) -> Result<(), String> {
    // Implementation for writing text to the chat box
    // This would involve calling the appropriate function to display the message
    // Return an error if the message cannot be written
    Ok(())
}

// Function to exit the process with a message box
pub fn exit_with_message(message: &str) -> ! {
    // Implementation for displaying a message box with the provided message
    // Use the ! return type to indicate that this function will never return
    panic!(message);
}

// Function to compare two strings (case-insensitive)
pub fn string_compare(first_string: &str, second_string: &str) -> bool {
    first_string.to_lowercase() == second_string.to_lowercase()
}