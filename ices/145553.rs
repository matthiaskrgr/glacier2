pub trait Mirror {
    type Refl: ?Sized;
}

impl<T> Mirror for T
where
    T: ?Sized,
{
    type Refl = T;
}

pub trait Trait {
    type Assoc: ?Sized;
}

impl<T, U> Trait for T
where
    T: Mirror<Refl = U> + ?Sized,
    U: ?Sized,
{
    type Assoc = U;
}

pub struct Thing<T>(<T as Trait>::Assoc)
where
    T: ?Sized;

pub fn x<T: ?Sized>(_: *const Thing<T>) {}
