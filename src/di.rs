use crate::{
    app::commands::{
        register_user::{RegisterUser, RegisterUserDao},
        authenticate_user::{AuthenticateUser, AuthenticateUserDao},
    },
    providers::{HashFuncProvider, HashVerifierProvider, IdProvider, TokenProvider},
};

pub struct Container<H, V, I, T, R, A>
where
    H: HashFuncProvider,
    V: HashVerifierProvider,
    I: IdProvider,
    T: TokenProvider,
    R: RegisterUserDao,
    A: AuthenticateUserDao,
{
    pub register_user_command: RegisterUser<H, R>,
    pub authenticate_user_command: AuthenticateUser<V, I, T, A>
}

impl<H, V, I, T, R, A> Container<H, V, I, T, R, A>
where
    H: HashFuncProvider,
    V: HashVerifierProvider,
    I: IdProvider,
    T: TokenProvider,
    R: RegisterUserDao,
    A: AuthenticateUserDao,
{
    pub fn new(
        hash_func_provider: H,
        hash_verifier_provider: V,
        id_provider: I,
        token_provider: T,
        register_user_dto: R, 
        authenticate_user_dto: A
    ) -> Self {
        let register_user_command = RegisterUser::new(hash_func_provider, register_user_dto);
        let authenticate_user_command = AuthenticateUser::new(hash_verifier_provider, id_provider, token_provider, authenticate_user_dto);
        
        Self { 
            register_user_command,
            authenticate_user_command
        }
    }
}

