use backoff::{ExponentialBackoff, backoff::Backoff};
use std::time::Duration;
use std::process::Command;
use std::thread;

pub async fn with_retry_async<F, Fut, T, E>(
    mut operation: F,
    max_elapsed_time: Option<Duration>,
    operation_name: &str,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut backoff = ExponentialBackoff {
        initial_interval: Duration::from_secs(1),
        max_interval: Duration::from_secs(60),
        multiplier: 2.0,
        max_elapsed_time,
        ..Default::default()
    };
    
    let mut attempt = 1;
    
    loop {
        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    println!("✅ {} succeeded after {} attempts", operation_name, attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if let Some(duration) = backoff.next_backoff() {
                    println!(
                        "⚠️  {} failed (attempt {}): {}",
                        operation_name, attempt, e
                    );
                    println!("⏳ Retrying in {:?}...", duration);
                    thread::sleep(duration);
                    attempt += 1;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

// Синхронная версия
pub fn with_retry_sync<F, T, E>(
    mut operation: F,
    max_attempts: u32,
    operation_name: &str,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut attempt = 1;
    let mut delay = Duration::from_secs(1);
    
    loop {
        match operation() {
            Ok(result) => {
                if attempt > 1 {
                    println!("✅ {} succeeded after {} attempts", operation_name, attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if attempt >= max_attempts {
                    return Err(e);
                }
                
                println!(
                    "⚠️  {} failed (attempt {}/{}): {}",
                    operation_name, attempt, max_attempts, e
                );
                println!("⏳ Retrying in {} sec...", delay.as_secs());
                thread::sleep(delay);
                
                delay *= 2; // Exponential backoff
                attempt += 1;
            }
        }
    }
}
