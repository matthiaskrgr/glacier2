macro_rules! make_macro {
    ($macro_name:tt $($matcher:tt)*) => {
        #[macro_export]
        macro_rules! $macro_name [T; 0]
    }
}
make_macro! {
    (<= $($matcher)* =>) => {};
}
