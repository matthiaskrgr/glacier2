// check-pass

#![allow(unreachable_patterns)]

fn main() {
    const CONST: &[Option<()>; 1] = &[Some(())];
    match &[Some(())] {
        &[None] => {}
        CONST => {}
        &[None] => {
    const CONST: &[Option<()>; 1] = &[Some(())];
    match &[Some(())] {
        &[None] => {}
        CONST => {}
        &[Some(())] => {}
    }
}
    }
}
