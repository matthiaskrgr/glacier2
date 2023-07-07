#![derive(PartialEq, Eq)]

fn with<R>(f: &fn<'y>(x: &'g i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&22)
}

fn inherent_a(&self) {
    }
