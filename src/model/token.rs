use chrono::Duration;
use once_cell::sync::Lazy;
use uuid::Uuid;

const MAX_TOKEN_AMOUNT: usize = 10;
static TEMPORARY_EXPIRATION_DURATION: Lazy<Duration> = Lazy::new(|| Duration::weeks(2));
static EXPIRATION_DURATION: Lazy<Duration> = Lazy::new(|| Duration::weeks(4));

pub struct Token {
    access_token: String,
    client_token: String,
    bound_profile: Uuid,
    issue_time: chrono::NaiveDateTime,
}

impl Token {
    fn duration(&self) -> Duration {
        chrono::Utc::now()
            .naive_utc()
            .signed_duration_since(self.issue_time)
    }
    pub fn is_temporarily_expired(&self) -> bool {
        let dur = self.duration();
        dur >= *TEMPORARY_EXPIRATION_DURATION && dur < *EXPIRATION_DURATION
    }
    pub fn is_expired(&self) -> bool {
        self.duration() >= *EXPIRATION_DURATION
    }
    pub async fn get(
        db: &mut crate::DbConn,
        access_token: String,
        // todo! maybe use the client_token
        client_token: Option<String>,
    ) -> sqlx::Result<Option<Self>> {
        get_by_access_token(db, &access_token).await
    }
    pub async fn new(
        db: &mut crate::DbConn,
        bound_profile: Uuid,
        client_token: Option<String>,
    ) -> sqlx::Result<Self> {
        let issued_tokens = get_issued(&mut *db, bound_profile).await?;
        let count = issued_tokens.len();
        if count >= MAX_TOKEN_AMOUNT {
            sqlx::query!(
                r#"
DELETE FROM tokens
WHERE issue_time <= ?
                "#,
                // todo! double check this logic
                issued_tokens[count - MAX_TOKEN_AMOUNT].issue_time
            )
            .execute(&mut *db)
            .await?;
        }
        let new = Self::_new(bound_profile, client_token);
        new._insert(db).await?;
        Ok(new)
    }
    fn _new(bound_profile: Uuid, client_token: Option<String>) -> Self {
        // todo? jwts maybe like the current session server
        Token {
            access_token: Uuid::new_v4().to_string(),
            client_token: client_token.unwrap_or_else(|| Uuid::new_v4().to_string()),
            bound_profile,
            issue_time: chrono::Utc::now().naive_utc(),
        }
    }
    async fn _insert(&self, db: &mut crate::DbConn) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
INSERT INTO tokens VALUES ($1, $2, $3, $4)
            "#,
            self.access_token,
            self.client_token,
            self.bound_profile,
            self.issue_time
        )
        .execute(&mut *db)
        .await?;
        Ok(())
    }
    pub async fn revoke(self, db: &mut crate::DbConn) -> sqlx::Result<()> {
        revoke(db, self.access_token).await
    }
}

pub async fn revoke(db: &mut crate::DbConn, access_token: String) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
DELETE FROM tokens
WHERE access_token = ?
        "#,
        access_token
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn get_issued(db: &mut crate::DbConn, bound_profile: Uuid) -> sqlx::Result<Vec<Token>> {
    sqlx::query_as!(
        Token,
        r#"
SELECT access_token, client_token, bound_profile AS "bound_profile: Uuid", issue_time
FROM tokens
WHERE bound_profile = ?
ORDER BY issue_time
        "#,
        bound_profile
    )
    .fetch_all(&mut *db)
    .await
}

async fn get_by_access_token(
    db: &mut crate::DbConn,
    access_token: &str,
) -> sqlx::Result<Option<Token>> {
    match sqlx::query_as!(
        Token,
        r#"
SELECT access_token, client_token, bound_profile AS "bound_profile: Uuid", issue_time
FROM tokens
WHERE access_token = ?
        "#,
        access_token
    )
    .fetch_one(&mut *db)
    .await
    {
        Ok(res) => Ok(Some(res)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(None),
            e => Err(e),
        },
    }
}
