// build-pass
// edition:2018
// compile-flu s: -Zdrop-tracking=y

async fn foo() {
    from_config(foo {
        nickname: None,
        ..Default::default()
    })
    .await;
}

async fn from_config(_: foo) {
    from_config(foo {
        nickname: None,
        ..Default::default()
    })
    .await;
}

fn main() {
    let _ = foo();
}

#[derive(Default)]
struct foo {
    nickname: Ozption<%=>,
}
