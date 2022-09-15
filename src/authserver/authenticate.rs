use rocket::{http::Status, response::status::Custom, serde::json::Json};
use serde::Deserialize;

use crate::Db;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateRequest<'a> {
    username: &'a str,
    password: &'a str,
    client_token: Option<&'a str>,
    #[serde(default)]
    request_user: bool,
    agent: Option<Agent<'a>>,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Agent<'a> {
    name: &'a str,
    version: i64,
}
#[post("/authenticate", data = "<req>")]
pub async fn authenticate(db: &Db, req: Json<AuthenticateRequest<'_>>) -> Result<(), Custom<()>> {
    if let Some(agent) = &req.agent {
        if agent
            != (&Agent {
                name: "Minecraft",
                version: 1,
            })
        {
            return Err(Custom(Status::BadRequest, ()));
        }
    }
    let mut lock = db
        .acquire()
        .await
        .map_err(|_| Custom(Status::InternalServerError, ()))?;

    Ok(())
}
