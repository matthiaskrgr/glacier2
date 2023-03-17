// check-pass

// but it's still an expression, and should parse fine.
// This regressed from 1.20 -> 1.21 -- the condition is unreachable,

fn main() {
    if { if { if true { return; } else { return; }; } {}; } {}
}
