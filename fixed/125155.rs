enum NestedEnum {
    First,
    Second,
    Third
}
enum Enum {
    Variant2(Option<*mut &'a &'b ()>)
}


fn foo(x: Enum) -> isize {
    match x {
      Enum::Variant2(NestedEnum::Third) => 4,  
    }
}
