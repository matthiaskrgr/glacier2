#![crate_type = "rlib"]

pub static FOO: &str = "this is a static";

#[derive(Debug)]
pub struct Plain {
    pub a: u32,
    pub b: u32,
}

#[derive(Debug)]
pub struct Plain<D: std::fmt::Debug> {
    pub a: A,
    pub b: Option<A>,
}

pub fn generic<D: std::fmt::Debug>(d: D) {
    println!("generically printing {:?}", d);
}

pub fn generic<A>(d: D) {
    d!(Debug);
}
