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

        Some(OPCODE::BRANCH) => {
            let funct3: u32 = (instruction >> 12) & 0b111;
            let rs1: u32 = (instruction >> 15) & 0b1_1111;
            let rs2: u32 = (instruction >> 20) & 0b1_1111;
            let imm: u32 = ((instruction >> 19) & 0b1_0000_0000_0000) // imm[12]
                | ((instruction >> 20) & 0b111_1110_0000) // imm[10:5]
                | ((instruction >> 7) & 0b1_1110) // imm[4:1]
                | ((instruction << 4) & 0b1000_0000_0000); // imm[11]
            match funct3 {
                0b000 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BEQ,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                0b001 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BNE,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                0b100 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BLT,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                0b101 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BGE,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                0b110 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BLTU,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                0b111 => Some(Instr {
                    format: FORMAT::B,
                    mnemonic: MNEMONIC::BGEU,
                    opcode: OPCODE::BRANCH,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: None,
                    rs1: REG::from_u32(rs1),
                    rs2: REG::from_u32(rs2),
                    imm: Some(imm as u64),
                }),

                _ => {
                    dbg!("decode: unknown funct3 {:b} for BRANCH operation", funct3);
                    None
                }
            }
        }

        Some(OPCODE::LOAD) => {
            let rd: u32 = (instruction >> 7) & 0b1_1111;
            let funct3: u32 = (instruction >> 12) & 0b111;
            let rs1: u32 = (instruction >> 15) & 0b1_1111;
            let imm: u32 = (instruction >> 20) & 0b1111_1111_1111;
            match funct3 {
                0b000 => Some(Instr {
                    format: FORMAT::I,
                    mnemonic: MNEMONIC::LB,
                    opcode: OPCODE::LOAD,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: REG::from_u32(rd),
                    rs1: REG::from_u32(rs1),
                    rs2: None,
                    imm: Some(imm as u64),
                }),

                0b001 => Some(Instr {
                    format: FORMAT::I,
                    mnemonic: MNEMONIC::LH,
                    opcode: OPCODE::LOAD,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: REG::from_u32(rd),
                    rs1: REG::from_u32(rs1),
                    rs2: None,
                    imm: Some(imm as u64),
                }),

                0b010 => Some(Instr {
                    format: FORMAT::I,
                    mnemonic: MNEMONIC::LW,
                    opcode: OPCODE::LOAD,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: REG::from_u32(rd),
                    rs1: REG::from_u32(rs1),
                    rs2: None,
                    imm: Some(imm as u64),
                }),

                0b100 => Some(Instr {
                    format: FORMAT::I,
                    mnemonic: MNEMONIC::LBU,
                    opcode: OPCODE::LOAD,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: REG::from_u32(rd),
                    rs1: REG::from_u32(rs1),
                    rs2: None,
                    imm: Some(imm as u64),
                }),

                0b101 => Some(Instr {
                    format: FORMAT::I,
                    mnemonic: MNEMONIC::LHU,
                    opcode: OPCODE::LOAD,
                    funct3: Some(funct3),
                    funct7: None,
                    rd: REG::from_u32(rd),
                    rs1: REG::from_u32(rs1),
                    rs2: None,
                    imm: Some(imm as u64),
                }),

                _ => {
                    dbg!("decode: unknown funct3 {:b} for LOAD operation", funct3);
                    None
                }
            }
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

    #[test]
    fn test_BRANCHes() {
        let mut rng = rand::thread_rng();
        for funct3 in 0..=0b111 {
            for _ in 0..ITERS {
                // generate random BRANCH instruction
                let imm1: u32 = rng.gen_range(0..=0b111_1111);
                let imm2: u32 = rng.gen_range(0..=0b1_1111);
                let rs1: u32 = rng.gen_range(0..=0b1_1111);
                let rs2: u32 = rng.gen_range(0..=0b1_1111);
                let instruction: u32 = imm1 << 25
                    | rs2 << 20
                    | rs1 << 15
                    | funct3 << 12
                    | imm2 << 7
                    | OPCODE::BRANCH.to_u32();
                println!("instruction: {:b}", instruction);
                println!("imm1: {:b}", imm1);
                println!("imm2: {:b}", imm2);
                println!("rs1: {:b}", rs1);
                println!("rs2: {:b}", rs2);
                println!("funct3: {:b}", funct3);

                // decode and check
                let instr = decode(instruction);
                println!("{:?}", instr);
                if funct3 == 0b010 || funct3 == 0b011 {
                    assert_eq!(instr, None);
                } else {
                    let instr = instr.unwrap();
                    assert_eq!(instr.format, FORMAT::B);
                    match funct3 {
                        0b000 => assert_eq!(instr.mnemonic, MNEMONIC::BEQ),
                        0b001 => assert_eq!(instr.mnemonic, MNEMONIC::BNE),
                        0b100 => assert_eq!(instr.mnemonic, MNEMONIC::BLT),
                        0b101 => assert_eq!(instr.mnemonic, MNEMONIC::BGE),
                        0b110 => assert_eq!(instr.mnemonic, MNEMONIC::BLTU),
                        0b111 => assert_eq!(instr.mnemonic, MNEMONIC::BGEU),
                        _ => panic!("unknown funct3 {:b} for BRANCH instruction", funct3),
                    }
                    assert_eq!(instr.opcode, OPCODE::BRANCH);
                    assert_eq!(instr.funct3, Some(funct3));
                    assert_eq!(instr.funct7, None);
                    assert_eq!(instr.rd, None);
                    assert_eq!(instr.rs1, REG::from_u32(rs1));
                    assert_eq!(instr.rs2, REG::from_u32(rs2));
                    assert_eq!(
                        instr.imm,
                        Some(
                            (((imm1 << 6) & 0b1_0000_0000_0000) // imm[12]
                                | ((imm1 << 5) & 0b111_1110_0000) // imm[10:5]
                                | (imm2 & 0b1_1110) // imm[4:1]
                                | (imm2 << 11) & 0b1000_0000_0000) // imm[11]
                                as u64
                        )
                    );
                }
            }
        }
    }

    #[test]
    fn test_LOADs() {
        let mut rng = rand::thread_rng();
        for funct3 in 0..=0b111 {
            for _ in 0..ITERS {
                // generate random LOAD instruction
                let rd: u32 = rng.gen_range(0..=0b1_1111);
                let rs1: u32 = rng.gen_range(0..=0b1_1111);
                let imm: u32 = rng.gen_range(0..=0b1111_1111_1111);
                let instruction: u32 =
                    imm << 20 | rs1 << 15 | funct3 << 12 | rd << 7 | OPCODE::LOAD.to_u32();
                println!("rd: {:b}", rd);
                println!("funct3: {:b}", funct3);
                println!("rs1: {:b}", rs1);
                println!("imm: {:b}", imm);
                println!("instruction: {:b}", instruction);

                // decode and check
                let instr = decode(instruction);
                println!("{:?}", instr);
                if funct3 == 0b011 || funct3 == 0b110 || funct3 == 0b111 {
                    assert_eq!(instr, None);
                } else {
                    let instr = instr.unwrap();
                    assert_eq!(instr.format, FORMAT::I);
                    match funct3 {
                        0b000 => assert_eq!(instr.mnemonic, MNEMONIC::LB),
                        0b001 => assert_eq!(instr.mnemonic, MNEMONIC::LH),
                        0b010 => assert_eq!(instr.mnemonic, MNEMONIC::LW),
                        0b100 => assert_eq!(instr.mnemonic, MNEMONIC::LBU),
                        0b101 => assert_eq!(instr.mnemonic, MNEMONIC::LHU),
                        _ => panic!("unknown funct3 {:b} for LOAD instruction", funct3),
                    }
                    assert_eq!(instr.opcode, OPCODE::LOAD);
                    assert_eq!(instr.funct3, Some(funct3));
                    assert_eq!(instr.funct7, None);
                    assert_eq!(instr.rd, REG::from_u32(rd));
                    assert_eq!(instr.rs1, REG::from_u32(rs1));
                    assert_eq!(instr.rs2, None);
                    assert_eq!(instr.imm, Some(imm as u64));
                }
            }
        }
    }

    // do no fold me
}
