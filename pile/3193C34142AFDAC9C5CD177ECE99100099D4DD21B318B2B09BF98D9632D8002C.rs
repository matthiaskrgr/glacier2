const fn foo() { (||{})() }
//~ ERROR evaluation of constant value failed

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
