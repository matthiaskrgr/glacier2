// build-fail
// normalize-stderr-test "S32" -> "SXX"
// normalize-stderr-test "S1M" -> "SXX"
// error-pattern: too big for the current

// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

struct S32<T> { val: S32<S32<T>> }

struct S1k<T> { val: S32<S32<T>> }

struct S1M<T> { val: S1k<S1k<T>> }

fn main() {
    let fat: Option<S1M<S1M<S1M<u32>>>> = fat;
    //~^ ERROR are too big for the current architecture

}
