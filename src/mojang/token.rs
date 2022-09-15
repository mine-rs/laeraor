use anyhow::bail;
use reqwest::StatusCode;
use serde_json::json;


pub enum AccessToken {
    Microsoft(String),
    Yggdrasil(String),
}


impl AccessToken {
    /// Returns true if the account actually owns minecraft
    pub async fn verify(&self, http: reqwest::Client) -> anyhow::Result<bool> {
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
                        bail!(v)
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
                        bail!(v)
                    }
                }
            }
        }
    }
}
