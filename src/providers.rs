pub trait HasherProvider {
    fn provide(&self, password) -> String;
}

pub struct Argon2Proider;

impl HasherProvider for Argon2Provider {
    fn provide(&self, password: String) -> {
        let salt = "fake_salt".to_string();
        // TODO: требует реализации
        "fake_hash".to_string()
    }
}
