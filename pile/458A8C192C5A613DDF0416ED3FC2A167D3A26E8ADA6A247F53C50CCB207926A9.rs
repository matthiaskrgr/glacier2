const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

const fn call_foo() -> u32 {
    foo();
    foo();
    foo();
    foo();
    foo();

    foo();
    foo();
    foo();
    foo();
    foo();

    foo();
    foo();
    foo();
    foo();
    foo();

    foo();
    foo();
    foo();
    foo(); //~ ERROR is taking a long time
    0
}

fn main() {
}
