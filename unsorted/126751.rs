#![feature(lint_reasons)]
macro_rules! foo {
    ($val:ident) => {
        (5_i32.overflowing_sub(3));
    };
}

fn main() {
    #[expect(semicolon_in_expressions_from_macros)]
    let _ = foo!(x);
}
