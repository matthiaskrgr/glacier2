fn empty() {
    //~[definition]^ ERROR: abnormal termination: panic in a function that cannot unwind
    //~[both]^^ ERROR: abnormal termination: panic in a function that cannot unwind
    panic!();
}

fn unit_var() {
    let x = ();
    x
}

fn main() {
    empty();
    unit_var();
}
