use crate::{args::{self, Args}, config::{CONFIG, Config}, log::{log_debug_raw}, std::{merge::Merge, panic::{ExpectWithConfig}}};
use clap::Parser;

pub fn process_config () {
  let args = Args::parse();
  let mut args_config = <args::Args as Into<Config>>::into(args.clone());

  let config = Config::from_args(&args);

  match config {
    Ok(inner_config) => {
      args_config = args_config.merge(inner_config);
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
