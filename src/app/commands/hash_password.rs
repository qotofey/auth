use crate::providers::HashFuncProvider;

pub struct CreatePasswordHash<H>
where
    H: HashFuncProvider,
{
    hash_func_provider: H,
}

impl<H> CreatePasswordHash<H> 
where
    H: HashFuncProvider,
{
    pub fn new(hash_func_provider: H) -> Self {
        Self { hash_func_provider }
    }

    pub async fn call(&self, password: String) -> Result<String, String> {
        let password_hash = self.hash_func_provider.provide(password);

        Ok(password_hash) } }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::Argon2Provider;

    #[tokio::test]
    async fn get_password_hash() {
        // Given
        let hash_func_provider = Argon2Provider;
        let command = CreatePasswordHash::new(hash_func_provider);

        // When
        let res = command.call("!Qwerty123".to_owned()).await;

        // Then
        assert_ne!(res, Ok("!Qwerty123".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_password_hash() {
        // Given
        let hash_func_provider = Argon2Provider;
        let command = CreatePasswordHash::new(hash_func_provider);

        // When
        let res1 = command.call("!Qwerty123".to_owned()).await;
        let res2 = command.call("!Qwerty123".to_owned()).await;

        // Then
        assert_ne!(res1, res2);
    }

    #[tokio::test]
    async fn get_params_from_password_hash() {
        // Given
        let hash_func_provider = Argon2Provider;
        let command = CreatePasswordHash::new(hash_func_provider);

        // When
        let res = command.call("!Qwerty123".to_owned()).await;
        let password_hash = res.unwrap();
        let parsed_hash = argon2::PasswordHash::new(&password_hash).unwrap();
        let parsed_params = argon2::Params::try_from(&parsed_hash).unwrap(); 

        // Then
        assert_eq!(parsed_params.m_cost(), 32768);
        assert_eq!(parsed_params.t_cost(), 2);
        assert_eq!(parsed_params.p_cost(), 1);
    }
}
