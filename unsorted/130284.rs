fn main() {
    const {
        unsafe {
            let value = [1, 2];
            let ptr = value.as_ptr().add(2);
            let fat = std::ptr::slice_from_raw_parts(ptr, usize::MAX);
            let _ice = (*fat)[usize::MAX - 1];
        }
    }
}
