use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MintResult {
    pub user_amount: u64,
    pub platform_amount: u64,
    pub total: u64,
}

#[wasm_bindgen]
pub fn mint_nun(total: u64) -> JsValue {
    let user_amount = total / 2;
    let platform_amount = total - user_amount;
    JsValue::from_serde(&MintResult {
        user_amount,
        platform_amount,
        total,
    }).unwrap()
}

#[wasm_bindgen]
pub fn claim_nun(user: &str, amount: u64) -> bool {
    // TODO: Implement real on-chain transfer logic
    // For now, always return true (success)
    true
}
