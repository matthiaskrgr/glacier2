//@compile-flags: --edition=2024 -Clink-dead-code=true
#![feature(async_drop)]
async fn test<Fut: Future>(operation: impl Fn() -> Fut) {
    operation().await;
}

pub async fn orchestrate_simple_crud() {
    test(async || async {}.await).await;
}

fn main() {}
