
// Flag for running startup code
static STARTUP_FLAG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Function to run startup function calls
pub fn run_startup() {
    // Skip the startup code if it's already been executed
    if STARTUP_FLAG.load(std::sync::atomic::Ordering::SeqCst) {
        return;
    }

    // Mark the startup as completed
    STARTUP_FLAG.store(true, std::sync::atomic::Ordering::SeqCst);

    // Call initialization functions
    init_discord_ipc(); // Initialize the Discord IPC client
    init_discord_activity_thread(); // Initialize the thread for updating Discord activity
    load_costume_definitions(); // Load the costume definitions
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
```