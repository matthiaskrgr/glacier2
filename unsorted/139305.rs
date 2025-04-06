fn fun(x: u32){
    fn A(x: u32, y: u32) -> u32 {
    }
    fn B(x: u32, y: u32) -> impl ToString {
        become A(x, y);
    }
}
fn main() {
}
