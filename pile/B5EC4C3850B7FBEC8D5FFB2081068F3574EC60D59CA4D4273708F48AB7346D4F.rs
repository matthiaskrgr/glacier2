#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_assignments)]
#![allow(unused_assignments)]

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
    if {return} { //~ ERROR unreachable block in `if`
        println!("Hello, world!");
    }
}

fn unused_variables() { }
