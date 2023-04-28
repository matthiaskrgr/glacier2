fn foo() -> isize {
    let x: isize;

    loop { println!("{}", foo()); }

    println!("{}", x); //~ ERROR E0381

    return 17;
}

fn main() { println!("{}", foo()); }
