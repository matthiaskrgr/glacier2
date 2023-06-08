fn main() { // we don't complain about the return type being `{integer}`
    let _: extern fn<'a: 'static>();
    [].len::<'static,, T> //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `::`
}

fn main() {
    let in = "foo"; //~ error: expected pattern, found keyword `in`
}
