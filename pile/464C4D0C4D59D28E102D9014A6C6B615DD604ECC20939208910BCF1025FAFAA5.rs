fn main() {
    struct Thing(u8, [u8; 0]);
    let _: extern fn<'a: 'static>();

    for Thing(x[]) in foo {}
    //~^ ERROR: expected one of `)`, `,`, `@`, or `|`, found `[`
}

const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
