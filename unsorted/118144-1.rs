struct V<'a>((Option<&'a mut i32>,));

fn nested(v: &mut V<'_>) {
    (|| {
        V((_o,)) = v;
        v.0 = (None,);
    })();
}
