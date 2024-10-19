use std::sync::LazyLock;

static TEST = LazyLock::new(|| 42);

fn main() {}
