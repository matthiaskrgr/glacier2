const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

const fn Z_CORE(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
