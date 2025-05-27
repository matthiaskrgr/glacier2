#![feature(associated_const_equality)]

pub enum LowPass {}
pub trait ConfTrait {}
impl ConfTrait for LowPass {}

pub trait FirstOrderFilterConf: ConfTrait {
    const OUTPUTS: usize = 0;
}

impl FirstOrderFilterConf for LowPass {}

pub trait ButterworthFilterConf: ConfTrait {
    type Assoc;
}

impl<C, const OUTPUTS: usize> ButterworthFilterConf for C
where
    C: FirstOrderFilterConf<OUTPUTS = { OUTPUTS }>,
{
    type Assoc = Self;
}

fn make_coeffs() {
    fn make_coeffs_inner()
    where
        LowPass: ButterworthFilterConf<Assoc = LowPass>,
    {
    }

    make_coeffs_inner()
}
