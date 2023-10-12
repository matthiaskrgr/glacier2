const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

const fn STR_TOO_LONG(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
