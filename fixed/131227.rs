pub static mut G: i32 = 0;
fn myfunc() -> i32 {
    let var: *mut i32 = &raw mut G;
    if !var.is_null() {
        return 0;
    }
    return 0;
}
pub fn main() {
    myfunc();
}
