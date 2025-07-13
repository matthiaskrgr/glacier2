macro_rules! foo {
    ($($($arg_type:ident),*),*) => {
        $(
            $(
                ${concat(le_, $arg_type)}
            )*
        )*
    };
}

foo!(u32);
