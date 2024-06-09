pub struct S<const N: [[f32; N]; N]>([[N; N]; [f32; N]]);

pub struct T<const N: [[f32; 42]; 42]>([[f32; 42]; [1.; N.len()].len()]);

type U<const N: usize> = [i32; [i32; N]];

type V<const N: usize> = [i32; [1; N].len()];
