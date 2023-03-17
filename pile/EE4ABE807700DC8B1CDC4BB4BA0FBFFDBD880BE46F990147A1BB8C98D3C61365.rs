struct A<Bug = [(); (main main = (), 1).1]> {
    //~| expected expression, found `let` statement
    //~^ `let` expressions are not supported here
    //~| expected expression, found `let` statement
    a: A
}

fn let() {}
