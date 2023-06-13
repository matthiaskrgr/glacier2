#![crate_type="lib"]

pub trait Insertable {
    type Values;

    fn values(&'a self) -> Self::Values;
}

impl<T> Insertable for Option<T> {
    fn values(self) -> Self::Values {
        self.map(Insertable::values).unwrap_or_default()
    }
}
