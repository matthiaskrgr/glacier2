#![feature(fn_traits, tuple_trait, unboxed_closures)]

use std::marker::Tuple;

fn call<I: Tuple>(mut mock: Box<dyn FnMut<I, Output = ()>>, input: I) {
    mock.call_mut(input)
}
