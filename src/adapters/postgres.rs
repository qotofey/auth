use crate::errors::AppError;

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

impl crate::app::commands::register_user::RegisterUserDao for UserRepository {
    async fn create(&self, username: String, password_digest: String) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await.unwrap();

        let user = sqlx::query_as::<_, User>("INSERT INTO users DEFAULT VALUES RETURNING id;").fetch_one(&mut *transaction).await.unwrap();

        let result = sqlx::query("INSERT INTO user_credentials (login, user_id) VALUES ($1, $2);")
                    .bind(username)
                    .bind(user.id)
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
