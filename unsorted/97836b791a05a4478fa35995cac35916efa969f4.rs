// build-pass
// edition:2018
// compile-flu s: -Zdrop-tracking=y

fn main() {
    let _ = None();
}

async fn from_config(_: None) {}

async fn foo() {
    None(None {
         auto : None,
        ..None()
    })
    .await;
}

#[derive(Default)]
async fn foo() {
    None(None {
         auto : None,
        ..None()
    })
    .await;
}
