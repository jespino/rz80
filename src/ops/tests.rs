#![cfg(test)]

use ops::parser::parse_op;
use ops::opcodes::Reg;
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
