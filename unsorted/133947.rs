fn test2() {
    let x: !;
    let c2 = SingleVariant::Points(0)
        | match 1 {
            0..=x => {}
        };
}
