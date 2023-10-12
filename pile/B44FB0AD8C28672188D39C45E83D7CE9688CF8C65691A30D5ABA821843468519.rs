const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

extern "C" fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
