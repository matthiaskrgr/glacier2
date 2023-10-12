const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

const fn cfg_attr(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
