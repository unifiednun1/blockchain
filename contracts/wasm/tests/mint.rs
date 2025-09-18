use nun_contract::{mint_nun, MintResult};
use wasm_bindgen::JsValue;

#[test]
fn test_mint_nun_split() {
    let total = 10;
    let result: MintResult = mint_nun(total).into_serde().unwrap();
    assert_eq!(result.user_amount, 5);
    assert_eq!(result.platform_amount, 5);
    assert_eq!(result.total, 10);
}
