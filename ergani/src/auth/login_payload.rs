#[derive(bon::Builder, Clone)]
pub struct LoginPayload {
    username: String,
    password: String,
}

impl LoginPayload {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
