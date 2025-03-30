trait F = Fn() -> Self;

fn _f3<Fut>(a: dyn F<Fut>) {}
