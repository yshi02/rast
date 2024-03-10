use crate::cpu::defs::*;

/// Decode a 32-bit RISC-V (FIXME: RV64I + ???) instruction
pub fn decode(instruction: u32) -> Option<Instr> {
    match OPCODE::from_u32(instruction & 0b111_1111) {
        Some(OPCODE::LUI) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = (instruction >> 12) << 12;
            Some(Instr {
                format: FORMAT::U,
                mnemonic: MNEMONIC::LUI,
                opcode: OPCODE::LUI,
                funct3: None,
                funct7: None,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64),
            })
        }

        Some(OPCODE::AUIPC) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = (instruction >> 12) << 12;
            Some(Instr {
                format: FORMAT::U,
                mnemonic: MNEMONIC::AUIPC,
                opcode: OPCODE::AUIPC,
                funct3: None,
                funct7: None,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64),
            })
        }

        Some(OPCODE::JAL) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let imm: u32 = ((instruction >> 11) & 0b1_0000_0000_0000_0000_0000) // imm[20]
                | ((instruction >> 20) & 0b111_1111_1110) // imm[10:1]
                | ((instruction >> 9) & 0b1000_0000_0000) // imm[11]
                | (instruction & 0b1111_1111_0000_0000_0000); // imm[19:12]
            Some(Instr {
                format: FORMAT::J,
                mnemonic: MNEMONIC::JAL,
                opcode: OPCODE::JAL,
                funct3: None,
                funct7: None,
                rd: REG::from_u32(rd),
                rs1: None,
                rs2: None,
                imm: Some(imm as u64),
            })
        }

        Some(OPCODE::JALR) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let rs1: u32 = (instruction >> 15) & 0b1_1111;
            let imm: u32 = instruction >> 20;
            let funct3: u32 = (instruction >> 12) & 0b111;
            Some(Instr {
                format: FORMAT::I,
                mnemonic: MNEMONIC::JALR,
                opcode: OPCODE::JALR,
                funct3: Some(funct3),
                funct7: None,
                rd: REG::from_u32(rd),
                rs1: REG::from_u32(rs1),
                rs2: None,
                imm: Some(imm as u64),
            })
        }
        _ => None,
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rand::Rng;

    use crate::cpu::decoder::*;

    const ITERS: u32 = 1000;

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
            assert_eq!(instr.funct3, None);
            assert_eq!(instr.funct7, None);
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
            assert_eq!(instr.funct3, None);
            assert_eq!(instr.funct7, None);
            assert_eq!(instr.rd, REG::from_u32(rd));
            assert_eq!(instr.rs1, None);
            assert_eq!(instr.rs2, None);
            assert_eq!(instr.imm, Some((imm as u64) << 12));
        }
    }

    #[test]
    fn test_JAL() {
        let mut rng = rand::thread_rng();
        for _ in 0..ITERS {
            // generate random JAL instruction
            let imm: u32 = rng.gen_range(0..=0b1111_1111_1111_1111_1111);
            let rd: u32 = rng.gen_range(0..=0b1_1111);
            let instruction: u32 = imm << 12 | rd << 7 | OPCODE::JAL.to_u32();
            println!("instruction: {:b}", instruction);
            println!("imm: {:b}", imm);
            println!("rd: {:b}", rd);

            // decode and check
            let instr = decode(instruction).expect("decode returned None");
            println!("{:?}", instr);
            assert_eq!(instr.format, FORMAT::J);
            assert_eq!(instr.mnemonic, MNEMONIC::JAL);
            assert_eq!(instr.opcode, OPCODE::JAL);
            assert_eq!(instr.funct3, None);
            assert_eq!(instr.funct7, None);
            assert_eq!(instr.rd, REG::from_u32(rd));
            assert_eq!(instr.rs1, None);
            assert_eq!(instr.rs2, None);
            assert_eq!(
                instr.imm,
                Some(
                    (((imm << 1) & 0b1_0000_0000_0000_0000_0000)
                        | ((imm >> 8) & 0b111_1111_1110)
                        | ((imm << 3) & 0b1000_0000_0000)
                        | ((imm << 12) & 0b1111_1111_0000_0000_0000)) as u64
                )
            );
        }
    }

    #[test]
    fn test_JALR() {
        let mut rng = rand::thread_rng();
        for _ in 0..ITERS {
            // generate random JALR instruction
            let imm: u32 = rng.gen_range(0..=0b1111_1111_1111);
            let rd: u32 = rng.gen_range(0..=0b1_1111);
            let rs1: u32 = rng.gen_range(0..=0b1_1111);
            let instruction: u32 =
                imm << 20 | rs1 << 15 | 0b000 << 10 | rd << 7 | OPCODE::JALR.to_u32();
            println!("instruction: {:b}", instruction);
            println!("imm: {:b}", imm);
            println!("rd: {:b}", rd);
            println!("rs1: {:b}", rs1);

            // decode and check
            let instr = decode(instruction).expect("decode returned None");
            println!("{:?}", instr);
            assert_eq!(instr.format, FORMAT::I);
            assert_eq!(instr.mnemonic, MNEMONIC::JALR);
            assert_eq!(instr.opcode, OPCODE::JALR);
            assert_eq!(instr.funct3, Some(0b000));
            assert_eq!(instr.funct7, None);
            assert_eq!(instr.rd, REG::from_u32(rd));
            assert_eq!(instr.rs1, REG::from_u32(rs1));
            assert_eq!(instr.rs2, None);
            assert_eq!(instr.imm, Some(imm as u64));
        }
    }
}
