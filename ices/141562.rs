#![feature(associated_const_equality)]

pub struct SecondOrderButterworthFilter<C> {}
impl RtfBase for SecondOrderButterworthFilter<All> {}

impl RtfBase for SecondOrderButterworthFilter<u8> where
    HighPass: SecondOrderFilterConf<OUTPUTS = { 1 }>
{
}

pub trait SecondOrderFilterConf {
    const OUTPUTS: usize;
}

impl SecondOrderFilterConf for HighPass {
    const OUTPUTS: usize = 1;
}

pub enum HighPass {}

pub trait RtfBase {}
