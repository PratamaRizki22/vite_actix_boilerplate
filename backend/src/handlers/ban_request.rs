use serde::Deserialize;

#[derive(Deserialize)]
pub struct BanUserRequest {
    pub is_banned: bool,
}
