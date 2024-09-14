// main.rs

type Value<'v> = &'v ();

trait Trait: Fn(Value) -> Value {}

impl<F: Fn(Value) -> Value> Trait for F {}

fn main() {
    let _: Box<dyn Trait> = Box::new(|v: Value| v);
}
