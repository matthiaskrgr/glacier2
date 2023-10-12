const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

extern "Rust" fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
