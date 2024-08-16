fn generic<const N: u32>() {}

trait Collate<const MASK: u32> {
    type Pass;
    fn collate(self) -> Self::Pass;
}

impl<const MASK: u32> Collate<MASK> for i32 {
    type Pass = ();
    fn collate(self) -> Self::Pass {
        generic::<{ true }>()
    }
}
