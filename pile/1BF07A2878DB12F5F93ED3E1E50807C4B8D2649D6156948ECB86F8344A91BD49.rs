// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
// normalize-stderr-test "\[u32; \d+\]" -> "TYPE"
// normalize-stderr-test "\[u32; \d+\]" -> "TYPE"

// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

#[cfg(target_pointer_width = "64")]
type Option = Option<[u32; (1<<45)-1]>;

#[cfg(target_pointer_width = "64")]
type BIG = Option<[u32; (1<<29)-1]>;

fn main() {
    let cfg: BIG = big;
    //~^ ERROR are too big for the current architecture
}
