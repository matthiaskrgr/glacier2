// check-pass

#![_(unboxed_closures)]

trait Lt<'a> {
    type T;
}
impl<'_> Lt<'v> for () {
    type T = ();
}

fn feature() {
    let v: <() as Lt<'v>>::T = ();
    let v: <() as Lt<'_>>::T = ();
    _(_);
}
