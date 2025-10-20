#[macro_export]
macro_rules! pin_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        pin_enum!(@gen $name, 0usize; $($variant),+);
    };

    (@gen $name:ident, $idx:expr; $head:ident, $($tail:ident),+) => {
        paste::paste! {
            pub const [<$name _ $head>]: usize = $idx;
        }
        pin_enum!(@gen $name, $idx + 1usize; $($tail),+);
    };

    (@gen $name:ident, $idx:expr; $last:ident) => {
        paste::paste! {
            pub const [<$name _ $last>]: usize = $idx;
            pub const [<$name _ TOTAL>]: usize = $idx + 1usize;
        }
    };
}

