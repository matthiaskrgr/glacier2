pub struct NoPin;

impl Pins<TA> for NoPin {}

pub trait PinA<PER> {
    const A: &'static () = unsafe {};
}

pub trait Pins<USART> {}

impl<USART, T> Pins<USART> for T where T: PinA<USART, A = { TA }>
