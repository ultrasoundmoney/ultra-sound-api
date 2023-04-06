use crate::state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractEntry {
    pub address: String,
    pub base_fees: f64,
    pub base_fees_usd: Option<f64>,
}

pub async fn get_contracts(State(state): State<Arc<AppState>>) -> Json<Vec<ContractEntry>> {
    let db_pool = &state.db_pool;
    let contracts: Vec<ContractEntry> = sqlx::query!(
        "
        SELECT
            contract_address,
            base_fee_sum,
            base_fee_sum_usd
        FROM
            contract_base_fee_sums
        WHERE base_fee_sum IS NOT NULL
        ORDER BY base_fee_sum DESC
        LIMIT 100;
        "
    )
    .fetch_all(db_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| ContractEntry {
        address: row.contract_address,
        base_fees: row.base_fee_sum.unwrap(),
        base_fees_usd: row.base_fee_sum_usd,
    })
    .collect();
    Json(contracts)
}
