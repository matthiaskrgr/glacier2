fn test2() {
    let x: !;
    let c2 = SingleVariant::Points(_)
        | match x {
            _ => (),
        };
}

enum SingleVariant {
    Points(u32),
}
