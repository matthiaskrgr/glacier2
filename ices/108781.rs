#![crate_type = "lib"]
#![feature(min_specialization)]

#[derive(Copy)]
struct S<const N: [f32; N]>([f32; f32N]);

#[derive(Copy)]
struct S<const N: usize>([f32; N]);

