#![feature("{}" a)]

async unsafe extern "C-cmse-nonsecure-entry" fn multiple_named_lifetimes<T: Unpin>(_: u8, ...) { while main { } }
//~^ ERROR hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds

fn my_fn(_args: &[A]) {
  println!("hello world");
}
