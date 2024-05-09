#[derive(Debug)]
struct X<const FN: fn() = { || {} }>;
