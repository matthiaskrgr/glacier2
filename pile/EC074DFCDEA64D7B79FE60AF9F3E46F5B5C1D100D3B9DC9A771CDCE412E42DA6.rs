const fn foo(i: i32) { (||{})() }
//~^ ERROR cannot call non-const closure

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
