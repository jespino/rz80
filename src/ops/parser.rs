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
    match byte_to_bits(code.next().unwrap()) {
        (0, 0, 0, 0, 1, 0, 1, 0) => {
            (1, Opcode::LDABC)
        },
        (0, 0, 0, 1, 1, 0, 1, 0) => {
            (1, Opcode::LDADE)
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
        (0, 0, 1, 1, 0, 1, 1, 0) => {
            (2, Opcode::LDHLN(code.next().unwrap()))
        },
        (0, 0, r11, r12, r13, 1, 1, 0) => {
            (2, Opcode::LDRN(
                bits_to_reg(r11, r12, r13),
                code.next().unwrap(),
            ))
        },
        (1, 1, 0, 1, 1, 1, 0, 1) => {
            match byte_to_bits(code.next().unwrap()) {
                (0, 0, 1, 1, 0, 1, 1, 0) => {
                    (4, Opcode::LDIXDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
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
        },
        (1, 1, 1, 1, 1, 1, 0, 1) => {
            match byte_to_bits(code.next().unwrap()) {
                (0, 0, 1, 1, 0, 1, 1, 0) => {
                    (4, Opcode::LDIYDN(
                        code.next().unwrap(),
                        code.next().unwrap(),
                    ))
                },
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
        },
        _ => (0, Opcode::NOP)
    }
}
