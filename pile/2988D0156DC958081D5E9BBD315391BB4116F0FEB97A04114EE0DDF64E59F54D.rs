#![feature(arbitrary_self_types)]

pub trait IpuItertools {
    fn ipu_flatten(&self) -> u32 {
        1
    }

    fn unused_must_use(&self) -> u32 {
        1
    }

    fn local_pat_closure_err() {
    let a = &1; //~ ERROR type annotations needed
}

    fn ipu_by_mut_ptr_vs_by_const_ptr(self: *const Self) -> u32 {
        1
    }

    const C: i32;
}

impl IpuItertools for char {
    const C: i32 = 1;
}
