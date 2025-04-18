//@compile-flags: -Wrust-2021-incompatible-closure-captures
enum Either {
    One(XorShiftRng),
    Two(X),
}

struct X;

fn move_into_fnmut() {
    let x = Either::One(X);
    let y = || {
        let Either::Two(a) = x;
    };
}
