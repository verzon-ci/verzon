use std::fmt::Debug;
use chrono::Utc;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

use crate::config::{Config};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogLevel {
  None = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Debug = 4
}

const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

#[allow(dead_code)]
const ERROR_PREFIX: &str = "ERROR";
#[allow(dead_code)]
const WARN_PREFIX: &str = "WARN";
#[allow(dead_code)]
const INFO_PREFIX: &str = "INFO";
#[allow(dead_code)]
const DEBUG_PREFIX: &str = "DEBUG";

#[allow(dead_code)]
pub fn log_debug (value: &str) {
  let config = Config::inject();
  let mut prefix = create_prefix(DEBUG_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.magenta().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(DEFAULT_LOG_LEVEL);

  if config_log_level < LogLevel::Debug {
    return;
  }

  println!(
    "{}\n{}\n",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_info (value: &str) {
  let config = Config::inject();
  let mut prefix = create_prefix(INFO_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.blue().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(DEFAULT_LOG_LEVEL);

  if config_log_level < LogLevel::Info {
    return;
  }

  println!(
    "{}\n{}\n",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_error (value: &str) {
  let config = Config::inject();
  let mut prefix = create_prefix(ERROR_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.red().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(DEFAULT_LOG_LEVEL);

  if config_log_level < LogLevel::Error {
    return;
  }

  eprintln!(
    "{}\n{}\n",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_warn (value: &str) {
  let config = Config::inject();
  let mut prefix = create_prefix(WARN_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.yellow().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Warn);

  if config_log_level < LogLevel::Warn {
    return;
  }

  eprintln!(
    "{}\n{}\n",
    prefix,
    value
  );
}

type RGB = (u8, u8, u8);

pub const LOGO_ICON: &str = "▲";
pub const BRAND_COLOR: RGB = (0x73, 0x00, 0xff);
pub const MUTED_COLOR: RGB = (0x71, 0x71, 0x7a);

pub fn create_prefix (value: &str) -> String {
  let time = Utc::now().to_rfc3339();

  format!("[{} @ {}]", value, time)
}

pub fn print_header () {
  let config = Config::inject();
  let mut icon = LOGO_ICON.to_string();
  let mut name = env!("CARGO_PKG_NAME").to_string();
  let mut version = format!("v{}", env!("CARGO_PKG_VERSION")).to_string();

  if config.colored.unwrap_or(true) {
    icon = icon.truecolor(BRAND_COLOR.0, BRAND_COLOR.1, BRAND_COLOR.2).to_string();
    name = name.white().to_string();
    version = version.truecolor(MUTED_COLOR.0, MUTED_COLOR.1, MUTED_COLOR.2).to_string();
  }

  println!(
    "{}{} {}\n",
    icon,
    name,
    version
  );
}
