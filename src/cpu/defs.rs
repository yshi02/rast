/// Register names for the RISC-V ISA
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum REG {
    x0,
    x1,
    x2,
    x3,
    x4,
    x5,
    x6,
    x7,
    x8,
    x9,
    x10,
    x11,
    x12,
    x13,
    x14,
    x15,
    x16,
    x17,
    x18,
    x19,
    x20,
    x21,
    x22,
    x23,
    x24,
    x25,
    x26,
    x27,
    x28,
    x29,
    x30,
    x31,
}
impl REG {
    pub fn to_usize(&self) -> usize {
        match self {
            REG::x0 => 0,
            REG::x1 => 1,
            REG::x2 => 2,
            REG::x3 => 3,
            REG::x4 => 4,
            REG::x5 => 5,
            REG::x6 => 6,
            REG::x7 => 7,
            REG::x8 => 8,
            REG::x9 => 9,
            REG::x10 => 10,
            REG::x11 => 11,
            REG::x12 => 12,
            REG::x13 => 13,
            REG::x14 => 14,
            REG::x15 => 15,
            REG::x16 => 16,
            REG::x17 => 17,
            REG::x18 => 18,
            REG::x19 => 19,
            REG::x20 => 20,
            REG::x21 => 21,
            REG::x22 => 22,
            REG::x23 => 23,
            REG::x24 => 24,
            REG::x25 => 25,
            REG::x26 => 26,
            REG::x27 => 27,
            REG::x28 => 28,
            REG::x29 => 29,
            REG::x30 => 30,
            REG::x31 => 31,
        }
    }

    pub fn from_u32(n: u32) -> Option<REG> {
        match n {
            0 => Some(REG::x0),
            1 => Some(REG::x1),
            2 => Some(REG::x2),
            3 => Some(REG::x3),
            4 => Some(REG::x4),
            5 => Some(REG::x5),
            6 => Some(REG::x6),
            7 => Some(REG::x7),
            8 => Some(REG::x8),
            9 => Some(REG::x9),
            10 => Some(REG::x10),
            11 => Some(REG::x11),
            12 => Some(REG::x12),
            13 => Some(REG::x13),
            14 => Some(REG::x14),
            15 => Some(REG::x15),
            16 => Some(REG::x16),
            17 => Some(REG::x17),
            18 => Some(REG::x18),
            19 => Some(REG::x19),
            20 => Some(REG::x20),
            21 => Some(REG::x21),
            22 => Some(REG::x22),
            23 => Some(REG::x23),
            24 => Some(REG::x24),
            25 => Some(REG::x25),
            26 => Some(REG::x26),
            27 => Some(REG::x27),
            28 => Some(REG::x28),
            29 => Some(REG::x29),
            30 => Some(REG::x30),
            31 => Some(REG::x31),
            _ => None,
        }
    }

    pub fn from_u64(n: u64) -> Option<REG> {
        match n {
            0 => Some(REG::x0),
            1 => Some(REG::x1),
            2 => Some(REG::x2),
            3 => Some(REG::x3),
            4 => Some(REG::x4),
            5 => Some(REG::x5),
            6 => Some(REG::x6),
            7 => Some(REG::x7),
            8 => Some(REG::x8),
            9 => Some(REG::x9),
            10 => Some(REG::x10),
            11 => Some(REG::x11),
            12 => Some(REG::x12),
            13 => Some(REG::x13),
            14 => Some(REG::x14),
            15 => Some(REG::x15),
            16 => Some(REG::x16),
            17 => Some(REG::x17),
            18 => Some(REG::x18),
            19 => Some(REG::x19),
            20 => Some(REG::x20),
            21 => Some(REG::x21),
            22 => Some(REG::x22),
            23 => Some(REG::x23),
            24 => Some(REG::x24),
            25 => Some(REG::x25),
            26 => Some(REG::x26),
            27 => Some(REG::x27),
            28 => Some(REG::x28),
            29 => Some(REG::x29),
            30 => Some(REG::x30),
            31 => Some(REG::x31),
            _ => None,
        }
    }
}

