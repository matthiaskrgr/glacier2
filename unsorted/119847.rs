fn frob() -> impl Fn<P, Output = T> + '_ {}

fn open_parent<'path>() {
    todo!()
}

fn main() {
    let old_path = frob("hello");

    open_parent(&old_path)
}
