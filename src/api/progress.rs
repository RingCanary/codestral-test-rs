use crate::error::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// A wrapper for API requests that displays a progress spinner
pub struct ProgressTracker {
    spinner: ProgressBar,
}

impl ProgressTracker {
    /// Create a new progress tracker with the given message
    pub fn new(message: &str) -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(Duration::from_millis(100));
        
        Self { spinner }
    }
    
    /// Update the spinner message
    pub fn update_message(&self, message: &str) {
        self.spinner.set_message(message.to_string());
    }
    
    /// Wrap an async API call with progress tracking
    pub async fn track_api_call<F, T>(&self, future: F, success_message: &str) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        // Execute the API call
        let result = future.await;
        
        // Update spinner based on result
        match &result {
            Ok(_) => self.spinner.finish_with_message(success_message.to_string()),
            Err(e) => self.spinner.finish_with_message(format!("Error: {}", e)),
        }
        
        result
    }
}

/// Helper function to track an API request with a progress spinner
pub async fn with_progress<F, T>(message: &str, success_message: &str, future: F) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    let tracker = ProgressTracker::new(message);
    tracker.track_api_call(future, success_message).await
}
