
struct Struct<'a>(&'a str);

async fn foo() -> Struct {
    todo!()
}

fn main() {
    foo();
}
