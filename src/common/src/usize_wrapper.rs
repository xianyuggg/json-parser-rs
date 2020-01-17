use std::convert::*;
use std::fmt::{Display, Error, Formatter};

// Copy is alright
#[derive(Clone, Copy)]
pub struct USIZEWrapper(pub usize);

impl Into<usize> for USIZEWrapper {
    fn into(self) -> usize {
        self.0
    }
}
impl Display for USIZEWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

// Some tricky things : when using go_ahead.trim. , we have to use assign idx = idx.trim.go
impl USIZEWrapper {
    pub fn new(u: usize) -> USIZEWrapper {
        USIZEWrapper { 0: u }
    }
    pub fn go_ahead(&mut self, bytes: &[u8]) -> USIZEWrapper {
        if self.0 == bytes.len() - 1 {
            println!("Go ahead reach bound!");
        } else {
            self.0 += 1;
        }

        return *self;
    }
    pub fn go_ahead_by_times(&mut self, bytes: &[u8], times: isize) -> USIZEWrapper {
        for _ in 0..times {
            if self.0 == bytes.len() - 1 {
                println!("Go ahead reach bound!");
            } else {
                self.0 += 1;
            }
        }
        return *self;
    }
    pub fn trim_whitespace(&mut self, bytes: &[u8]) -> USIZEWrapper {
        while self.0 != bytes.len() - 1 {
            if !(bytes[self.0].is_ascii_whitespace()) {
                return *self;
            } else {
                self.0 += 1;
            }
        }
        return *self;
    }
    pub fn is_end(&mut self, bytes: &[u8]) -> bool {
        return self.0 == bytes.len() - 1;
    }
}
