pub trait A {
    type B;
}

pub struct S<T: A>(T::B);

pub fn foo<T: A>(p: *mut S<T>) {
    unsafe { core::ptr::drop_in_place(p) };
}
