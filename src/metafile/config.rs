use serde::{Deserialize, Serialize};

use crate::git::tracking::GitTracking;

pub const DEFAULT_METAFILE_ENABLED: bool = true;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetafileTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metafile {
  pub r#type: MetafileTypes,
  pub enabled: Option<bool>,
  pub path: String,
  pub tracking: Option<GitTracking>
}
