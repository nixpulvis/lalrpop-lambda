use wasm_bindgen::prelude::*;
use crate::{parse, Expression};

/// A parsed λ-expression
///
/// This struct is a wrapper around an [`Expression`] to both allow exporting it to JS, and to
/// contain functions functions we want to export for the `Exp`, for example [`Exp::to_string`].
#[wasm_bindgen]
pub struct Exp(Expression);

#[wasm_bindgen]
impl Exp {
    /// Parse and construct a new `Expr`
    ///
    /// ```js
    /// new lambda.Exp("(\\x.x x) y");
    /// new lambda.Exp(2);
    /// new lambda.Exp(false);
    /// new lambda.Exp("*wtf");  // Throws exception.
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(v: JsValue) -> Result<Exp, JsValue> {
        if let Some(s) = v.as_string() {
            let parser = parse::ExpressionParser::new();
            match parser.parse(&s) {
                Ok(e) => Ok(Exp(e)),
                Err(e) => Err(JsValue::from_str(&format!("{}", e))),
            }
        } else if let Some(n) = v.as_f64() {
            Ok(Exp(Expression::from(n as u64)))
        } else if let Some(b) = v.as_bool() {
            Ok(Exp(Expression::from(b)))
        } else {
            Err(JsValue::from_str("invalid constructor type"))
        }
    }

    /// See [`Expression::normalize`]
    ///
    /// ```js
    /// let expr = new lambda.Exp("(\\x.x x) y");
    /// let norm = new lambda.Exp("y y");
    /// console.log(`${norm} = ${expr.normalize(false)}`);
    /// ```
    pub fn normalize(&self, η: bool) -> Self {
        Exp(self.0.normalize(η))
    }

    /// See [`std::fmt::Display`]
    ///
    /// ```js
    /// let expr = new lambda.Exp("\\x.x x");
    /// console.log(`${expr}`);
    /// ```
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    /// See `From<Expression> for u64`
    ///
    /// ```js
    /// let two = new lambda.Exp("\\f.\\x.(f (f x))");
    /// console.log(`${two.toNumber()}`);
    /// ```
    #[wasm_bindgen(method, js_name = toNumber)]
    pub fn to_number(&self) -> usize {
        u64::from(self.0.clone()) as usize
    }

    /// See `From<Expression> for bool`
    ///
    ///
    /// ```js
    /// let t = new lambda.Exp("\\a.\\b.a");
    /// console.log(`${t.toBool()}`);
    /// ```
    #[wasm_bindgen(method, js_name = toBool)]
    pub fn to_bool(&self) -> bool {
        self.0.clone().into()
    }
}
