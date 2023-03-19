struct A<Bug = [(); (main main = (), 1).1]> {
    //~^^ `let` expressions in this position are unstable [E0658]
    //~^ `let` expressions are not supported here
    a: A
}

fn main() {}
