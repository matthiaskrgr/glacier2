// clippy, edition 2021

use std::future::Future;

async fn do_transaction<O>(f: impl FnOnce() -> TransactionFuture<'_, O>) -> TransactionResult<O> {
    f().await
}

type TransactionFuture<'__, O> = impl '__ + Future<Output = FnOnce<O>>;
