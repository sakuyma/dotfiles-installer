use crate::cli::formatter::*;
use std::thread;
use std::time::Duration;
use std::process::Command;

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
                "SSL",
                "TLS",
                "Could not resolve",
                "Temporary failure",
                "Broken pipe",
                "Connection reset",
                "Network error",
                "Connection closed",
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
                let should_retry = config.retryable_keywords
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
                
                print_progress(&format!(
                    "Waiting {} seconds before retry...",
                    delay_secs
                ));
                
                thread::sleep(Duration::from_secs(delay_secs));
                last_error = Some(e);
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| "Unknown error".to_string()))
}

/// Specialized version for Command
pub fn run_command_with_retry(
    cmd: &mut Command,
    config: &RetryConfig,
    description: &str,
) -> Result<(), String> {
    with_retry(
        || {
            let output = cmd
                .output()
                .map_err(|e| format!("Failed to execute {}: {}", description, e))?;
            
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("Command failed: {}", stderr))
            }
        },
        config,
        description,
    )
}

/// Version for commands that return Output
pub fn run_command_with_retry_output(
    cmd: &mut Command,
    config: &RetryConfig,
    description: &str,
) -> Result<std::process::Output, String> {
    with_retry(
        || {
            cmd.output()
                .map_err(|e| format!("Failed to execute {}: {}", description, e))
                .and_then(|output| {
                    if output.status.success() {
                        Ok(output)
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        Err(format!("Command failed: {}", stderr))
                    }
                })
        },
        config,
        description,
    )
}

/// Check if a host is reachable (ping)
pub fn check_host_reachable(host: &str, timeout_secs: u64) -> bool {
    let status = Command::new("ping")
        .args(["-c", "1", "-W", &timeout_secs.to_string(), host])
        .status();
    
    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

/// Wait for host to become reachable with retries
pub fn wait_for_host(
    host: &str,
    max_attempts: u32,
    delay_secs: u64,
) -> Result<(), String> {
    let config = RetryConfig {
        max_attempts,
        strategy: RetryStrategy::Fixed(delay_secs),
        retryable_keywords: vec!["Network is unreachable", "Connection refused"],
    };
    
    with_retry(
        || {
            if check_host_reachable(host, 5) {
                Ok(())
            } else {
                Err(format!("Host {} is not reachable", host))
            }
        },
        &config,
        &format!("Waiting for host {}", host),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_success_on_second_attempt() {
        let mut attempts = 0;
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };
        
        let result = with_retry(
            || {
                attempts += 1;
                if attempts < 2 {
                    Err("retry me".to_string())
                } else {
                    Ok(42)
                }
            },
            &config,
            "test operation",
        );
        
        assert_eq!(result, Ok(42));
        assert_eq!(attempts, 2);
    }

    #[test]
    fn test_retry_fails_after_max_attempts() {
        let mut attempts = 0;
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };
        
        let result = with_retry(
            || {
                attempts += 1;
                Err("retry me".to_string())
            },
            &config,
            "test operation",
        );
        
        assert!(result.is_err());
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_non_retryable_error() {
        let mut attempts = 0;
        let config = RetryConfig {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(1),
            retryable_keywords: vec!["retry me"],
        };
        
        let result = with_retry(
            || {
                attempts += 1;
                Err("fatal error".to_string())
            },
            &config,
            "test operation",
        );
        
        assert!(result.is_err());
        assert_eq!(attempts, 1); // Should stop on first attempt
    }
}
