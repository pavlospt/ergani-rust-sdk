use serde_derive::Deserialize;

pub type LookupRoot = Vec<LookupResponse>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LookupResponse {
    pub id: i64,
    pub code: String,
    pub description: String,
}
