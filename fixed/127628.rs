use std::io::{self, Read};

pub struct Container<'a> {
    reader: &'a mut dyn Read,
}

impl<'a> Container {
    pub fn wrap<'s>(reader: &'s mut dyn io::Read) -> Container<'s> {
        Container { reader: reader }
    }
}
