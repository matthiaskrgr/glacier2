fn foo() {
    let a = for<'a> |b: &'a ()| -> &'a () {
        const {
            let awd = ();
            let _: &'a () = &awd;
        };
        b
    };
}
