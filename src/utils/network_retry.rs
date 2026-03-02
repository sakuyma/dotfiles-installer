use crate::cli::formatter::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Delay strategy between retry attempts
pub enum RetryStrategy {
    /// Fixed delay in seconds
    Fixed(u64),
    /// Exponential delay: 1s, 2s, 4s, 8s... (initial delay)
    Exponential(u64),
    /// Linear delay: 1s, 2s, 3s, 4s... (step)
    Linear(u64),
}

/// Retry configuration
pub struct RetryConfig {
    /// Maximum number of attempts
    pub max_attempts: u32,
    /// Delay strategy
    pub strategy: RetryStrategy,
    /// Keywords in error messages that should trigger a retry
    pub retryable_keywords: Vec<&'static str>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            strategy: RetryStrategy::Exponential(1),
            retryable_keywords: vec![
                "Connection refused",
                "Network is unreachable",
                "Timeout",
                "reset by peer",
                "Could not resolve",
                "Temporary failure",
                "Network error",
            ],
        }
    }
}

/// Execute an operation with retries on network errors
pub fn with_retry<F, T>(
    mut operation: F,
    config: &RetryConfig,
    operation_name: &str,
) -> Result<T, String>
where
    F: FnMut() -> Result<T, String>,
{
    let mut last_error = None;

    for attempt in 1..=config.max_attempts {
        match operation() {
            Ok(result) => {
                if attempt > 1 {
                    print_success(&format!(
                        "{} succeeded after {} attempts",
                        operation_name, attempt
                    ));
                }
                return Ok(result);
            }
            Err(e) => {
                let error_msg = e.to_lowercase();
                let should_retry = config
                    .retryable_keywords
                    .iter()
                    .any(|&keyword| error_msg.contains(&keyword.to_lowercase()));

                if !should_retry {
                    print_error(&format!("Non-retryable error: {}", e));
                    return Err(e);
                }

                if attempt == config.max_attempts {
                    print_error(&format!(
                        "{} failed after {} attempts: {}",
                        operation_name, config.max_attempts, e
                    ));
                    return Err(e);
                }

                print_warning(&format!(
                    "{} failed (attempt {}/{}): {}",
                    operation_name, attempt, config.max_attempts, e
                ));

                // Calculate delay based on strategy
                let delay_secs = match config.strategy {
                    RetryStrategy::Fixed(delay) => delay,
                    RetryStrategy::Exponential(initial) => initial * 2u64.pow(attempt - 1),
                    RetryStrategy::Linear(step) => step * attempt as u64,
                };

                print_progress(&format!("Waiting {} seconds before retry...", delay_secs));

                thread::sleep(Duration::from_secs(delay_secs));
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error".to_string()))
}

/// Check internet connectivity by pinging a reliable host
pub fn check_internet_connection(host: &str, timeout_secs: u64) -> bool {
    let status = Command::new("ping")
        .args(["-c", "1", "-W", &timeout_secs.to_string(), host])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

/// Wait for internet connection with retries before proceeding with installation
pub fn wait_for_internet(host: &str, max_attempts: u32, delay_secs: u64) -> Result<(), String> {
    // Create spinner for visual feedback
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message(format!("Checking internet connection to {}...", host));

    let config = RetryConfig {
        max_attempts,
        strategy: RetryStrategy::Fixed(delay_secs),
        retryable_keywords: vec!["Network is unreachable", "Connection refused", "Timeout"],
    };

    let result = with_retry(
        || {
            if check_internet_connection(host, 5) {
                spinner.finish_with_message("✓ Internet connection is available");
                Ok(())
            } else {
                Err(format!("Cannot reach {}", host))
            }
        },
        &config,
        "Internet connection check",
    );

    result
}

/// Main function to call before installation
pub fn ensure_internet_before_install() -> Result<(), String> {
    // Try multiple hosts in case one is down
    let hosts = ["archlinux.org", "github.com", "google.com"];

    for &host in &hosts {
        match wait_for_internet(host, 2, 2) {
            Ok(()) => return Ok(()),
            Err(e) => {
                // Use println instead of print_warning to avoid interfering with spinner
                println!("⚠️  Failed to reach {}: {}", host, e);
            }
        }
    }

    Err("No internet connection available after trying multiple hosts".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_retry_success_on_second_attempt() {
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };

        let attempts = RefCell::new(0);

        let result = with_retry(
            || {
                *attempts.borrow_mut() += 1;
                if *attempts.borrow() < 2 {
                    Err("retry me".to_string())
                } else {
                    Ok(42)
                }
            },
            &config,
            "test",
        );

        assert_eq!(result, Ok(42));
        assert_eq!(*attempts.borrow(), 2);
    }

    #[test]
    fn test_retry_fails_after_max_attempts() {
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };

        let attempts = RefCell::new(0);

        let result: Result<(), String> = with_retry(
            || -> Result<(), String> {
                *attempts.borrow_mut() += 1;
                Err("retry me".to_string())
            },
            &config,
            "test",
        );

        assert!(result.is_err());
        assert_eq!(*attempts.borrow(), 3);
    }

    #[test]
    fn test_non_retryable_error() {
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };

        let attempts = RefCell::new(0);

        let result: Result<(), String> = with_retry(
            || -> Result<(), String> {
                *attempts.borrow_mut() += 1;
                Err("fatal error".to_string())
            },
            &config,
            "test",
        );

        assert!(result.is_err());
        assert_eq!(*attempts.borrow(), 1);
    }
}
