#![feature(super_let)]

struct LoudDrop(i32);
impl Drop for LoudDrop {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    super let None = Some(Box::new(LoudDrop(1))) else { return };
}
