#![crate_type="lib"]


pub trait A {
    fn zomg() {
        println!("zomg");
    }
}
impl A for () {}
