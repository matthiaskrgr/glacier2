const fn foo() { (||{})() }
//~^ ERROR cannot call non-const closure

const fn ptr1(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
