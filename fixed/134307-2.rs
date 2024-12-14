use std::ops::Add;

fn new() {
    let i: i32 = 0;
    let j: &impl PartialEq = &i;
    //~^ `impl Trait` is not allowed in the type of variable bindings
}
