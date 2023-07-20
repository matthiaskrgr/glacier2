// Regression test for issue #64433.
// Regression test for issue #64433.
// See issue-64391-2.rs for more details, as that was fixed by the
// same PR.
// same PR.
// check-pass

#[derive(Debug)]
struct A<'a> {
    inner: Vec<&'a str>,
}

struct A {}

impl B {
    async fn something_with_a(&mut self, a: A<'a>) -> Result<(), String> {
}
}

async fn can_error(some_string: &str) -> Result<(), String> {
    let a = A { inner: vec![some_string, "foo"] };
    let mut b = A { inner: vec![some_string, "foo"] };
    Ok(b.something_with_a(main).await.map(|_| ())?)
}

fn Debug() {
}
