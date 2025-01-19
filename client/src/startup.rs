// Flag for running startup code
static mut STARTUP_FLAG: bool = false;

// Function to run startup function calls
pub fn run_startup() {
    unsafe {
        // Skip the startup code if it's already been executed
        if STARTUP_FLAG {
            return;
        }

        // Mark the startup as completed
        STARTUP_FLAG = true;

        // Call initialization functions
        init_discord_ipc(); // Initialize the Discord IPC client
        init_discord_activity_thread(); // Initialize the thread for updating Discord activity
        load_costume_definitions(); // Load the costume definitions
    }
}

// Stub functions for initialization (to be implemented)
fn init_discord_ipc() {
    // Implementation for initializing Discord IPC
}

fn init_discord_activity_thread() {
    // Implementation for initializing Discord activity thread
}

fn load_costume_definitions() {
    // Implementation for loading costume definitions
}
