use crate::{
    errors::AppError,
    providers::{
        HashVerifierProvider, 
        IdProvider, 
        TokenProvider,
    }
};

#[derive(sqlx::FromRow)]
pub struct UserPassword {
    #[sqlx(try_from = "sqlx::types::uuid::Uuid")]
    pub user_id: String,
    pub password_digest: String,
}

pub trait AuthenticateUserDao {
    async fn find_password_hash_by_login(&self, login: String) -> Result<UserPassword, AppError>;
    // async fn create_session(&self, login: String, password: String, refresh_token: String) -> Result<(), AppError>;
}

pub struct AuthenticateUser<V, I, T, A>
where
    V: HashVerifierProvider,
    I: IdProvider,
    T: TokenProvider,
    A: AuthenticateUserDao,
{
    hash_verifier_provider: V,
    refresh_token_generator: I,
    access_token_provider: T,
    repo: A,
}

impl<V, I, T, A> AuthenticateUser<V, I, T, A>
where
    V: HashVerifierProvider,
    I: IdProvider,
    T: TokenProvider,
    A: AuthenticateUserDao,
{
    pub fn new(hash_verifier_provider: V, refresh_token_generator: I, access_token_provider: T, repo: A) -> Self {
        Self {
            hash_verifier_provider,
            refresh_token_generator,
            access_token_provider,
            repo,
        }
    }

    // TODO:
    pub async fn call(&self, login: String, password: String) -> Result<(String, String), AppError> {
        // переделать на структуру с полями user_id, password_digest и поместить её в Option
        let record: UserPassword = self.repo.find_password_hash_by_login(login).await.unwrap();
        if !self.hash_verifier_provider.provide(password, record.password_digest) {
            return Err(AppError::LoginError) 
        };
        let refresh_token = self.refresh_token_generator.provide().unwrap();
        let access_token = self.access_token_provider.provide(record.user_id).unwrap();
        
        Ok((refresh_token, access_token))
     }
}
