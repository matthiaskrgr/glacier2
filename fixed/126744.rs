struct TestStruct {,}

fn edge_case_if() {
    let ts = TestStruct {};
    let _b = || { match ts {
        TestStruct{ x: 1, .. } | _ => (),
    }};
}
