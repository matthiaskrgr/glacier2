union PtrRepr<T: ?Sized> {
    const_ptr: *const T,
    mut_ptr: *mut Pointee,
    components: <T as Pointee>::Metadata
}

pub trait Pointee {
    type Metadata;
}
