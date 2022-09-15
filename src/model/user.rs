use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    mail: String,
    // todo! change to (salted?) hash
    password: String,
    // todo! how to do properties
    // properties: Vec<UserProperty>,
}
impl User {
    pub async fn find(
        db: &mut crate::DbConn,
        username: &str,
        password: &str,
    ) -> sqlx::Result<Option<User>> {
        match sqlx::query_as!(
            User,
            r#"
SELECT id AS "id: Uuid", mail, password
FROM users
WHERE mail = $1 AND password = $2
        "#,
            username,
            password
        )
        .fetch_one(db)
        .await
        {
            Ok(res) => Ok(Some(res)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                e => Err(e),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "name")]
pub enum UserProperty {
    PreferredLanguage {
        // "en", "zh_CN", or other
        value: String,
    },
}
