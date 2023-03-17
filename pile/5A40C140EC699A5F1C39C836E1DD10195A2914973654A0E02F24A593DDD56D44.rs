#![allow(unused_variables)]
#![allow(unused_assignments)]
#![deny(unreachable_code)]
#![deny(dead_code)]

fn foo() {
    if {return} { //~ ERROR unreachable block in `if`
        println!("Hello, world!");
    }
}

fn bar() {
    if {true} {
        return;
    }
    println!("I am not dead.");
}

fn baz() {
    if {true} {
        return;
    }
    // As the next action to be taken after the if arms, we should
    // report the `println!` as unreachable:
    println!("But I am.");
    //~^ ERROR unreachable statement
}

fn main() { }
