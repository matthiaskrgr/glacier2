fn foo(_a: &str) {}

fn main() {
    let x = foo as fn(&'static str);
}
