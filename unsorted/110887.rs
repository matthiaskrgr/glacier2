#![crate_type="lib"]
trait Bar<const N: BB> {}
trait BB = Bar<{ 2 + 1 }>;
