use ops::opcodes::Reg;
use ops::opcodes::BigReg;
use ops::opcodes::Opcode;

fn bits_to_reg(bit1: u8, bit2: u8, bit3: u8) -> Reg {
    match (bit1, bit2, bit3) {
        (1,1,1) => Reg::A,
        (0,0,0) => Reg::B,
        (0,0,1) => Reg::C,
        (0,1,0) => Reg::D,
        (0,1,1) => Reg::E,
        (1,0,0) => Reg::H,
        (1,0,1) => Reg::L,
        _ => unreachable!()
    }
}

fn bits_to_bigreg1(bit1: u8, bit2: u8) -> BigReg {
    match (bit1, bit2) {
        (0,0) => BigReg::BC,
        (0,1) => BigReg::DE,
        (1,0) => BigReg::HL,
        (1,1) => BigReg::SP,
        _ => unreachable!()
    }
}

fn byte_to_bits(byte: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (byte >> 7 & 1,
     byte >> 6 & 1,
     byte >> 5 & 1,
     byte >> 4 & 1,
     byte >> 3 & 1,
     byte >> 2 & 1,
     byte >> 1 & 1,
     byte & 1)
}


pub fn parse_op(code: &mut Iterator<Item=u8>) -> (u8, Opcode) {
    let byte = code.next().unwrap();
    match byte {
        0x02 => (1, Opcode::LDBCA),
        0x08 => (1, Opcode::EXAFAF2),
        0x0A => (1, Opcode::LDABC),
        0x12 => (1, Opcode::LDDEA),
        0x1A => (1, Opcode::LDADE),
        0x22 => {
            let byte1 = (code.next().unwrap() as u16) << 8;
            let byte2 = code.next().unwrap() as u16;
            (3, Opcode::LDNNHL(byte1 + byte2))
        }
        0x2A => {
            let byte1 = (code.next().unwrap() as u16) << 8;
            let byte2 = code.next().unwrap() as u16;
            (3, Opcode::LDHLNN(byte1 + byte2))
        }
        0x32 => {
            let byte1 = (code.next().unwrap() as u16) << 8;
            let byte2 = code.next().unwrap() as u16;
            (3, Opcode::LDNNA(byte1 + byte2))
        },
        0x3A => {
            let byte1 = (code.next().unwrap() as u16) << 8;
            let byte2 = code.next().unwrap() as u16;
            (3, Opcode::LDANN(byte1 + byte2))
        },
        0x34 => (1, Opcode::INCHL),
        0x35 => (1, Opcode::DECHL),
        0x36 => (2, Opcode::LDHLN(code.next().unwrap())),
        0x86 => (1, Opcode::ADDAHL),
        0x96 => (1, Opcode::SUBAHL),
        0x9E => (1, Opcode::SBCAHL),
        0xA6 => (1, Opcode::ANDAHL),
        0xAE => (1, Opcode::XORAHL),
        0xB6 => (1, Opcode::ORAHL),
        0xBE => (1, Opcode::CPAHL),
        0xC6 => (2, Opcode::ADDAN(code.next().unwrap())),
        0xD6 => (2, Opcode::SUBAN(code.next().unwrap())),
        0xDE => (2, Opcode::SBCAN(code.next().unwrap())),
        0xE6 => (2, Opcode::ANDAN(code.next().unwrap())),
        0xEE => (2, Opcode::XORAN(code.next().unwrap())),
        0xF6 => (2, Opcode::ORAN(code.next().unwrap())),
        0xFE => (2, Opcode::CPAN(code.next().unwrap())),
        0xD9 => (1, Opcode::EXX),
        0xE3 => (1, Opcode::EXSPHL),
        0xEB => (1, Opcode::EXDEHL),
        0xED => {
            let second_byte = code.next().unwrap();
            match second_byte {
                0x57 => (2, Opcode::LDAI),
                0x5F => (2, Opcode::LDAR),
                0x47 => (2, Opcode::LDIA),
                0x4F => (2, Opcode::LDRA),
                0xA0 => (2, Opcode::LDI),
                0xA1 => (2, Opcode::CPI),
                0xA8 => (2, Opcode::LDD),
                0xA9 => (2, Opcode::CPD),
                0xB0 => (2, Opcode::LDIR),
                0xB1 => (2, Opcode::CPIR),
                0xB8 => (2, Opcode::LDDR),
                0xB9 => (2, Opcode::CPDR),
                _ => match byte_to_bits(second_byte) {
                    (0, 1, d1, d2, 1, 0, 1, 1) => {
                        let byte1 = (code.next().unwrap() as u16) << 8;
                        let byte2 = code.next().unwrap() as u16;
                        (4, Opcode::LDDDNN2(
                            bits_to_bigreg1(d1, d2),
                            byte1 + byte2,
                        ))
                    },
                    (0, 1, d1, d2, 0, 0, 1, 1) => {
                        let byte1 = (code.next().unwrap() as u16) << 8;
                        let byte2 = code.next().unwrap() as u16;
                        (4, Opcode::LDNNDD(
                            byte1 + byte2,
                            bits_to_bigreg1(d1, d2),
                        ))
                    },
                    _ => (0, Opcode::NOP)
                }
            }
        },
        0xDD => {
            let second_byte = code.next().unwrap();
            match second_byte {
                0x21 => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDIXNN(byte1 + byte2))
                },
                0x22 => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDNNIX(byte1 + byte2))
                },
                0x2A => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDIXNN2(byte1 + byte2))
                },
                0x36 => {
                    (4, Opcode::LDIXDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
                0x34 => (3, Opcode::INCIXD(code.next().unwrap())),
                0x35 => (3, Opcode::DECIXD(code.next().unwrap())),
                0x86 => (3, Opcode::ADDAIXD(code.next().unwrap())),
                0x96 => (3, Opcode::SUBAIXD(code.next().unwrap())),
                0x9E => (3, Opcode::SBCAIXD(code.next().unwrap())),
                0xA6 => (3, Opcode::ANDAIXD(code.next().unwrap())),
                0xAE => (3, Opcode::XORAIXD(code.next().unwrap())),
                0xB6 => (3, Opcode::ORAIXD(code.next().unwrap())),
                0xBE => (3, Opcode::CPAIXD(code.next().unwrap())),

                0xE1 => (2, Opcode::POPIX),
                0xE3 => (2, Opcode::EXSPIX),
                0xE5 => (2, Opcode::PUSHIX),
                0xF9 => (2, Opcode::LDSPIX),
                _ => match byte_to_bits(second_byte) {
                    (0, 1, 1, 1, 0, r11, r12, r13) => {
                        (3, Opcode::LDIXDR(
                            code.next().unwrap(),
                            bits_to_reg(r11, r12, r13),
                        ))
                    },
                    (0, 1, r11, r12, r13, 1, 1, 0) => {
                        (3, Opcode::LDRIXD(
                            bits_to_reg(r11, r12, r13),
                            code.next().unwrap(),
                        ))
                    },
                    _ => (0, Opcode::NOP)
                }
            }
        },
        0xFD => {
            let second_byte = code.next().unwrap();
            match second_byte {
                0x21 => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDIYNN(byte1 + byte2))
                },
                0x22 => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDNNIY(byte1 + byte2))
                },
                0x2A => {
                    let byte1 = (code.next().unwrap() as u16) << 8;
                    let byte2 = code.next().unwrap() as u16;
                    (4, Opcode::LDIYNN2(byte1 + byte2))
                },
                0x36 => {
                    (4, Opcode::LDIYDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
                0x34 => (3, Opcode::INCIYD(code.next().unwrap())),
                0x35 => (3, Opcode::DECIYD(code.next().unwrap())),
                0x86 => (3, Opcode::ADDAIYD(code.next().unwrap())),
                0x96 => (3, Opcode::SUBAIYD(code.next().unwrap())),
                0x9E => (3, Opcode::SBCAIYD(code.next().unwrap())),
                0xA6 => (3, Opcode::ANDAIYD(code.next().unwrap())),
                0xAE => (3, Opcode::XORAIYD(code.next().unwrap())),
                0xB6 => (3, Opcode::ORAIYD(code.next().unwrap())),
                0xBE => (3, Opcode::CPAIYD(code.next().unwrap())),
                0xE1 => (2, Opcode::POPIY),
                0xE3 => (2, Opcode::EXSPIY),
                0xE5 => (2, Opcode::PUSHIY),
                0xF9 => (2, Opcode::LDSPIY),
                _ => match byte_to_bits(second_byte) {
                    (0, 1, 1, 1, 0, r11, r12, r13) => {
                        (3, Opcode::LDIYDR(
                            code.next().unwrap(),
                            bits_to_reg(r11, r12, r13),
                        ))
                    },
                    (0, 1, r11, r12, r13, 1, 1, 0) => {
                        (3, Opcode::LDRIYD(
                            bits_to_reg(r11, r12, r13),
                            code.next().unwrap(),
                        ))
                    },
                    _ => (0, Opcode::NOP)
                }
            }
        },
        0xF9 => (1, Opcode::LDSPHL),
        _ => match byte_to_bits(byte) {
            (1, 0, 0, 0, 0, r11, r12, r13) => {
                (1, Opcode::ADDAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 0, 1, 0, r11, r12, r13) => {
                (1, Opcode::SUBAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 0, 1, 1, r11, r12, r13) => {
                (1, Opcode::SBCAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 1, 0, 0, r11, r12, r13) => {
                (1, Opcode::ANDAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 1, 0, 1, r11, r12, r13) => {
                (1, Opcode::XORAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 1, 1, 0, r11, r12, r13) => {
                (1, Opcode::ORAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (1, 0, 1, 1, 1, r11, r12, r13) => {
                (1, Opcode::CPAR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (0, 1, 1, 1, 0, r11, r12, r13) => {
                (1, Opcode::LDHLR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (0, 1, r11, r12, r13, 1, 1, 0) => {
                (1, Opcode::LDRHL(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (0, 1, r11, r12, r13, r21, r22, r23) => {
                (1, Opcode::LDRR(
                    bits_to_reg(r11, r12, r13),
                    bits_to_reg(r21, r22, r23),
                ))
            },
            (0, 0, r11, r12, r13, 1, 1, 0) => {
                (2, Opcode::LDRN(
                    bits_to_reg(r11, r12, r13),
                    code.next().unwrap(),
                ))
            },
            (0, 0, r11, r12, r13, 1, 0, 0) => {
                (1, Opcode::INCR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (0, 0, r11, r12, r13, 1, 0, 1) => {
                (1, Opcode::DECR(
                    bits_to_reg(r11, r12, r13),
                ))
            },
            (0, 0, d1, d2, 0, 0, 0, 1) => {
                let byte1 = (code.next().unwrap() as u16) << 8;
                let byte2 = code.next().unwrap() as u16;
                (3, Opcode::LDDDNN(
                    bits_to_bigreg1(d1, d2),
                    byte1 + byte2,
                ))
            },
            (1, 1, d1, d2, 0, 1, 0, 1) => {
                (1, Opcode::PUSHQQ(
                    bits_to_bigreg1(d1, d2),
                ))
            },
            (1, 1, d1, d2, 0, 0, 0, 1) => {
                (1, Opcode::POPQQ(
                    bits_to_bigreg1(d1, d2),
                ))
            },
            _ => (0, Opcode::NOP)
        }
    }
}
