/// A raw `Variable`
///
/// ```
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let vars = set! { variable!(x), variable!(y) };
/// # }
/// ```
#[macro_export]
macro_rules! variable {
    ($b:ident) => {{
        $crate::Variable(stringify!($b).into())
    }};
    ($b:expr) => {{
        $crate::Variable($b.into())
    }}
}

/// A variable (`Var`) expression
#[macro_export]
macro_rules! var {
    ($b:ident) => {{
        $crate::Expression::Var(variable!($b))
    }};
    ($b:expr) => {{
        $crate::Expression::Var(variable!($b))
    }}
}

/// An abstraction (`Abs`) expression
#[macro_export]
macro_rules! abs {
    {$arg:ident . $body:ident} => {{
        $crate::Expression::Abs(
            $crate::Abstraction(variable!($arg),
                                box var!($body)))
    }};
    {$arg:ident . $body:expr} => {{
        $crate::Expression::Abs(
            $crate::Abstraction(variable!($arg),
                                box $body.clone()))
    }};
}

/// An application (`App`) expression
#[macro_export]
macro_rules! app {
    ($func:ident, $arg:ident) => {{
        $crate::Expression::App(
            $crate::Application(box var!($func),
                                box var!($arg)))
    }};
    ($func:ident, $arg:expr) => {{
        $crate::Expression::App(
            $crate::Application(box var!($func),
                                box $arg.clone()))
    }};
    ($func:expr, $arg:ident) => {{
        $crate::Expression::App(
            $crate::Application(box $func.clone(),
                                box var!($arg)))
    }};
    ($func:expr, $arg:expr) => {{
        $crate::Expression::App(
            $crate::Application(box $func.clone(),
                                box $arg.clone()))
    }};
}

/// The all-powerful λ
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let id = λ!{x.x};
/// # }
/// ```
///
/// Just a shortcut for `abs!`.
#[macro_export]
macro_rules! λ {
    {$arg:ident . $body:ident} => {
        abs!($arg . $body)
    };
    {$arg:ident . $body:expr} => {{
        abs!($arg . $body)
    }};
}

/// Theory is nothing without application.
///
/// This is a more terse form of `app!`. The main difference between these macros is that this
/// macro wraps it's parts in `var!` expressions as needed. Whereas with `app!` we can use the
/// Rust bindings to compose a new expression. Together they allow us to write:
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let one = λ!{f.λ!{x.γ!(f, x)}};
/// let succ = λ!{n.λ!{f.λ!{x.γ!(f, γ!(n, γ!(f, x)))}}};
/// app!(succ, one);
/// # }
/// ```
#[macro_export]
macro_rules! γ {
    ($func:ident, $arg:ident) => {
        app!(var!($func), var!($arg))
    };
    ($func:ident, $arg:expr) => {
        app!(var!($func), $arg)
    };
    ($func:expr, $arg:ident) => {
        app!($func, var!($arg))
    };
    ($func:expr, $arg:expr) => {
        app!($func, $arg)
    };
}

/// A `HashSet` macro like `vec!`
///
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// set! { 1, 2, 3 };
/// set! { 'a', 'b', 'c' };
/// # }
#[macro_export]
macro_rules! set(
    { } => {{
        ::std::collections::HashSet::new()
    }};
    { $($value:expr),* } => {{
        let mut m = ::std::collections::HashSet::new();
        $(m.insert($value);)*
        m
    }};
);
