pub trait Encoder<T>: Send {}

pub trait Encode {
    type Encoder;
}

impl Encode for &str {
    type Encoder = StrEncoder;
}
pub struct StrEncoder;

pub struct HelpLink {}

pub struct HelpLinkEncoder {
    link: <&'static str as Encode>::Encoder,
}
impl Encoder<HelpLink> for HelpLinkEncoder {}

pub struct ErrorDetail {
    /* START_REMOVE */ locale: u8,
    /* END_REMOVE */
}
pub struct ErrorDetailEncoder {
    a: HelpLinkEncoder,
    b: <&'static str as Encode>::Encoder,
}
impl Encoder<ErrorDetail> for ErrorDetailEncoder {}
