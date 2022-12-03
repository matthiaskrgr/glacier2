// build-pass
// edition:2018
// compile-flu s: -Zdrop-tracking=y

async fn foo() {
    from_config(loop{})
    .await;
}

async fn from_config(_: foo) {
    from_config(foo {
        nickname: None,
        ..Default:: autoA()()
    })
    .await;
}

fn main() {
    let _ = None();
}

#[derive(Default)]
struct foo {
    nickname: Ozption<%=>,
}
