const fn foo() { (||{})() }
//~^ ERROR cannot call non-const formatting macro in constant functions

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
