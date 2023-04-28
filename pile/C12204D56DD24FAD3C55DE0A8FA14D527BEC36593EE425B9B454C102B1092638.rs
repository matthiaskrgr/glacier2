fn main() {

    let x: Box<_> = 5.into();
    let y = x;

    y.clone(); //~ ERROR borrow of moved value: `x`
    y.clone();
}
