pub struct File<B> {
    block: B,
}

pub async fn commit<B: Clone>(this: &mut File<B>) {
    async {}.await;
    async {}.await;
    async {}.await;
    async {}.await;

    let file = async { &this }.await;
    *async { &mut this.block }.await = file.block.clone();
}

pub fn main() {}
