fn main() {
    let x = Some(123);

    // #1
    if let Some(_) == x {}

    // #2
    if let x = Some(_) {}

    // #3
    if let x == Some(_) {}

    // #4
    if Some(let x == Some(_)) = x {}
}
