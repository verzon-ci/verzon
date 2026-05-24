use url::Url;

use crate::package::NAME;

#[derive(Debug)]
pub struct GitHubRemote {
  #[allow(dead_code)]
  pub url: Url,
  pub host: String,
  pub owner: String,
  pub repository: String
}

impl GitHubRemote {
  #[allow(dead_code)]
  pub fn to_origin (&self, token: &Option<String>) -> String {
    let mut url = self.url.clone();

    if self.url.scheme() != "https" {
      url.set_scheme("https").ok();
    }

    url.set_username(NAME).ok();
    url.set_password(token.as_deref()).ok();

    url.to_string()
  }
}

impl TryFrom<&str> for GitHubRemote {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut mapped_value = value;

    if let Some(stripped) = mapped_value.strip_suffix(".git") {
      mapped_value = stripped;
    }

    let url = Url::parse(mapped_value).map_err(|_| "URL could not be parsed".to_string())?;

    let mut parts = url.path_segments().ok_or("URL has no segments")?.rev();

    let repository = parts.next().ok_or("No repository found")?.to_string();
    let owner = parts.next().ok_or("No owner found")?.to_string();

    let mut host_parts = parts.collect::<Vec<&str>>();
    host_parts.reverse();
    let host_joined_parts = host_parts.join("/");
    let host_path = if host_joined_parts.is_empty() {
      ""
    } else {
      &format!("/{}", host_joined_parts)
    };

    let scheme = match url.scheme() {
      "ssh" => "http",
      other => other
    };

    let host = format!(
      "{}://{}{}",
      scheme,
      url.host().ok_or("No host found")?,
      host_path
    );

    Ok(
      Self {
        url,
        host,
        owner,
        repository
      }
    )
  }
}
