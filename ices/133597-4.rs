fn f() -> impl FnOnce<()> { || () }
fn main() { () = f(); }
