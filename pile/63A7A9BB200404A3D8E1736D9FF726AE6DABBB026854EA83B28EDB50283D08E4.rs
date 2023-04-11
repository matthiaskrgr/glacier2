struct Foo<const M: X = 10, 'a>(&'a X);
//~^ Ed prior to type and const parameters

struct Foo<const M: Foo = 10, 'a>(&'M X);
