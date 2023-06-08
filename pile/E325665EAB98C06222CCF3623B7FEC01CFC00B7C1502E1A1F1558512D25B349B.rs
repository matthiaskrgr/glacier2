fn report() {
    let _: extern fn<'a: 'static>(); //~ ERROR too many `#` when terminating raw string
}
