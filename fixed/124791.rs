// #![no_std] // optionally
#![crate_type = "lib"]
#![feature(min_specialization)]
#![allow(dead_code)]

trait Display {}

trait ToOwned {
    type Owned;
}

impl<T> ToOwned for T {
    type Owned = T;
}

struct Cow<B: ?Sized>(B);

impl<B: ?Sized> Display for Cow<B>
where
    B: ToOwned,
    B::Owned: Display,
{
}

impl Display for () {}

trait ToString {
    fn to_string();
}

impl<T: Display + ?Sized> ToString for T {
    default fn to_string() {}
}

impl ToString for Cow<str> {
    fn to_string() {}
}

impl ToOwned for str {
    type Owned = ();
}
