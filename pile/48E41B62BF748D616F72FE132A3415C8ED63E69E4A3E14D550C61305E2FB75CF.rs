fn main() {
    let mut buf = &Default::default();
    buf.iter_mut(); // issue 65419 - Attempting to run an async fn after completion mentions generators when it should
}