/// Register ABI names for the RISC-V ISA
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ABI {
    zero,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
}
impl ABI {
    pub fn to_usize(&self) -> usize {
        match self {
            ABI::zero => 0,
            ABI::ra => 1,
            ABI::sp => 2,
            ABI::gp => 3,
            ABI::tp => 4,
            ABI::t0 => 5,
            ABI::t1 => 6,
            ABI::t2 => 7,
            ABI::s0 => 8,
            ABI::s1 => 9,
            ABI::a0 => 10,
            ABI::a1 => 11,
            ABI::a2 => 12,
            ABI::a3 => 13,
            ABI::a4 => 14,
            ABI::a5 => 15,
            ABI::a6 => 16,
            ABI::a7 => 17,
            ABI::s2 => 18,
            ABI::s3 => 19,
            ABI::s4 => 20,
            ABI::s5 => 21,
            ABI::s6 => 22,
            ABI::s7 => 23,
            ABI::s8 => 24,
            ABI::s9 => 25,
            ABI::s10 => 26,
            ABI::s11 => 27,
            ABI::t3 => 28,
            ABI::t4 => 29,
            ABI::t5 => 30,
            ABI::t6 => 31,
        }
    }

    pub fn from_u64(n: u64) -> Option<ABI> {
        match n {
            0 => Some(ABI::zero),
            1 => Some(ABI::ra),
            2 => Some(ABI::sp),
            3 => Some(ABI::gp),
            4 => Some(ABI::tp),
            5 => Some(ABI::t0),
            6 => Some(ABI::t1),
            7 => Some(ABI::t2),
            8 => Some(ABI::s0),
            9 => Some(ABI::s1),
            10 => Some(ABI::a0),
            11 => Some(ABI::a1),
            12 => Some(ABI::a2),
            13 => Some(ABI::a3),
            14 => Some(ABI::a4),
            15 => Some(ABI::a5),
            16 => Some(ABI::a6),
            17 => Some(ABI::a7),
            18 => Some(ABI::s2),
            19 => Some(ABI::s3),
            20 => Some(ABI::s4),
            21 => Some(ABI::s5),
            22 => Some(ABI::s6),
            23 => Some(ABI::s7),
            24 => Some(ABI::s8),
            25 => Some(ABI::s9),
            26 => Some(ABI::s10),
            27 => Some(ABI::s11),
            28 => Some(ABI::t3),
            29 => Some(ABI::t4),
            30 => Some(ABI::t5),
            31 => Some(ABI::t6),
            _ => None,
        }
    }
}

/// Instruction formats for the RISC-V ISA
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FORMAT {
    R,
    I,
    S,
    B,
    U,
    J,
}

/// Instruction opcodes for the RISC-V ISA
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum OPCODE {
    LUI,
    AUIPC,
    JAL,
    JALR,
    BRANCH,
    LOAD,
    STORE,
    OP_IMM,
}
impl OPCODE {
    pub fn to_u32(&self) -> u32 {
        match self {
            OPCODE::LUI => 0b011_0111,
            OPCODE::AUIPC => 0b001_0111,
            OPCODE::JAL => 0b110_1111,
            OPCODE::JALR => 0b110_0111,
            OPCODE::BRANCH => 0b110_0011,
            OPCODE::LOAD => 0b000_0011,
            OPCODE::STORE => 0b010_0011,
            OPCODE::OP_IMM => 0b001_0011,
        }
    }

    pub fn from_u32(n: u32) -> Option<OPCODE> {
        match n {
            0b011_0111 => Some(OPCODE::LUI),
            0b001_0111 => Some(OPCODE::AUIPC),
            0b110_1111 => Some(OPCODE::JAL),
            0b110_0111 => Some(OPCODE::JALR),
            0b110_0011 => Some(OPCODE::BRANCH),
            0b000_0011 => Some(OPCODE::LOAD),
            0b010_0011 => Some(OPCODE::STORE),
            0b001_0011 => Some(OPCODE::OP_IMM),
            _ => None,
        }
    }
}

/// Instruction mnemonics for the RISC-V ISA (RV64I + ???)
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MNEMONIC {
    // RV32I
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
}

/// Decoded instruction structure
#[derive(Debug, PartialEq, Eq)]
pub struct DecodedInstr {
    pub format: FORMAT,
    pub mnemonic: MNEMONIC,
    pub opcode: OPCODE,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>,
    pub rd: Option<REG>,
    pub rs1: Option<REG>,
    pub rs2: Option<REG>,
    pub imm: Option<u64>,
}
