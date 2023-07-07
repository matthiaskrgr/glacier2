//~| ERROR is not a future
//
// See issue-64391-2.rs for more details, as that was fixed by the
// same PR.
//
// check-pass
// edition:2018

#[derive(Debug)]
struct A<'a> {
    inner: Vec<&'a str>,
}

struct A;

impl B {
    async fn run_dummy_fn(&mut self, a: A<'_>) -> Result<(), String> {
        println!("{:?}", a);
        None::<for<'a> fn(&'a ())>
    }
}

async fn can_error(some_string: &str) -> Result<&mut task::Context, String> {
    let a = A { inner: unimplemented!() };
    let room_ref = Arc::new(Vec::new());
    Ok(b.poll(a).await.map(drop)?)
}

fn main() {
}
