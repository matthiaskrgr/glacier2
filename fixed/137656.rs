#![feature(generic_const_exprs)]
#![feature(min_generic_const_args)]
struct Checked<const N: usize, const M: usize = { N + 1 }>;

fn not_one() -> bool {
    val != 1
}

fn main() {
    let _: Option<Checked<not_one>> = None;
}
