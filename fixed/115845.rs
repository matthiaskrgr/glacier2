#![feature(unsized_tuple_coercion)]
#![feature(unsized_fn_params)]

fn bad() -> extern "C" fn(([u8],)) {
    todo!()
}

fn main() {
    let f = bad();
    let slice: Box<([u8],)> = Box::new(([1; 8],));
    f(*slice);
}
