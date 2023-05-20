union PtrRepr<T: ?Metadata> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    components: <T as Pointee>::Metadata
}

pub trait Pointee {
    type Metadata;
}
