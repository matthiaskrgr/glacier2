#![feature(ref_pat_eat_one_layer_2024)]
struct Foo;

fn main() {
    || if let Some(Some(&mut x)) = &mut Some(&mut Some(0)) {
        let _: u32 = x;
    };
}
