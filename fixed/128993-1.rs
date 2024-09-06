use std::sync::LazyLock;

struct DoAnything {
    name: String
}

impl Anything for DoAnything {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

trait Anything {
    fn new(name: &str) -> Self where Self:Sized;
}

// Panics if there is no type annotations
static SOMETHING = LazyLock::new(|| DoAnything::new("David"));

// This works
// static SOMETHING: LazyLock<Anything> = LazyLock::new(|| DoAnything::new("David"));

fn main() {
    println!("Hello, world!");
}
