#![recursion_limit = "6"]
#![allow(unconditional_recursion)]

trait Project {
    type Projected<T: Project>: Project;
}
impl Project for i32 {
    type Projected<T: Project> = T;
}
impl<U: Project> Project for Option<U> {
    type Projected<T: Project> = Option<<U as Project>::Projected<T>>;
}
const X: () = foo::<i32>();
const fn foo<T: Project>() {
    foo::<<Option<T> as Project>::Projected<T>>();
}
