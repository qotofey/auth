use sqlx::types::uuid;
use crate::{
    errors::AppError,
    providers::{
        IdProvider, 
        TokenProvider,
    },
    app::commands::Session,
    adapters::postgres::UserCredential,
};

pub trait RefreshSessionDao {
    // add code here
    async fn refresh_session(&self, old_refresh_token: String, new_refresh_token: String) -> Result<Option<UserCredential>, sqlx::Error>;
}

pub struct RefreshSession<I, T, R>
where
    I: IdProvider,
    T: TokenProvider,
    R: RefreshSessionDao,
{
    id_provider: I,
    token_provider: T,
    repo: R,
}

#[derive(sqlx::FromRow)]
pub struct UserSession {
    pub user_credential_id: uuid::Uuid,
}

impl<I, T, R> RefreshSession<I, T, R> 
where
    I: IdProvider,
    T: TokenProvider,
    R: RefreshSessionDao
{
    pub fn new(id_provider: I, token_provider: T, repo: R) -> Self {
        Self { id_provider, token_provider, repo }
    }

    pub async fn call(&self, old_refresh_token: String) -> Result<Session, AppError> {
        let new_refresh_token = match self.id_provider.provide() {
            Some(token) => token,
            None => return Err(AppError::UnknownError),
        };
        // TODO: убрать анврапы
        let credential = self.repo.refresh_session(old_refresh_token, new_refresh_token.clone()).await.unwrap().unwrap();

        let access_token = match self.token_provider.provide(credential.user_id.to_string()) {
            Some(token) => token,
            None => return Err(AppError::UnknownError),
        };

        let refresh_token = new_refresh_token;
        Ok(Session { refresh_token, access_token })
    }
}
