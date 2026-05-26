use crate::{config::Config, log::log_debug, semver::{config::{DEFAULT_SEMVER_INCLUDE_ZERO_ITERATION, DEFAULT_SEMVER_STRATEGY, SemVerStrategy}, core::SemVer, r#type::SemVerType}, std::merge::Merge};

pub struct GetSemVerResult {
  pub semver: SemVer
}

pub fn get_semver (
  semver_type: &SemVerType,
  latest_semver: Option<SemVer>
) -> GetSemVerResult {
  let config = Config::inject();

  let strategy = config.semver.as_ref().map(|inner_semver_config| inner_semver_config.strategy.clone()).flatten().unwrap_or(DEFAULT_SEMVER_STRATEGY);
  let include_zero_iteration = config.semver.as_ref().map(|innver_semver_config| innver_semver_config.include_zero_iteration.clone()).flatten().unwrap_or(DEFAULT_SEMVER_INCLUDE_ZERO_ITERATION);
  let configured_semver = config.semver.clone().map(|v| v.to_semver_with_format()).flatten();

  log_debug(
    &format!(
      "Derived SemVer from config is:\n{:?}",
      configured_semver
    )
  );

  let is_configured_semver_pre_release = configured_semver.as_ref().is_some_and(|v| v.is_prerelease());

  log_debug(
    &format!(
      "Upcoming release is pre-release: {:?}",
      is_configured_semver_pre_release
    )
  );

  let base_semver = if let Some(inner_latest_semver) = latest_semver
    && inner_latest_semver.is_fullfilled() {
    inner_latest_semver
  } else {
    SemVer::default()
  };

  let mut semver = if is_configured_semver_pre_release {
    // Prerelease enabled, so take care of it

    let mut result_semver = base_semver;

    match strategy {
      SemVerStrategy::Iterate => {
        if result_semver.is_prerelease() {
          result_semver = result_semver.iterate();
        } else {
          result_semver = result_semver.bump(semver_type);
        }
      },
      SemVerStrategy::Increment => {
        result_semver = result_semver.bump(semver_type);
        result_semver.iteration = None;
      }
    }

    result_semver
  } else {
    // Should just be a regular bump
    if base_semver.is_prerelease() {
      base_semver.remove_pre_release()
    } else {
      base_semver.bump(semver_type).remove_pre_release()
    }
  };

  log_debug(
    &format!(
      "Bumped SemVer: {:?}",
      semver
    )
  );

  log_debug(
    &format!(
      "Should include zero iteration: {:?}",
      include_zero_iteration
    )
  );

  if include_zero_iteration {
    if is_configured_semver_pre_release && semver.iteration.is_none() {
      semver.iteration = Some(0);
      log_debug("Applied zero iteration");
    }
  }

  if let Some(inner_configured_semver) = configured_semver {
    semver = inner_configured_semver.merge(
      semver
    );
  }

  log_debug(
    &format!(
      "Calculated SemVer: {:?}",
      &semver
    )
  );

  GetSemVerResult {
    semver
  }
}
