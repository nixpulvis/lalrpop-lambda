# λ-calculus Parser (using LALRPOP)

```rust
use lalrpop_lambda::lambda::ExpressionParser;
let parser = ExpressionParser::new();

let x = parser.parse(r"x");
let id = parser.parse(r"\x.x");
let unbound_y = parser.parse(r"\x.x y");
let compose = parser.parse(r"\x.\y.x y"));

// Make the Y combinator.
let ω = parser.parse(r"\x. (x x)"));
let Ω = parser.parse(r"\x. (x x) \x. (x x)")
let W = parser.parse(r"\x. f x x");
let Y = parser.parse(r"\f.(\x. f (x x)) (\x. f (x x))");
```
