#![feature(stmt_expr_attributes)]
fn main() {
    let _: fn(isize) -> usize = #[repr()] std::string::String::new();
}
