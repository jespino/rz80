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

#[test]
fn test_parse_ldddnn2() {
    assert_op!(vec![0xED, 0b01001011, 1, 2], 4, Opcode::LDDDNN2(BigReg::BC, 258));
}

#[test]
fn test_parse_ldixnn2() {
    assert_op!(vec![0xDD, 0x2A, 1, 2], 4, Opcode::LDIXNN2(258));
}

#[test]
fn test_parse_ldiynn2() {
    assert_op!(vec![0xFD, 0x2A, 1, 2], 4, Opcode::LDIYNN2(258));
}

#[test]
fn test_parse_ldnnhl() {
    assert_op!(vec![0x22, 1, 2], 3, Opcode::LDNNHL(258));
}

#[test]
fn test_parse_ldnndd() {
    assert_op!(vec![0xED, 0b01000011, 1, 2], 4, Opcode::LDNNDD(258, BigReg::BC));
}

#[test]
fn test_parse_ldnnix() {
    assert_op!(vec![0xDD, 0x22, 1, 2], 4, Opcode::LDNNIX(258));
}

#[test]
fn test_parse_ldnniy() {
    assert_op!(vec![0xFD, 0x22, 1, 2], 4, Opcode::LDNNIY(258));
}

#[test]
fn test_parse_ldsphl() {
    assert_op!(vec![0xF9], 1, Opcode::LDSPHL);
}

#[test]
fn test_parse_ldspix() {
    assert_op!(vec![0xDD, 0xF9], 2, Opcode::LDSPIX);
}

#[test]
fn test_parse_ldspiy() {
    assert_op!(vec![0xFD, 0xF9], 2, Opcode::LDSPIY);
}

#[test]
fn test_parse_pushqq() {
    assert_op!(vec![0b11000101], 1, Opcode::PUSHQQ(BigReg::BC));
}

#[test]
fn test_parse_pushix() {
    assert_op!(vec![0xDD, 0xE5], 2, Opcode::PUSHIX);
}

#[test]
fn test_parse_pushiy() {
    assert_op!(vec![0xFD, 0xE5], 2, Opcode::PUSHIY);
}

#[test]
fn test_parse_popqq() {
    assert_op!(vec![0b11000001], 1, Opcode::POPQQ(BigReg::BC));
}

#[test]
fn test_parse_popix() {
    assert_op!(vec![0xDD, 0xE1], 2, Opcode::POPIX);
}

#[test]
fn test_parse_popiy() {
    assert_op!(vec![0xFD, 0xE1], 2, Opcode::POPIY);
}

#[test]
fn test_parse_exdehl() {
    assert_op!(vec![0xEB], 1, Opcode::EXDEHL);
}

#[test]
fn test_parse_exafaf2() {
    assert_op!(vec![0x08], 1, Opcode::EXAFAF2);
}

#[test]
fn test_parse_exx() {
    assert_op!(vec![0xD9], 1, Opcode::EXX);
}

#[test]
fn test_parse_exsphl() {
    assert_op!(vec![0xE3], 1, Opcode::EXSPHL);
}

#[test]
fn test_parse_exspix() {
    assert_op!(vec![0xDD, 0xE3], 2, Opcode::EXSPIX);
}

#[test]
fn test_parse_exspiy() {
    assert_op!(vec![0xFD, 0xE3], 2, Opcode::EXSPIY);
}

#[test]
fn test_parse_ldi() {
    assert_op!(vec![0xED, 0xA0], 2, Opcode::LDI);
}

#[test]
fn test_parse_ldir() {
    assert_op!(vec![0xED, 0xB0], 2, Opcode::LDIR);
}

#[test]
fn test_parse_ldd() {
    assert_op!(vec![0xED, 0xA8], 2, Opcode::LDD);
}

#[test]
fn test_parse_lddr() {
    assert_op!(vec![0xED, 0xB8], 2, Opcode::LDDR);
}

#[test]
fn test_parse_cpi() {
    assert_op!(vec![0xED, 0xA1], 2, Opcode::CPI);
}

#[test]
fn test_parse_cpir() {
    assert_op!(vec![0xED, 0xB1], 2, Opcode::CPIR);
}

#[test]
fn test_parse_cpd() {
    assert_op!(vec![0xED, 0xA9], 2, Opcode::CPD);
}

#[test]
fn test_parse_cpdr() {
    assert_op!(vec![0xED, 0xB9], 2, Opcode::CPDR);
}

#[test]
fn test_parse_addar() {
    assert_op!(vec![0b10000111], 1, Opcode::ADDAR(Reg::A));
}

#[test]
fn test_parse_addan() {
    assert_op!(vec![0xC6, 1], 2, Opcode::ADDAN(1));
}

#[test]
fn test_parse_addahl() {
    assert_op!(vec![0x86], 1, Opcode::ADDAHL);
}

#[test]
fn test_parse_addaixd() {
    assert_op!(vec![0xDD, 0x86, 1], 3, Opcode::ADDAIXD(1));
}

#[test]
fn test_parse_addaiyd() {
    assert_op!(vec![0xFD, 0x86, 1], 3, Opcode::ADDAIYD(1));
}

#[test]
fn test_parse_subar() {
    assert_op!(vec![0b10010111], 1, Opcode::SUBAR(Reg::A));
}

#[test]
fn test_parse_suban() {
    assert_op!(vec![0xD6, 1], 2, Opcode::SUBAN(1));
}

#[test]
fn test_parse_subahl() {
    assert_op!(vec![0x96], 1, Opcode::SUBAHL);
}

#[test]
fn test_parse_subaixd() {
    assert_op!(vec![0xDD, 0x96, 1], 3, Opcode::SUBAIXD(1));
}

#[test]
fn test_parse_subaiyd() {
    assert_op!(vec![0xFD, 0x96, 1], 3, Opcode::SUBAIYD(1));
}
