#![cfg(test)]

use ops::parser::parse_op;
use ops::opcodes::Reg;
use ops::opcodes::BigReg;
use ops::opcodes::Opcode;

macro_rules! assert_op {
    ($data:expr, $size:expr, $op:pat) => {{
        let (bytes, op) = parse_op(&mut $data.into_iter());
        assert_eq!(bytes, $size);
        match op {
            $op => assert!(true),
            _ => assert!(false)
        }
    }}
}

#[test]
fn test_parse_ldrr() {
    assert_op!(vec![0b01111000], 1, Opcode::LDRR(Reg::A, Reg::B));
}

#[test]
fn test_parse_ldrn() {
    assert_op!(vec![0b00111110, 0b00000001], 2, Opcode::LDRN(Reg::A, 1));
}

#[test]
fn test_parse_ldrhl() {
    assert_op!(vec![0b01111110], 1, Opcode::LDRHL(Reg::A));
}

#[test]
fn test_parse_ldrixd() {
    assert_op!(vec![0b11011101, 0b01111110, 0b00000001], 3,
               Opcode::LDRIXD(Reg::A, 1));
}

#[test]
fn test_parse_ldriyd() {
    assert_op!(vec![0b11111101, 0b01111110, 0b00000001], 3,
               Opcode::LDRIYD(Reg::A, 1));
}

#[test]
fn test_parse_ldhlr() {
    assert_op!(vec![0b01110111], 1, Opcode::LDHLR(Reg::A));
}

#[test]
fn test_parse_ldixdr() {
    assert_op!(vec![0b11011101, 0b01110111, 0b00000001], 3, Opcode::LDIXDR(1, Reg::A));
}

#[test]
fn test_parse_ldiydr() {
    assert_op!(vec![0b11111101, 0b01110111, 0b00000001], 3, Opcode::LDIYDR(1, Reg::A));
}

#[test]
fn test_parse_ldhln() {
    assert_op!(vec![0b00110110, 0b00000001], 2, Opcode::LDHLN(1));
}

#[test]
fn test_parse_ldixdn() {
    assert_op!(vec![0b11011101, 0b00110110, 0b00000001, 0b00000010], 4, Opcode::LDIXDN(1, 2));
}

#[test]
fn test_parse_ldiydn() {
    assert_op!(vec![0b11111101, 0b00110110, 0b00000001, 0b00000010], 4, Opcode::LDIYDN(1, 2));
}

#[test]
fn test_parse_ldabc() {
    assert_op!(vec![0b00001010], 1, Opcode::LDABC);
}

#[test]
fn test_parse_ldade() {
    assert_op!(vec![0b00011010], 1, Opcode::LDADE);
}

#[test]
fn test_parse_ldann() {
    assert_op!(vec![0b00111010, 0b00000001, 0b00000010], 3, Opcode::LDANN(258));
}

#[test]
fn test_parse_ldbca() {
    assert_op!(vec![0b00000010], 1, Opcode::LDBCA);
}

#[test]
fn test_parse_lddea() {
    assert_op!(vec![0b00010010], 1, Opcode::LDDEA);
}

#[test]
fn test_parse_ldnna() {
    assert_op!(vec![0b00110010, 0b00000001, 0b00000010], 3, Opcode::LDNNA(258));
}

#[test]
fn test_parse_ldai() {
    assert_op!(vec![0xED, 0x57], 2, Opcode::LDAI);
}

#[test]
fn test_parse_ldar() {
    assert_op!(vec![0xED, 0x5F], 2, Opcode::LDAR);
}

#[test]
fn test_parse_ldia() {
    assert_op!(vec![0xED, 0x47], 2, Opcode::LDIA);
}

#[test]
fn test_parse_ldra() {
    assert_op!(vec![0xED, 0x4F], 2, Opcode::LDRA);
}

#[test]
fn test_parse_ldddnn() {
    assert_op!(vec![0b00000001, 0b00000001, 0b00000010], 3, Opcode::LDDDNN(BigReg::BC, 258));
}

#[test]
fn test_parse_ldixnn() {
    assert_op!(vec![0xDD, 0x21, 1, 2], 4, Opcode::LDIXNN(258));
}

#[test]
fn test_parse_ldiynn() {
    assert_op!(vec![0xFD, 0x21, 1, 2], 4, Opcode::LDIYNN(258));
}

#[test]
fn test_parse_ldhlnn() {
    assert_op!(vec![0x2A, 1, 2], 3, Opcode::LDHLNN(258));
}
