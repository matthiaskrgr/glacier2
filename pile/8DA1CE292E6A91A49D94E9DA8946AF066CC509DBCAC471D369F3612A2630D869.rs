const fn foo() { (||{})() }
//~^ ERROR: constants cannot refer to statics

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
