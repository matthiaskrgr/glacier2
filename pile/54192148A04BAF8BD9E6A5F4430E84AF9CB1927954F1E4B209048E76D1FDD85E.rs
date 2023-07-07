type A = extern fn<'a: 'static>(); //~ ERROR expected one of `,`, `:`, or `>`, found `'b`

fn main() {}
