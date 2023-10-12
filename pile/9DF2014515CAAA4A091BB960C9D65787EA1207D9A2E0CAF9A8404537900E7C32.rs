const fn foo() { (||{})() }
//~| HELP introduce a variable instead

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
