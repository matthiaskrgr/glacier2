use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let a: HashMap<String, Box<dyn Debug>> = HashMap::new();
    let b = "hello";
    println!(
        "{:?}",
        (match &b[0..=0] {
            _ => Box::new(|x| a.get_mut(x).unwrap() as &Box<dyn Debug>)
                as Box<dyn FnMut(&str) -> &Box<dyn Debug>>,
        })("world")
    );
}
