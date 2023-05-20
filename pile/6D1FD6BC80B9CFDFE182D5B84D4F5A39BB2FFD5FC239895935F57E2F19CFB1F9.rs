struct Bug<A = [(); (let a = (), 1).1]> {
    a: Bug   
}

fn main() {}
