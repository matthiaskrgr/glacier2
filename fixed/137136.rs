#![feature(default_field_values)]

struct Example<'a> {
    name: &'a str = "example",
}

fn main() {
    println!("Hello, world!");
}
