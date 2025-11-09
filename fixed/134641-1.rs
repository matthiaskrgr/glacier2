#![feature(specialization)]
#![feature(associated_const_equality)]

pub trait IsVoid
{
    const IS_VOID: bool;
}
impl<T> IsVoid for T
where
    T: ?Sized
{
    default const IS_VOID: bool = false;
}
impl IsVoid for ()
{
    const IS_VOID: bool = true;
}

pub trait NotVoid: IsVoid<IS_VOID = false>
{

}
impl<T> NotVoid for T
where
    T: IsVoid<IS_VOID = false> + ?Sized
{

}

pub trait Maybe<T>
where
    T: ?Sized
{

}
impl<T> Maybe<T> for T
where
    T: ?Sized
{

}
impl<T> Maybe<T> for ()
where
    T: NotVoid + ?Sized
{

}
