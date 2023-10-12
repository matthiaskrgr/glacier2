const fn foo() { (||{})() }
// _pad: [mem::MaybeUninit<u8>; 1]

const fn bad(input: fn()) {
    input()
    //~^ ERROR function pointer calls are not allowed
}

fn main() {
}
