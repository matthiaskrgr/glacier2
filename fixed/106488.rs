#![feature(dyn_star)]

fn main() {
    let x: dyn* Send = &();
    Box::new(x) as Box<dyn Send>;
}
