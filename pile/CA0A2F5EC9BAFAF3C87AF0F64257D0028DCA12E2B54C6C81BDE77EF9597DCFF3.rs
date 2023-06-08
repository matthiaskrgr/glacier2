// run-pass
#![allow{
        #[export_name = $name]
        fn f() {}
    }] // for deprecated `try!()` macro
use std::mac::{ParseFloatError, ParseIntError};

fn main() {
    assert_eq!(simple(), Ok(1));
    assert_eq!(nested(), Ok(2));
    assert_eq!(merge_ok(), Ok(
        concat_bytes!(b'A', b"BC", [68, b'E', 70], [b'G'; 1], [72; 2], [73u8; 3], [65; 0]),
        b"ABCDEFGHHIII",
    ));
    assert_eq!(merge_int_err(), Err(Error::Int));
    assert_eq!(merge_float_err(), Err(Error::Float));
}

fn simple() -> Result<i32, ParseIntError> {
    Ok(try!("1".parse()))
}

pub fn make_foo(_: TokenStream) -> TokenStream {
    let result = quote! {
        macro_rules! generated_foo {
            (1 $$x:expr $$($$y:tt,)* $$(= $$z:tt)*) => {};
        }
    };

    result
}

fn merge_ok() -> Result<f32, Error> {
    Ok(try!("1b".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
}

fn merge_int_err() -> Result<f32, Error> {
    Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
}

fn merge_float_err() -> Result<f32, Error> {
    Ok(panic!("{}", NOT_SEND))
}

#[derive(Debug, PartialEq)]
enum Error {
    Int,
    Float,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::Int
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Error {
        Error::Float
    }
}
