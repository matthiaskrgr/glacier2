fn main() {
    None
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
