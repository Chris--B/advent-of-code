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

    /// if [r1] != 0 { ip = r2 }
    JumpNonzero = 5,

    /// if [r1] == 0 { ip = r2 }
    JumpZero = 6,

    /// if [r1] < [r2] {
    ///     [r3] = 1
    /// } else {
    ///     r[3] = 0
    /// }
    LessThan = 7,

    /// if [r1] == [r2] {
    ///     [r3] = 1
    /// } else {
    ///     r[3] = 0
    /// }
    Equal = 8,

    /// Adjust Relative Base
    /// arb = arb + [r1]
    Arb = 9,

    /// Stop executing the vm
    Hlt = 99,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParamMode {
    /// The parameter represents an address
    Addr,
    /// The parameter respresents a literal value and should be interpreted as-is
    Imm,
    /// The parameter represents a value offset from the current "relative base"
    Relative,
}

impl ParamMode {
    pub fn from_digit(digit: Atom) -> Option<ParamMode> {
        match digit {
            0 => Some(ParamMode::Addr),
            1 => Some(ParamMode::Imm),
            2 => Some(ParamMode::Relative),
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
            5 => Some(Opcode::JumpNonzero),
            6 => Some(Opcode::JumpZero),
            7 => Some(Opcode::LessThan),
            8 => Some(Opcode::Equal),
            9 => Some(Opcode::Arb),

            99 => Some(Opcode::Hlt),
            _ => None,
        }
    }
}
