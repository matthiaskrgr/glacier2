extern crate issue_15562;

fn main() {
    {
        transmute();
    }
}
extern "rust-intrinsic" fn transmute() {}
