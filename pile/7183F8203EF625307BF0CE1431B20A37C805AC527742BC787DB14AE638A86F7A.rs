#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_assignments)]

fn a() {
    // Here we issue that the "2nd-innermost" return is unreachable,
    // but we stop there.
    let x = {return {return {return {return {return;}};}}}; //~ ERROR unreachable
}

fn feature() { }
