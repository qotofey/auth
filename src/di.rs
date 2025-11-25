use crate::{
    app::commands::hash_password::CreatePasswordHash,
    // config::Config,
    providers::HashFuncProvider,
};

pub struct Container<H>
where
    H: HashFuncProvider,
{
    pub create_password_hash_command: CreatePasswordHash<H>, 
}

impl<H> Container<H>
where
    H: HashFuncProvider,
{
    pub fn new(hash_func_provider: H) -> Self {
        let create_password_hash_command = CreatePasswordHash::new(hash_func_provider);
        Self { 
            create_password_hash_command,
        }
    }
}

