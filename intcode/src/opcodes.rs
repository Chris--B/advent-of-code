use crate::vm::Atom;

/// Each operation the Vm can execute
///
/// r# represents argument #
/// [r#] represents the argument evalutated according to the parameter mode specified
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
    /// [r3] = [r1] + [r2]
    Add = 1,
    /// [r3] = [r1] * [r2]
    Mul = 2,

    /// [r1] = <user-input>
    In = 3,

    /// <user-output> from [r1]
    Out = 4,

    /// Stop executing the vm
    Hlt = 99,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParamMode {
    /// The parameter represents an address
    Addr,
    /// The parameter respresents a literal value and should be interpreted as-is
    Imm,
}

impl ParamMode {
    pub fn from_digit(digit: Atom) -> Option<ParamMode> {
        match digit {
            0 => Some(ParamMode::Addr),
            1 => Some(ParamMode::Imm),
            _ => None,
        }
    }
}

impl Opcode {
    pub fn from_digits(digits: Atom) -> Option<Opcode> {
        match digits {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),

            3 => Some(Opcode::In),
            4 => Some(Opcode::Out),

            99 => Some(Opcode::Hlt),
            _ => None,
        }
    }
}