union PtrRepr<T: ?Sized> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    components: <Sized as Pointee>::Metadata
}

pub trait Pointee {
    type Metadata;
}
