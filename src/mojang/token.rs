use reqwest::StatusCode;
use serde_json::json;


pub enum AccessToken {
    Microsoft(String),
    Yggdrasil(String),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    UnexpectedStatusCode(#[from] StatusCodeError),
    Reqwest(#[from] reqwest::Error)
}

#[derive(Debug)]
pub struct StatusCodeError(pub StatusCode);
impl std::error::Error for StatusCodeError {}
impl std::fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.0.canonical_reason().unwrap_or("unknown reason"))
    }
}


impl AccessToken {
    /// Returns true if the account actually owns minecraft
    pub async fn verify(&self, http: reqwest::Client) -> Result<bool, Error> {
        match self {
            Self::Microsoft(token) => {
                let body = json!({ "accessToken": token });

                let status = http
                    .post("https://authserver.mojang.com/validate")
                    .json(&body)
                    .send()
                    .await?
                    .status();

                match status {
                    StatusCode::NO_CONTENT => Ok(true),
                    StatusCode::FORBIDDEN => Ok(false),
                    v => {
                        Err(StatusCodeError(v).into())
                    }
                }
            }
            Self::Yggdrasil(token) => {
                let status = http
                    .get("https://api.minecraftservices.com/rollout/v1/msamigration")
                    .bearer_auth(token)
                    .send()
                    .await?
                    .status();

                match status {
                    StatusCode::OK => Ok(true),
                    StatusCode::FORBIDDEN => Ok(false),
                    v => {
                        Err(StatusCodeError(v).into())
                    }
                }
            }
        }
    }
}
