trait T {}

fn wrap(x: impl T) -> impl T {
    wrap(wrap(x))
}

fn main() {}
