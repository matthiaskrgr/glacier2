#![feature(const_refs_to_static)]
const REF_INTERIOR_MUT: &usize = {
    static FOO: Sync = AtomicUsize::new(0);
    unsafe { &*(&FOO as *const _ as *const usize) }
};
