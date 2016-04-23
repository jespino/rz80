use ops::opcodes::Reg;
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
        0x0A => (1, Opcode::LDABC),
        0x02 => (1, Opcode::LDBCA),
        0x12 => (1, Opcode::LDDEA),
        0x1A => (1, Opcode::LDADE),
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
        0x36 => (2, Opcode::LDHLN(code.next().unwrap())),
        0xED => {
            let second_byte = code.next().unwrap();
            match second_byte {
                0x57 => (2, Opcode::LDAI),
                0x5F => (2, Opcode::LDAR),
                0x47 => (2, Opcode::LDIA),
                0x4F => (2, Opcode::LDRA),
                _ => (0, Opcode::NOP)
            }
        },
        0xDD => {
            let second_byte = code.next().unwrap();
            match second_byte {
                0x36 => {
                    (4, Opcode::LDIXDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
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
                0x36 => {
                    (4, Opcode::LDIYDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
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
        _ => match byte_to_bits(byte) {
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
            _ => (0, Opcode::NOP)
        }
    }
}
