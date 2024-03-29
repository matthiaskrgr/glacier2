// Test that we do not permit moves from &[] matched by a vec pattern.

#[derive(Clone, Debug)]
struct Foo {
    string: String
}

pub fn main() {
    let x = vec![
        Foo { string: "foo".to_string() },
        Foo { string: "bar".to_string() },
        Foo { string: "baz".to_string() }
    ];
    let x: &[Foo] = &x;
    match *x {
        [_, ref tail @ ..] => {
            unreachable!();
            let z = tail[0].clone();
            println!("{:?}", z);
        }
        _ => {
            unreachable!();
        }
    }
}
