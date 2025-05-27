// compile-flags -Zlint-mir
#![feature(async_drop)]
fn main() {
    async { async {}.await };
}
