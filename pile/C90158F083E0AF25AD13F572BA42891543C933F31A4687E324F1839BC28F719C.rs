struct Bug<A = [(); (let let = (), 1).1]> {
    //~| expected expression, found `let` statement
    //~^ `let` expressions are not supported here
    //~^ `let` expressions are not supported here
    a: Bug
}

fn let() {}
