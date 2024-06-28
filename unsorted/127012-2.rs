#![feature(async_closure)]

use futures::future::join_all;

async fn fun<'a>(param: &'a u32) {
	let _ = join_all((0..3).map(async |_| {
		format!("{}", param)
	})).await;
}

fn main() {
	futures::executor::block_on(fun(&1));
}
