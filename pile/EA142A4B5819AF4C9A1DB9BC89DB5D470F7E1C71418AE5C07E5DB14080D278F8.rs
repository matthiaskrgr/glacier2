// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(PushOnDrop::new(1, dropped_fields.clone()),
                                            PushOnDrop::new(2, dropped_fields.clone()),
                                            PushOnDrop::new(3, dropped_fields.clone()))
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
