use std::path::Path;

use url::Url;

pub struct GiteaRemote {
    // Might be useful some day
    #[allow(dead_code)]
    pub url: Url,
    pub host: String,
    pub owner: String,
    pub repository: String,
}

impl TryFrom<&str> for GiteaRemote {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|_| "URL could not be parsed".to_string())?;

        let host = format!(
            "{}://{}",
            url.scheme(),
            url.host_str().ok_or("Expected host in URL")?
        );
        let path = Path::new(url.path());
        let mut owner = None;
        let mut repository = None;

        for component in path.components() {
          match component {
            std::path::Component::Normal(value) => {
              let value = value.to_str();

              if value.is_none() {
                return Err("Invalid character in URL path".to_string());
              }

              let value = value.unwrap().to_string();

              if owner.is_none() {
                owner = Some(value);
                continue;
              }

              let len = value.chars().count().saturating_sub(4);
              let value = value.chars().take(len).collect::<String>();
              repository = Some(value);
            },
            _ => {}
          }
        }

        Ok(
            Self {
                url,
                host,
                owner: owner.ok_or("Owner not found")?,
                repository: repository.ok_or("Repository not found")?
            }
        )
    }
}
