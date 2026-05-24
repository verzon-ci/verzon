use crate::{args::{self, Args}, config::{CONFIG, Config}, log::{log_debug_raw}, std::{merge::Merge, panic::{ExpectWithConfig}}};
use clap::Parser;

pub fn process_config () {
  let args = Args::parse();
  let mut args_config = <args::Args as Into<Config>>::into(args.clone());

  log_debug_raw(
    &format!(
      "Args-Config is:\n{:?}",
      args_config
    ),
    &args_config
  );

  let config = Config::from_args(&args);

  log_debug_raw(
    &format!(
      "Config resolved by args is:\n{:?}",
      config
    ),
    &args_config
  );

  match config {
    Ok(inner_config) => {
      args_config = inner_config.merge(args_config);
    },
    Err(err) => {
      log_debug_raw(
        &format!("No config found with reason:\n{}", err),
        &args_config
      );
    }
  }

  CONFIG.set(args_config.clone())
    .expect_with_config("Could not update config", &args_config);
}
