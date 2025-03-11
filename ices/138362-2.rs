#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait HasSelf {
    fn method(self);
}
trait NoSelf {
    fn method();
}
impl NoSelf for u8 {
    reuse HasSelf::method;
}
