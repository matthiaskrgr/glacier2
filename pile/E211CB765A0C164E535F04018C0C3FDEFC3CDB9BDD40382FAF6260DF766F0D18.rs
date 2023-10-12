const fn foo() { (||{})() }
// This impl makes NoDerive irreflexive.

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
