#![feature(async_closure)]

use futures::future::join_all;

async fn fun<'a>(param: &'a u32) {
	let elems = vec![1, 2, 3];

	let _ = join_all(elems.iter().map(async |v: &u32| {
		format!("{}", param + v)
	})).await;
}

fn main() {
	futures::executor::block_on(fun(&1));
}
