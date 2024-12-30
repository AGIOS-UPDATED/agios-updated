use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_factor: 2.0,
        }
    }
}

pub async fn retry<T, E, Fut, F>(
    operation: F,
    config: RetryConfig,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    let mut delay = config.initial_delay_ms;

    loop {
        attempts += 1;
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if attempts >= config.max_attempts {
                    log::error!("Max retry attempts ({}) reached. Last error: {:?}", config.max_attempts, error);
                    return Err(error);
                }

                log::warn!(
                    "Attempt {}/{} failed: {:?}. Retrying in {}ms...",
                    attempts,
                    config.max_attempts,
                    error,
                    delay
                );

                sleep(Duration::from_millis(delay)).await;

                delay = (delay as f64 * config.backoff_factor) as u64;
                if delay > config.max_delay_ms {
                    delay = config.max_delay_ms;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let result = retry(
            || async { Ok::<_, String>("success") },
            RetryConfig::default(),
        )
        .await;
        
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = retry(
            || {
                let current_attempt = attempts_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if current_attempt < 2 {
                        Err("error")
                    } else {
                        Ok("success")
                    }
                }
            },
            RetryConfig::default(),
        )
        .await;

        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_attempts_reached() {
        let config = RetryConfig {
            max_attempts: 2,
            ..Default::default()
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result: Result<&str, &str> = retry(
            || {
                attempts_clone.fetch_add(1, Ordering::SeqCst);
                async { Err("error") }
            },
            config,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }
}
