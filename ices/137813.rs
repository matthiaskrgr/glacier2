//@compile-flags: -Zincremental-verify-ich=yes -Cincremental=<dir> -Cdebuginfo=2 -Clink-dead-code=true -Zvalidate-mir --edition=2024
pub struct NoPin;

pub trait PinA<PER> {
    const A: u8;
}

impl<PER> PinA<PER> for NoPin {
    const A: u8 = 0;
}

pub trait Pins<USART> {}

impl<USART, T> Pins for T where T: PinA<USART, A = { TA }> {}

impl<USART> Serial<USART> where NoPin: Pins<USART> {}
