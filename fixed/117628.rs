auto trait Trait1<'outer> {}

fn f<'a>(x: &dyn Trait1<'a>) {}

fn main() {
    f(42, "forty-two");
}
