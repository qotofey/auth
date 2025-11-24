use crate::{
    config::Config,
    providers,
}

pub struct Container<C>
where
    C: Config,
{
    pub get_config: C,
}

impl<C> Container<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }
}

