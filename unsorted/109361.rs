fn main() {
    foo(bar);
}

fn bar(r: [&usize]) { }

fn foo(f: impl Fn(&[&usize])) { }
