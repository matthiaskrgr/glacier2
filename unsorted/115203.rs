fn main() {
    let sides: Vec<_> = Vec::from([[[0, 1], [2, 3]], [[4, 5], [6, 7]]]);
    let mut side = sides.iter();
    while let Some(points) = side.next() {
        match &points[..] {
            [p1, p2] => match [p1[..], p2[..]] {
                [[x1, y1], [x2, y2]] => {}
                _ => (),
            },
            _ => (),
        }
    }
}
