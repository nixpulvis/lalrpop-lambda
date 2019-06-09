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
                                box $body.clone().into()))
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
                                box $arg.clone().into()))
    }};
    ($func:expr, $arg:ident) => {{
        $crate::Expression::App(
            $crate::Application(box $func.clone().into(),
                                box var!($arg)))
    }};
    ($func:expr, $arg:expr) => {{
        $crate::Expression::App(
            $crate::Application(box $func.clone().into(),
                                box $arg.clone().into()))
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

/// Theory is nothing without application
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

/// A `HashSet` macro like `map!`
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
macro_rules! set {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(set!(@single $rest)),*]));

    ($($key:expr,)+) => {
        set!($($key),+)
    };

    ($($key:expr),*) => {
        {
            let _cap = set!(@count $($key),*);
            let mut _set = ::std::collections::HashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}

/// A `HashMap` macro like `vec!`
///
/// ```rust
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// map! { 'a' => 1, 'b' => 2, 'c' => 3 };
/// map! { 'a' => "a", 'b' => "b", 'c' => "c" };
/// # }
#[macro_export]
macro_rules! map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => {
        map!($($key => $value),+)
    };

    ($($key:expr => $value:expr),*) => {
        {
            let _cap = map!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
