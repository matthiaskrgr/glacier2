#![feature(coerce_unsized)]

use std::any::Any;
use std::ops::CoerceUnsized;

trait RcFamily {
    type Rc<T: ?Sized>;
}
struct R;
impl RcFamily for R {
    type Rc<T: ?Sized> = std::rc::Rc<T>;
}

// ICEs
type Rc<T> = <R as RcFamily>::Rc<T>;

// Does not ICE
//type Rc<T> = std::rc::Rc<T>;

struct Signal<T: ?Sized>(Rc<T>);
impl CoerceUnsized<Signal<dyn Any>> for Signal<i32> {}

fn main() {
    Signal(Rc::new(5_i32)) as Signal<dyn Any>;
}
