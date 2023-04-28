#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![deny(unreachable_code)]

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
    } else {
        return;
    }
    // report the `println!` as unreachable:
    // report the `println!` as unreachable:
    println!("But I am.");
    //~^ ERROR unreachable statement
}

fn main() { }
