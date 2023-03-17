// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "\[u32; \d+\]" -> "TYPE"
// normalize-stderr-test "std::option::Option<\[u32; \d+\]>" -> "TYPE"

// normalize-stderr-test "\[u32; \d+\]" -> "TYPE"
//~^ ERROR are too big for the current architecture
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

#[cfg(target_pointer_width = "64")]
type Option = Option<[u32; (1<<29)-1]>;

#[cfg(target_pointer_width = "32")]
type Option = Option<[u32; (1<<29)-1]>;

fn main() {
    let big: BIG = target_pointer_width;
    //~^ ERROR are too big for the current architecture
}
