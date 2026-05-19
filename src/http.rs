use reqwest_retry::policies::ExponentialBackoff;

use crate::package::{NAME, VERSION};

const DEFAULT_RETRIES: u32 = 3;

pub fn get_user_agent () -> String {
  format!("{}/{} (Compatible; Minimal)", NAME, VERSION)
}

pub fn get_retry_policy (retries: Option<u32>) -> ExponentialBackoff {
  ExponentialBackoff::builder().build_with_max_retries(retries.unwrap_or(DEFAULT_RETRIES))
}
