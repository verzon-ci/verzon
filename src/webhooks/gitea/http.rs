use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest_middleware::ClientWithMiddleware;

use crate::{config::{Config, ToExitCode}, http::get_user_agent, log::{log_debug, log_info}, semver::core::SemVer, std::{panic::ExpectWithStatusCode, reqwest::FromWebhookItemConfig}, webhooks::{config::WebhookItemConfig, github::remote::GitHubRemote}};

pub async fn post_create_release (
    webhook_item: &WebhookItemConfig,
    remote: &GitHubRemote,
    semver: &SemVer,
    changelog: &Option<String>
) {
  let config = Config::inject();

  let url = format!(
    "{}/api/v1/repos/{}/{}/releases",
    remote.host,
    remote.owner,
    remote.repository
  );

  let mut headers = HeaderMap::new();

  headers.insert("Accept", "application/json".parse().unwrap());
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert(
    "Authorization",
    format!(
      "token {}",
      webhook_item.get_token()
        .expect_with_status_code(
          "Could not get token",
          config.to_exit_code()
        )
    ).parse().unwrap()
  );
  headers.insert("User-Agent", get_user_agent().parse().unwrap());

  let semver_format = semver.format(
    &config.semver.as_ref()
      .map(|v| v.format.clone())
      .flatten()
  );

  let mut body = HashMap::new();
  body.insert("tag_name", semver_format.as_str());
  body.insert("name", semver_format.as_str());

  if let Some(inner_changelog) = changelog {
    body.insert("body", inner_changelog.as_str());
  }

  let client = ClientWithMiddleware::from_webhook_item_config(webhook_item);

  let response = client.post(
    url
  ).headers(headers)
    .body(
      serde_json::to_string(&body).expect_with_status_code("Failed to serialize body", config.to_exit_code())
    )
    .send()
    .await;

  match response {
    Ok(inner_response) => {
      match inner_response.text().await {
        Ok(inner_response_text) => {
          log_info(
            &format!(
              "{}\n{}",
              "Recieved an response while handling webhook",
              inner_response_text
            )
          );
        },
        Err(_) => {
          log_info(
            &format!(
              "{}",
              "Recieved an response while handling webhook"
            )
          );
        }
      }
    },
    Err(inner_response) => {
      log_debug(
        &format!(
          "{}\n{:?}",
          "Recieved an error response while handling webhook",
          inner_response
        )
      );
    }
  }
}
