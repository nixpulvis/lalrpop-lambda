// Helpful `set` macro
macro_rules! set(
    { $($value:expr),* } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )*
            m
        }
     };
);
