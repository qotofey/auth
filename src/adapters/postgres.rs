use crate::{
    errors::AppError,
    app::commands::{
        register_user::RegisterUserDao,
        authenticate_user::{AuthenticateUserDao, UserPassword},
    },
};

#[derive(Clone)]
pub struct UserRepository {
    pool: sqlx::PgPool,
}

impl UserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: sqlx::types::uuid::Uuid,
}

impl RegisterUserDao for UserRepository {
    async fn register_user(&self, login_type: String, login: String, password_digest: String) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await.unwrap();

        let user = sqlx::query_as::<_, User>("INSERT INTO users DEFAULT VALUES RETURNING id;").fetch_one(&mut *transaction).await.unwrap();

        let result = sqlx::query("INSERT INTO user_credentials (login, user_id, kind) VALUES ($1, $2, $3);")
                    .bind(login)
                    .bind(user.id)
                    .bind(login_type)
                    .execute(&mut *transaction)
                    .await;
        match result {
            Err(sqlx::Error::Database(db_err)) => {
                if let Some(pg_err) = db_err.try_downcast_ref::<sqlx::postgres::PgDatabaseError>() {
                    match pg_err.code() {
                        "23505" => Err(AppError::UsernameIsTaken),
                        _ => Err(AppError::UnknownDatabaseError),
                    }
                } else {
                    Err(AppError::UnknownDatabaseError)
                }
            },
            Err(_) => {
                Err(AppError::UnknownDatabaseError)
            },
            Ok(_) => {
                sqlx::query("INSERT INTO user_passwords (password_digest, user_id) VALUES ($1, $2);")
                    .bind(password_digest)
                    .bind(user.id)
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                transaction.commit().await.unwrap();
                Ok(())
            }
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserCredential {
    pub kind: Option<String>,
    pub login: String,
    pub user_id: sqlx::types::uuid::Uuid,
    // pub login_attempts: u32,
}

impl AuthenticateUserDao for UserRepository {
    async fn find_password_hash_by_login(&self, login: String) -> Result<UserPassword, AppError> {
        let password = sqlx::query_as::<_, UserPassword>(r#"
                SELECT 
                    user_passwords.user_id, 
                    user_passwords.password_digest 
                FROM user_credentials
                INNER JOIN user_passwords ON user_credentials.user_id = user_passwords.user_id
                WHERE user_credentials.login = $1
            "#)
            .bind(login)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            .unwrap();

        Ok(password)
    }
}
