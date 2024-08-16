trait Collate<const MASK: u32> {
    type Pass;
    type Fail;
}

impl<H, T, const MASK: u32> Collate<MASK> for (H, T) {
    fn collate(self) -> (Self::Pass, Self::Fail) {
        <Collate<{ 1 == MASK & 1 }> as CollateStep<H>>::collate_step(self.0)
    }
}
