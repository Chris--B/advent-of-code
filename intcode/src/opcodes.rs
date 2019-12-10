#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    /// [r3] = [r1] + [r2]
    Add = 1,
    /// [r3] = [r1] * [r2]
    Mul = 2,

    /// Stop executing the vm
    Hlt = 99,
}

#[derive(Copy, Clone, Debug)]
pub enum ParamMode {
    /// The parameter represents an address
    Addr,
    /// The parameter respresents a literal value and should be interpreted as-is
    Imm,
}

impl ParamMode {
    pub fn from_digit(digit: u8) -> Option<ParamMode> {
        match digit {
            0 => Some(ParamMode::Addr),
            1 => Some(ParamMode::Imm),
            _ => None,
        }
    }
}

impl Opcode {
    pub fn from_digits(digits: u8) -> Option<Opcode> {
        match digits {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),

            99 => Some(Opcode::Hlt),
            _ => None,
        }
    }
}
