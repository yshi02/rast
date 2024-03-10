

use crate::cpu::defs::*;

use rand;

/// Decode a 32-bit RISC-V (FIXME: RV64I + ???) instruction
pub fn decode(instruction: u32) -> Option<Instr> {
    match instruction & 0b111_1111 {
        // LUI
        0b011_0111 => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = (instruction >> 12) << 12;
            Some(Instr {
                format: FORMAT::U,
                mnemonic: MNEMONIC::LUI,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64)
            })
        },

        _ => None
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rand::Rng;

    const ITERS: u32 = 10;

    #[test]
    fn test_LUI() {
        let mut rng = rand::thread_rng();
        for _ in 0..ITERS {
            // generate random LUI instruction
            let imm: u32 = rng.gen_range(0..=0b1111_1111_1111_1111_1111);
            let rd: u32 = rng.gen_range(0..=0b1_1111);
            let instruction: u32 = imm << 12 | rd << 7 | 0b011_0111;

            // decode and check
            let instr = super::decode(instruction).expect("decode returned None");
            assert_eq!(instr.format, super::FORMAT::U);
            assert_eq!(instr.mnemonic, super::MNEMONIC::LUI);
            assert_eq!(instr.rd, super::REG::from_u32(rd));
            assert_eq!(instr.rs1, None);
            assert_eq!(instr.rs2, None);
            assert_eq!(instr.imm, Some((imm as u64) << 12));
        }
    }
}