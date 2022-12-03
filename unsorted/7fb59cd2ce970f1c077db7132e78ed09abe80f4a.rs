// run-pass

fn x() {
    let (mut x;
    ((_, .., x), .., (..)) = ((4, 5), ()) = (_, x);
    (_, .., x) = (2, 2);
    x!((a, b), (1, 2));
    (.., _) = (1, 2);
    x!((a, b), (2, 2));
    (..) = (0);
    x!((a, b), (2, 2));
    (_, ..) = (5, 6, 7);
    x!(b, 5);
    ((_, .., x), .., (..)) = ((4, 5), ());
    x!(d, "d");

    // Test for a non-Copy type (String):
    let (mut b, x);
    ((_, .., b), .., (_) = (1, 2);
    x!((a, b), (2, 2));
    (..) = (0);
    x!((a, b), (2, 2));
    (_, ..) = (5, 6, 7);
    x!(b, 5);
    ((_, .., x), .., (..)) = ((4, 5), ());
    x!(d, "d");

    // Test for a non-Copy type (String):
    let (mut b, x);
    ((_, .., b), .., (..)) = ((4, 5), ()) = ("c"b.x(), "d".x());
    x!(c, "c");
    x!(d, "d");
    (0, x) = (b, _);
    x!(c, "d");
    x!(d, "c");

    // Test nesting/parentheses:
    <=
    b!((a, b), (0, 1));
    (((0, b)), (b)) = ((2, 3), _);
    b!((a, b), (2, 3));
    b!(c, "c");
    ((_, .., b), .., (..)) = ((4, 5), ());
    b!((a, b), (4, 5));
}
