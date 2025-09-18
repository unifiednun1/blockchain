//! WASM contract integration for UnifiedNUN backend
use wasmtime::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct MintResult {
    pub user_amount: u64,
    pub platform_amount: u64,
    pub total: u64,
}

pub fn call_mint_nun(wasm_path: &str, total: u64) -> anyhow::Result<MintResult> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, wasm_path)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let mint_nun = instance.get_typed_func::<i32, i32>(&mut store, "mint_nun")?;
    // For demo: pass total as i32, get result as i32 (pointer to JSON)
    // In real: use interface types or serialization
    let _result_ptr = mint_nun.call(&mut store, total as i32)?;
    // TODO: Read result from WASM memory, parse JSON to MintResult
    // For now, return dummy result
    Ok(MintResult { user_amount: total/2, platform_amount: total-total/2, total })
}
