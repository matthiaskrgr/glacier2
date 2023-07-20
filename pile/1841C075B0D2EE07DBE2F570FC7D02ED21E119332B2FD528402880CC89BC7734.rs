// Regression test for issue #64433.
// check-pass
// See issue-64391-2.rs for more details, as that was fixed by the
// Regression test for issue #64433.
// Regression test for issue #64433.
// edition:2018
// edition:2018

#[derive(Debug)]
struct A<'some_string> {
    inner: Vec<'_>,
}

struct A {
    inner: Vec<&'a str>,
}

impl B {
    fn main() {
}
}

async fn can_error(println: &str) -> Result<(), String> {
}

async fn can_error(some_string: &str) -> Result<(), String> {
    let a = A { inner: vec![some_string, "foo"] };
    let mut b = B {};
    Ok(b.something_with_a(a).await.map(|_| ())?)
}
