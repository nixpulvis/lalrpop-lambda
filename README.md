# λ-calculus Parser (using LALRPOP)

[![Build Status](https://travis-ci.org/nixpulvis/lalrpop-lambda.svg?branch=master)](https://travis-ci.org/nixpulvis/lalrpop-lambda)
[![Crates.io Version](https://img.shields.io/crates/v/lalrpop-lambda.svg?color=%238035b9)](https://crates.io/crates/lalrpop-lambda)
[![docs.rs](https://img.shields.io/badge/docs.rs-0.x.x-lightgrey.svg)](https://docs.rs/lalrpop-lambda/0.2.0/lalrpop_lambda)

```rust
use lalrpop_lambda::lambda::ExpressionParser;
let parser = ExpressionParser::new();

// Parse a single free variable.
let x = parser.parse("x");

// Parse the identity function.
let id = parser.parse(r"\x.x");

// f ∘ g
let compose = parser.parse(r"\f.\g.\x.(f (g x))"));

// Print the free variable in this expression.
let unbound_y = parser.parse(r"\x.x y");
println!("{}", unbound_y.free_variables());

// No need for parsing strings at all.
let id = λ!{x.x};
let one = λ!{f.λ!{x.γ!(f, x)}};

// Identity application.
let id = λ!{x.x};
println!("(id one): {} -> {}",
         app!({&id}, {&one}),
         app!({&id}, {&one}).normalize(false));

// Make the Y combinator.
let ω = parser.parse(r"λx.(x x)");
let Ω = parser.parse(r"(λx.(x x)) (λx.(x x))");
let W = parser.parse(r"λf.λx. f x x");
let Y = parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))");
```
