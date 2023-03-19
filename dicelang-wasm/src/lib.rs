use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RollResult {
    pub die: u32,
    pub result: u32,
}

// This just gets turned into a JSON object for JS to use
#[derive(Serialize, Deserialize)]
pub struct Output {
    pub result: i64,
    pub rolls: Vec<RollResult>,
}

#[wasm_bindgen]
pub fn roll(input: &str) -> Result<JsValue, JsValue> {
    let expr = dicelang::parse(input)?;
    let (result, rolls) = expr.eval();

    let converted_rolls: Vec<RollResult> = rolls
        .iter()
        .map(|dl_rr| RollResult {
            die: dl_rr.die,
            result: dl_rr.result,
        })
        .collect();
    let output = Output {
        result,
        rolls: converted_rolls,
    };

    Ok(serde_wasm_bindgen::to_value(&output)?)
}
