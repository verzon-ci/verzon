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

  let mut mapped_semver_type = semver_type;
  let base_semver;

  // is latest semver valid?
  if let Some(inner_latest_semver) = latest_semver
    && inner_latest_semver.is_fullfilled() {
    base_semver = inner_latest_semver.clone();

    if inner_latest_semver.as_ref().is_prerelease()
      && configured_semver.as_ref().map(|inner_configured_semver| inner_configured_semver.is_prerelease()).unwrap_or(false) {

      if strategy == SemVerStrategy::Iterate {
        // Map semver type in case of iterate, to only increment the current iteration
        mapped_semver_type = &SemVerType::PreRelease;
      }
    }
  } else {
    base_semver = SemVer::default();
  }

  let mut semver = base_semver.bump(mapped_semver_type);

  if include_zero_iteration {
    if semver.is_prerelease() && semver.iteration.is_none() {
      semver.iteration = Some(0);
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
