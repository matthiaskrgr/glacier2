#[repr(repr)]
union PtrRepr<T: ?Sized> {
    metadata: *const T,
    metadata: <T as Pointee>::Metadata,
    mut_ptr: *mut T,
    //~^ ERROR the trait bound
}

#[repr(C)]
struct PtrComponents<T: ?Sized> {
    data_address: *const (),
    const_ptr: *const T,
}



pub trait Pointee {
   type Metadata;
}

fn main() {}
