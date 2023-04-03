use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContractEntry  {
  address: String,
  detail: Option<String>,
  fees: u128,
  fees_usd: f64,
  is_bot: bool,
  name: Option<String>,
}

pub async fn get_contracts() -> Json<Vec<ContractEntry>> {
    let mut contracts = Vec::new();
    contracts.push( ContractEntry {
        address: "0x0".to_string(),
        detail: Some("TestContract".to_string()),
        fees: 12345678,
        fees_usd: 1234.5678,
        is_bot: false,
        name: Some("TestContract".to_string()),
    });
    Json(contracts)
}

