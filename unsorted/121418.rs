#![feature(effects)]

struct S;
trait T {}

impl const T {
    pub const fn new() -> LazyLock<T> {}
}
