use crate::ETERNAL_NEXUS;
pub mod types;

/// A trait designed to add support for logging various types to the error file.
pub trait ETERNAL_NEXUSLogger {
    /// Logs a type as an error.
    fn log_error(&self);
}

impl<T> ETERNAL_NEXUSLogger for anyhow::Result<T> {
    /// Logs a failed result as an error.
    fn log_error(&self) {
        if let Err(e) = &self {
            let mut eternal_nexus = ETERNAL_NEXUS.lock().unwrap();
            eternal_nexus.log(format!("ERROR! {}", e.to_string()));
        }
    }
}
