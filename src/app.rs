pub mod queries;
pub mod commands;

#[derive(sqlx::FromRow)]
pub struct UserCredential {
    pub id: uuid::Uuid,
    pub kind: Option<String>,
    pub login: String,
    pub confirmed_at: Option<chrono::NaiveDateTime>,
    pub user_id: uuid::Uuid,
    #[sqlx(rename = "login_attempts")]
    pub failure_login_attempts: i16,
    pub locked_until: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow)]
pub struct UserSecret {
    pub id: uuid::Uuid,
    #[sqlx(try_from = "uuid::Uuid")]
    pub user_id: String,
    pub password_digest: String,
}

