#![feature(stmt_expr_attributes)]
#![allow(clippy::never_loop, clippy::while_immutable_condition)]

fn main() {
    #[clippy::author]
    for y in 0..10 {
        let z = y;
    }

    #[clippy::author]
    for _ in 0..10 {
        break;
    }

    #[clippy::author]
    for i in 10..5 + 4 {
        println!("{}", i);
    }

    let a = true;

    #[clippy::author]
    while a {
        break;
    }

    #[clippy::author]
    while let true = a {
        break;
    }

    #[clippy::author]
    loop {
        break;
    }
}
