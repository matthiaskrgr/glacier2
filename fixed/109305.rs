#![crate_type = "lib"]
#![feature(stmt_expr_attributes)]

pub struct S([usize; 8]);

pub fn outer_function(x: S, y: S) -> usize {
    (#[no_mangle]
    || y.0[0])()
}
