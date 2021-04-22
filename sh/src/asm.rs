mod deserialize;
mod serialize;

// Referene:
// http://shared-ptr.com/sh_insns.html

/// Operands are used to specify the Way Moves should
/// Operate and where their Targets and Sources are,
/// as they can be (mostly) any kombination of these
#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    /// A simple Register, so the Value will be loaded/stored
    /// directly
    Register(u8),
    /// Loads the Value from the Memory-Location specified
    /// by the Value in the given Register
    AtRegister(u8),
    // TODO
    // Good Docs to describe this Feature
    Displacement8(u8),
    /// Evaluates to the Address (disp * (1|2|4) + Rn)
    /// Disp is 4-bit zero extended
    /// Format (disp, Rn)
    Displacement4Reg(u8, u8),
    /// Loads the Value from the Address at (R0 + Rn(the
    /// given Register))
    OffsetR0(u8),
}

/// These Instructions are in the Intel Format
/// (Target, Source)
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// A simple Nop, does nothing
    Nop,
    /// Moves the Value of Register 2 into Register 1
    Mov(u8, u8),
    /// Moves the Value in the T Register into the given Register
    MovT(u8),
    /// (Target-Register, Value)
    /// Stores the Value in the Target-Register. The Value
    /// will be sign-extended and can therefore only be between
    /// -128 and +127
    MovI(u8, u8),
    /// Stores the effective Address into R0
    /// Address = (disp * 4) + (PC & 0xFFFFFFFC) + 4
    MovA(u8),
    /// Moves a Byte from the Source to the Destination
    MovB(Operand, Operand),
    /// Moves a Word(16bit) from the Source to the Destination
    MovW(Operand, Operand),
    /// Moves a Long(32bit) from the Source to the Destination
    MovL(Operand, Operand),
    /// Zero extends the Source and stores the Result in the Target
    ExtuW(u8, u8),
    /// Moves the PR-Control-Register into the
    /// given Register
    StsPr(u8),
    /// Pushes the value in the given Register on the Stack
    Push(u8),
    /// In the Format of (Source, StackRegister)
    PushOther(u8, u8),
    /// Pushes the Source-Byte on the Stack.
    /// Format (Source, StackRegister)
    PushOtherB(u8, u8),
    /// Pushes the PR-Control-Register onto the Stack
    PushPR, // STS.L
    /// The Register that contains the StackPtr
    PushPROther(u8), // STS.L,
    /// Pops the top most Element from the Stack and stores
    /// it in the given Register
    Pop(u8),
    /// In the Format of (Destination, StackRegister)
    PopOther(u8, u8),
    /// Pops the top most value from the Stack and stores it
    /// in the PR-Control-Register
    PopPR, // LDS.L
    /// The Register that contains the StackPtr
    PopPROther(u8),
    /// Performs the Logical AND operation on the two registers
    /// and checks if the result is 0 and sets the T bit to whether
    /// or not it is equal to 0
    Tst(u8, u8),
    /// XORs the given two Registers
    Xor(u8, u8),
    /// ORs the two Registers and stores the result in the
    /// Target Register
    Or(u8, u8),
    /// Adds the two Registers together
    Add(u8, u8),
    /// Adds the Value directly to the given Register.
    /// The Value will be sign-extended before it is added
    /// so it can only represent values in the Range from
    /// -128 to +127
    AddI(u8, u8),
    /// Subtracts the "Source"-/Other-Register from the
    /// Target-Register and stores the Result in the Target-
    /// Register
    Sub(u8, u8),
    /// Subtracts the Register Source- and T-Register from the
    /// Target Register and stores the Result in the Target
    /// Register and stores the borrow in the T-Register
    Subc(u8, u8),
    /// Multiplies the two Registers together and stores
    /// the resulting value in the MACL Register
    /// Rn + Rm -> MACL
    MulL(u8, u8),
    /// Performs 32-Bit multiplication of the Two-Registers
    /// and stores the 64-Bit result into MACH:MACL
    DmulSL(u8, u8),
    /// Compares R0 to the given immediate Value after
    /// sign extension of it
    CmpEqI(u8),
    /// First == Second
    CmpEq(u8, u8),
    /// First >= Second (unsigned)
    CmpHs(u8, u8),
    /// First >= Second (signed)
    CmpGe(u8, u8),
    /// First > Second (unsigned)
    CmpHi(u8, u8),
    /// First > Second (signed)
    CmpGt(u8, u8),
    /// Value >= 0 (signed)
    CmpPz(u8),
    /// Decrements the Value in the given Register and then
    /// compares the result to 0
    Dt(u8),
    /// This is not an actual Instruction, but is
    /// used to tell the Assembler where something
    /// starts
    /// The Assembler will remove this Instruction
    /// before generating the final ByteCode
    Label(String),
    /// Branches if T = 1, the previous comparison
    /// evaluted to true
    BT(u8),
    /// Branches if T = 1, the previous comparison
    /// evaluted to true
    BTs(u8),
    /// Branches if T = 0, the previous comparison
    /// evaluted to false
    BF(u8),
    /// Branches if T = 1, the previous comparison
    /// evaluted to false
    BFs(u8),
    /// Unconditional Branch
    BRA(u16),
    /// Stores PC + 4 into PR and then performs an
    /// unconditional Branch
    BSR(u16),
    /// Jumps to the Address stored in the given Register
    /// and execution will resume there
    Jmp(u8),
    /// This is not an actual Instruction, but a
    /// simplification to deal with Jumps in combination
    /// with the Label-Instruction.
    /// This Instruction will be replaced with the
    /// right combination of different Instructions,
    /// as determined by the Assembler
    JmpLabel(String),
    /// Stores the PC + 4 into PR, to inform the called
    /// code where execution should resume afterwards.
    /// Then Jumps to the Address stored in the given
    /// Register
    Jsr(u8),
    /// This Instruction acts basically just like
    /// the `JmpLabel`-Instruction, but stores the
    /// Address where execution should return to in
    /// PR
    JsrLabel(String),
    /// Returns from a Subroutine
    /// PR -> PC
    Rts,
    /// Arithmetically shifts the Content of the Register
    /// to the Right and stores the bit shifted out in
    /// the T bit
    Shar(u8),
    /// Shifts the Value in the Register by 1
    /// to the left
    Shll(u8),
    /// Shifts the Value in the Register by 2
    /// to the left
    Shll2(u8),
    /// Shifts the Value in the Register by 8
    /// to the left
    Shll8(u8),
    /// Shifts the Value in the Register by 16
    /// to the left
    Shll16(u8),
    /// Shifts the Value in the Shift-Register by the
    /// amount of bits specified in the Shift-Count-Register
    /// Format (shift_register, shift_count_register)
    Shld(u8, u8),
    /// Shifts the Value in the Register by 1
    /// to the right
    Shlr(u8),
    /// Shifts the Value in the Register by 2
    /// to the right
    Shlr2(u8),
    /// Shifts the Value in the Register by 8
    /// to the right
    Shlr8(u8),
    /// Shifts the Value in the Register by 16
    /// to the right
    Shlr16(u8),
    /// Loads the MACL Register into the given Register
    StsMacl(u8),
    /// Pushes the MACL Register onto the Stack,
    /// The given Register is used as the StackPtr (usually R15)
    StsLMacl(u8),
    /// Pops the Value from the Stack and stores the Value in
    /// the MACL Register
    /// The given Register is used as the StackPtr (usually R15)
    LdsLMacl(u8),
    /// Loads the MACH Register into the given Register
    StsMach(u8),
    /// Pushes the MACH Register onto the Stack,
    /// The given Register is used as the StackPtr (usually R15)
    StsLMach(u8),
    /// Pops the Value from the Stack and stores the Value in
    /// the MACH Register
    /// The given Register is used as the StackPtr (usually R15)
    LdsLMach(u8),
    /// Used to store some literal value or here not documented instruction
    /// This will simply be returned as is, so the user is responsible for
    /// the correctness of this instruction
    Literal(u8, u8),
}

impl Instruction {
    /// Converts the given Instruction into its appropriate
    /// ByteCode Variant that can then be run on the Calculator
    pub fn to_byte(&self) -> [u8; 2] {
        serialize::serialize(self)
    }

    /// Parses the given 16-Bit-Instruction
    pub fn parse(raw: u16) -> Self {
        deserialize::deserialize(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov() {
        // R1 -> R0
        assert_eq!([0x60, 0x13], Instruction::Mov(0, 1).to_byte());
    }
    #[test]
    fn movi() {
        // 0x12 -> R0
        assert_eq!([0xe0, 0x12], Instruction::MovI(0, 0x12).to_byte());
    }
    #[test]
    fn push() {
        // Push R0
        assert_eq!([0x2f, 0x06], Instruction::Push(0).to_byte());
    }
    #[test]
    fn pop() {
        // Pop R0
        assert_eq!([0x60, 0xf6], Instruction::Pop(0).to_byte());
    }
}
