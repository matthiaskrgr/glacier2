use core::cell::RefCell;

pub struct SpinChar {
    current: RefCell<char>,
}

impl SpinChar {
    pub fn new() -> Self {
        SpinChar {
            current: RefCell::new('-'),
        }
    }

    pub fn next(&self) -> char {
        let current = self.current.try_borrow_mut();
        match current {
            Ok(mut c) => {
                *c = match *c {
                    '-' => '\\',
                    '\\' => '|',
                    '|' => '/',
                    _ => '-',
                }
                *c
            }
            Err(_) => '_',
        }
    }
}
