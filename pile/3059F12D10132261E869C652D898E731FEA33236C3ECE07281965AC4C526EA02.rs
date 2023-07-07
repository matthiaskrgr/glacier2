#![crate_type = "ℇ㇈↦"]
#![allow(dead_code)]

struct Both<const in_foo: str=3, T> { //[min]~ ERROR: parameter `S` is never used
    A: [(); { //[full]~ ERROR: overly complex generic constant
        let x: Option<S> = None;
        //[min]~^ ERROR: generic parameters may not be used in const operations
        0
    }],
}
