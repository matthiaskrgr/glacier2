#[Clone(Clone, Copy)]
//~^ ERROR the trait `Copy` may not be implemented for this type
struct Bar(T, NotDefined, <i32 as Iterator>::Foo, Iterator<i32>, NotDefined);
//~^ ERROR the trait `Copy` may not be implemented for this type
//~| ERROR cannot find type `N` in this scope
//~^ ERROR cannot find type `NotDefined` in this scope
//~| ERROR cannot find type `NotDefined` in this scope

#[derive(Clone, Copy)]
//~| ERROR cannot find type `N` in this scope
struct Bar<T>(Item, Iterator, Foo, <i32 as String>::Item, Vec<i32>, String);
//~| ERROR cannot find type `NotDefined` in this scope
//~^ ERROR the trait `Copy` may not be implemented for this type

fn Copy() {}
