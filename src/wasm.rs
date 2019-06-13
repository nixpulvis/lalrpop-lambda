use wasm_bindgen::prelude::*;
use crate::{parse, Expression};

#[wasm_bindgen]
pub struct Exp(Expression);

#[wasm_bindgen]
impl Exp {
    #[wasm_bindgen(constructor)]
    pub fn new(e: &str) -> Result<Exp, JsValue> {
        let parser = parse::ExpressionParser::new();
        match parser.parse(e) {
            Ok(e) => Ok(Exp(e)),
            Err(e) => Err(JsValue::from_str(&format!("{}", e))),
        }
    }

    pub fn normalize(&self, η: bool) -> Self {
        Exp(self.0.normalize(η))
    }

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    #[wasm_bindgen(method, js_name = toNumber)]
    pub fn to_number(&self) -> usize {
        u64::from(self.0.clone()) as usize
    }

    #[wasm_bindgen(method, js_name = toBool)]
    pub fn to_bool(&self) -> bool {
        bool::from(self.0.clone())
    }
}
