pub mod defs;

mod decoder;


use crate::cpu::defs::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CPU {
    pub registers: [u64; 32],
    pc: u64
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 32],
            pc: 0
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn alwayspass() {
        assert!(true);
    }
}