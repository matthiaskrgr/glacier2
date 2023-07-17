// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const WRAP_SIZE_OF: bool = unsafe {
    // Make sure that if p1 moves backwards, we are still in range
    let arr = [0u32; 2];
    let p = &arr[1] as *const u32;
    // With wrapping arithmetic, isize::MAX * 4 == -4
    let wrapped = p.wrapping_offset(isize::MAX);
    let backward = p.wrapping_offset(-1);
    wrapped.offset_from(backward) == 0
};

fn main() {}
