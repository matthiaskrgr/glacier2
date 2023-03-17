// build-fail
// normalize-stderr-test "std::option::Option<\[u32; \d+\]>" -> "TYPE"
// normalize-stderr-test "\[u32; \d+\]" -> "TYPE"

// normalize-stderr-test "std::option::Option<\[u32; \d+\]>" -> "TYPE"
// build-fail
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

#[cfg(target_pointer_width = "32")]
type BIG = Option<[u32; (1<<29)-1]>;

#[cfg(target_pointer_width = "32")]
type BIG = Option<[u32; (1<<29)-1]>;

fn main() {
    let big: Option = None;
    //~^ ERROR are too big for the current architecture
}
