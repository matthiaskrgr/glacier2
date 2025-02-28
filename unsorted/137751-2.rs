//@compile-flags: -Znext-solver=globally
async fn test() {
    Box::pin(test()).await;
}

fn main() {}
