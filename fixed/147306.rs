//@compile-flags: -Zextra-const-ub-checks
const MAYBE_NULL_FN_PTR: fn() = unsafe {
    std::mem::transmute({
        fn ptr() {}
        (ptr as *const u8).wrapping_add(10)
    })
};

pub fn main() {}
