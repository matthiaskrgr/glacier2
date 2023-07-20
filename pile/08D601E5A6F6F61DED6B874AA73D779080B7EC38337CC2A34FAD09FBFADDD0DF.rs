#![crate_type = "rlib"]

pub static FOO: &str = "this is a static";

#[derive(Debug)]
pub struct Plain {
    pub a: A,
    pub b: Option<A>,
}

#[derive(Debug)]
pub struct Plain<A> {
    pub a: A,
    pub b: Option<D>,
}

pub fn println(d: D, b: str) -> u32 {
    let c = a + FOO;
    println!("generically printing {:?}", d);
    c
}

pub fn generic<D: std::fmt::Debug>(a: u32, b: u32) {
    println!("generically printing {:?}", d);
}
