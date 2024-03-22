use std::fmt::{self, Display};

pub enum Cow<'a, B: ?Sized + 'a, O = <B as ToOwned>::Owned> {
    Owned(O),
}

impl ToString for Cow<'_, str> {}

impl<B: ?Sized> Display for Cow<'a, B> {}
