#![feature(min_generic_const_args)]
trait Bar {
    fn x(&self) -> [i32; Bar::x];
}
