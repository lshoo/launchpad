use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetResponse {
    pub asset_id: String,
    pub tick: String,
    pub name: String,
    pub total_supply: u64,
    pub data: Option<String>,
}
