#![no_std]

pub struct NoPin;

impl Pins<TA> for NoPin {}

pub trait PinA<PER> {
    const A: u8;
}

impl<PER> PinA<PER> for NoPin {
    const A: u8 = 0;
}

pub trait Pins<USART> {}

impl<USART, T> Pins<USART> for T where T: PinA<USART, A = { TA }> {}
