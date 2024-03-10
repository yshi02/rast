

use crate::cpu::defs::*;

/// Decode a 32-bit RISC-V (FIXME: RV64I + ???) instruction
pub fn decode(instruction: u32) -> Option<Instr> {
    match OPCODE::from_u32(instruction & 0b111_1111) {
        // LUI
        Some(OPCODE::LUI) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = (instruction >> 12) << 12;
            Some(Instr {
                format: FORMAT::U,
                mnemonic: MNEMONIC::LUI,
                opcode: OPCODE::LUI,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64)
            })
        },
        // AUIPC
        Some(OPCODE::AUIPC) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = (instruction >> 12) << 12;
            Some(Instr {
                format: FORMAT::U,
                mnemonic: MNEMONIC::AUIPC,
                opcode: OPCODE::AUIPC,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64)
            })
        }

        _ => None
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rand::Rng;

    use super::decode;
    use crate::cpu::defs::*;

    const ITERS: u32 = 10;

    #[test]
    fn test_LUI() {
        let mut rng = rand::thread_rng();
        for _ in 0..ITERS {
            // generate random LUI instruction
            let imm: u32 = rng.gen_range(0..=0b1111_1111_1111_1111_1111);
            let rd: u32 = rng.gen_range(0..=0b1_1111);
            let instruction: u32 = imm << 12 | rd << 7 | OPCODE::LUI.to_u32();
            println!("instruction: {:b}", instruction);
            println!("imm: {:b}", imm);
            println!("rd: {:b}", rd);

            // decode and check
            let instr = decode(instruction).expect("decode returned None");
            println!("{:?}", instr);
            assert_eq!(instr.format, FORMAT::U);
            assert_eq!(instr.mnemonic, MNEMONIC::LUI);
            assert_eq!(instr.opcode, OPCODE::LUI);
            assert_eq!(instr.rd, REG::from_u32(rd));
            assert_eq!(instr.rs1, None);
            assert_eq!(instr.rs2, None);
            assert_eq!(instr.imm, Some((imm as u64) << 12));
        }
    }

    #[test]
    fn test_AUIPC() {
        let mut rng = rand::thread_rng();
        for _ in 0..ITERS {
            // generate random AUIPC instruction
            let imm: u32 = rng.gen_range(0..=0b1111_1111_1111_1111_1111);
            let rd: u32 = rng.gen_range(0..=0b1_1111);
            let instruction: u32 = imm << 12 | rd << 7 | OPCODE::AUIPC.to_u32();
            println!("instruction: {:b}", instruction);
            println!("imm: {:b}", imm);
            println!("rd: {:b}", rd);

            // decode and check
            let instr = decode(instruction).expect("decode returned None");
            println!("{:?}", instr);
            assert_eq!(instr.format, FORMAT::U);
            assert_eq!(instr.mnemonic, MNEMONIC::AUIPC);
            assert_eq!(instr.opcode, OPCODE::AUIPC);
            assert_eq!(instr.rd, REG::from_u32(rd));
            assert_eq!(instr.rs1, None);
            assert_eq!(instr.rs2, None);
            assert_eq!(instr.imm, Some((imm as u64) << 12));
        }
    }
}