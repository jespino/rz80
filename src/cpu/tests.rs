#![cfg(test)]

use cpu::Z80;
use ops::opcodes::Opcode;
use ops::opcodes::Reg;
use ops::opcodes::BigReg;

#[test]
fn test_run_ldrr() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x8A;
    cpu.regs[Reg::E] = 0x10;
    cpu.run_op(Opcode::LDRR(Reg::H, Reg::E));
    assert_eq!(cpu.regs[Reg::H], 0x10);
    assert_eq!(cpu.regs[Reg::E], 0x10);
}

#[test]
fn test_run_ldrn() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::E] = 0x8A;
    cpu.run_op(Opcode::LDRN(Reg::E, 0x20));
    assert_eq!(cpu.regs[Reg::E], 0x20);
}

#[test]
fn test_run_ldrhl() {
    let mut cpu = Z80::new();
    cpu.mem[0x75A1] = 0x58;
    cpu.regs[Reg::C] = 0;
    cpu.regs[Reg::H] = 0x75;
    cpu.regs[Reg::L] = 0xA1;
    cpu.run_op(Opcode::LDRHL(Reg::C));
    assert_eq!(cpu.regs[Reg::C], 0x58);
}

#[test]
fn test_run_ldrixd() {
    let mut cpu = Z80::new();
    cpu.mem[0x25C8] = 0x39;
    cpu.ix = 0x25AF;
    cpu.regs[Reg::B] = 0;
    cpu.run_op(Opcode::LDRIXD(Reg::B, 0x19));
    assert_eq!(cpu.regs[Reg::B], 0x39);
}

#[test]
fn test_run_ldriyd() {
    let mut cpu = Z80::new();
    cpu.mem[0x25C8] = 0x39;
    cpu.iy = 0x25AF;
    cpu.regs[Reg::B] = 0;
    cpu.run_op(Opcode::LDRIYD(Reg::B, 0x19));
    assert_eq!(cpu.regs[Reg::B], 0x39);
}

#[test]
fn test_run_ldhlr() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::B] = 0x29;
    cpu.regs[Reg::H] = 0x21;
    cpu.regs[Reg::L] = 0x46;
    cpu.run_op(Opcode::LDHLR(Reg::B));
    assert_eq!(cpu.mem[0x2146], 0x29);
}

#[test]
fn test_run_ldixdr() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::C] = 0x1C;
    cpu.ix = 0x3100;
    cpu.run_op(Opcode::LDIXDR(0x6, Reg::C));
    assert_eq!(cpu.mem[0x3106], 0x1C);
}

#[test]
fn test_run_ldiydr() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::C] = 0x48;
    cpu.iy = 0x2A11;
    cpu.run_op(Opcode::LDIYDR(0x4, Reg::C));
    assert_eq!(cpu.mem[0x2A15], 0x48);
}

#[test]
fn test_run_ldhln() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x44;
    cpu.regs[Reg::L] = 0x44;
    cpu.run_op(Opcode::LDHLN(0x28));
    assert_eq!(cpu.mem[0x4444], 0x28);
}

#[test]
fn test_run_ldixdn() {
    let mut cpu = Z80::new();
    cpu.ix = 0xA940;
    cpu.run_op(Opcode::LDIXDN(0x10, 0x97));
    assert_eq!(cpu.mem[0xA950], 0x97);
}

#[test]
fn test_run_ldiydn() {
    let mut cpu = Z80::new();
    cpu.iy = 0xA940;
    cpu.run_op(Opcode::LDIYDN(0x10, 0x97));
    assert_eq!(cpu.mem[0xA950], 0x97);
}

#[test]
fn test_run_ldabc() {
    let mut cpu = Z80::new();
    cpu.mem[0x4747] = 0x12;
    cpu.regs[Reg::B] = 0x47;
    cpu.regs[Reg::C] = 0x47;
    cpu.run_op(Opcode::LDABC);
    assert_eq!(cpu.regs[Reg::A], 0x12);
}

#[test]
fn test_run_ldade() {
    let mut cpu = Z80::new();
    cpu.mem[0x30A2] = 0x22;
    cpu.regs[Reg::D] = 0x30;
    cpu.regs[Reg::E] = 0xA2;
    cpu.run_op(Opcode::LDADE);
    assert_eq!(cpu.regs[Reg::A], 0x22);
}

#[test]
fn test_run_ldann() {
    let mut cpu = Z80::new();
    cpu.mem[0x8832] = 0x4;
    cpu.run_op(Opcode::LDANN(0x8832));
    assert_eq!(cpu.regs[Reg::A], 0x4);
}

#[test]
fn test_run_ldbca() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x7A;
    cpu.regs[Reg::B] = 0x12;
    cpu.regs[Reg::C] = 0x12;
    cpu.run_op(Opcode::LDBCA);
    assert_eq!(cpu.mem[0x1212], 0x7A);
}

#[test]
fn test_run_lddea() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xA0;
    cpu.regs[Reg::D] = 0x11;
    cpu.regs[Reg::E] = 0x28;
    cpu.run_op(Opcode::LDDEA);
    assert_eq!(cpu.mem[0x1128], 0xA0);
}

#[test]
fn test_run_ldnna() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xD7;
    cpu.run_op(Opcode::LDNNA(0x3141));
    assert_eq!(cpu.mem[0x3141], 0xD7);
}

#[test]
fn test_run_ldai() {
    let mut cpu = Z80::new();
    cpu.i = 0xD7;
    cpu.run_op(Opcode::LDAI);
    assert_eq!(cpu.regs[Reg::A], 0xD7);
    assert_eq!(cpu.regs[Reg::F], 0b10000000);

    cpu.i = 0;
    cpu.regs[Reg::F] = 0;
    cpu.run_op(Opcode::LDAI);
    assert_eq!(cpu.regs[Reg::A], 0);
    assert_eq!(cpu.regs[Reg::F], 0b01000000);
}

#[test]
fn test_run_ldar() {
    let mut cpu = Z80::new();
    cpu.r = 0xD7;
    cpu.run_op(Opcode::LDAR);
    assert_eq!(cpu.regs[Reg::A], 0xD7);
    assert_eq!(cpu.regs[Reg::F], 0b10000000);

    cpu.r = 0;
    cpu.regs[Reg::F] = 0;
    cpu.run_op(Opcode::LDAI);
    assert_eq!(cpu.regs[Reg::A], 0);
    assert_eq!(cpu.regs[Reg::F], 0b01000000);
}

#[test]
fn test_run_ldia() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xD7;
    cpu.run_op(Opcode::LDIA);
    assert_eq!(cpu.i, 0xD7);
}

#[test]
fn test_run_ldra() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xD7;
    cpu.run_op(Opcode::LDRA);
    assert_eq!(cpu.r, 0xD7);
}

#[test]
fn test_run_ldddnn() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x99;
    cpu.regs[Reg::L] = 0x99;
    cpu.run_op(Opcode::LDDDNN(BigReg::HL, 0x5000));
    assert_eq!(cpu.regs[Reg::H], 0x50);
    assert_eq!(cpu.regs[Reg::L], 0x00);
}

#[test]
fn test_run_ldixnn() {
    let mut cpu = Z80::new();
    cpu.run_op(Opcode::LDIXNN(0x45A2));
    assert_eq!(cpu.ix, 0x45A2);
}

#[test]
fn test_run_ldiynn() {
    let mut cpu = Z80::new();
    cpu.run_op(Opcode::LDIYNN(0x45A2));
    assert_eq!(cpu.iy, 0x45A2);
}

#[test]
fn test_run_ldhlnn() {
    let mut cpu = Z80::new();
    cpu.mem[0x4545] = 0x37;
    cpu.mem[0x4546] = 0xA1;
    cpu.run_op(Opcode::LDHLNN(0x4545));
    assert_eq!(cpu.regs[Reg::H], 0xA1);
    assert_eq!(cpu.regs[Reg::L], 0x37);
}

#[test]
fn test_run_ldddnn2() {
    let mut cpu = Z80::new();
    cpu.mem[0x2130] = 0x65;
    cpu.mem[0x2131] = 0x78;
    cpu.run_op(Opcode::LDDDNN2(BigReg::HL, 0x2130));
    assert_eq!(cpu.regs[Reg::H], 0x78);
    assert_eq!(cpu.regs[Reg::L], 0x65);
}

#[test]
fn test_run_ldixnn2() {
    let mut cpu = Z80::new();
    cpu.mem[0x6666] = 0x92;
    cpu.mem[0x6667] = 0xDA;
    cpu.run_op(Opcode::LDIXNN2(0x6666));
    assert_eq!(cpu.ix, 0xDA92);
}

#[test]
fn test_run_ldiynn2() {
    let mut cpu = Z80::new();
    cpu.mem[0x6666] = 0x92;
    cpu.mem[0x6667] = 0xDA;
    cpu.run_op(Opcode::LDIYNN2(0x6666));
    assert_eq!(cpu.iy, 0xDA92);
}

#[test]
fn test_run_ldnnhl() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x48;
    cpu.regs[Reg::L] = 0x3A;
    cpu.run_op(Opcode::LDNNHL(0xB229));
    assert_eq!(cpu.mem[0xB229], 0x3A);
    assert_eq!(cpu.mem[0xB22A], 0x48);
}

#[test]
fn test_run_ldnndd() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x48;
    cpu.regs[Reg::L] = 0x3A;
    cpu.run_op(Opcode::LDNNDD(0xB229, BigReg::HL));
    assert_eq!(cpu.mem[0xB229], 0x3A);
    assert_eq!(cpu.mem[0xB22A], 0x48);
}

#[test]
fn test_run_ldnnix() {
    let mut cpu = Z80::new();
    cpu.ix = 0x5A30;
    cpu.run_op(Opcode::LDNNIX(0x4392));
    assert_eq!(cpu.mem[0x4392], 0x30);
    assert_eq!(cpu.mem[0x4393], 0x5A);
}

#[test]
fn test_run_ldnniy() {
    let mut cpu = Z80::new();
    cpu.iy = 0x5A30;
    cpu.run_op(Opcode::LDNNIY(0x4392));
    assert_eq!(cpu.mem[0x4392], 0x30);
    assert_eq!(cpu.mem[0x4393], 0x5A);
}

#[test]
fn test_run_ldsphl() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x44;
    cpu.regs[Reg::L] = 0x23;
    cpu.run_op(Opcode::LDSPHL);
    assert_eq!(cpu.sp, 0x4423);
}

#[test]
fn test_run_ldspix() {
    let mut cpu = Z80::new();
    cpu.ix = 0x4423;
    cpu.run_op(Opcode::LDSPIX);
    assert_eq!(cpu.sp, 0x4423);
}

#[test]
fn test_run_ldspiy() {
    let mut cpu = Z80::new();
    cpu.iy = 0x4423;
    cpu.run_op(Opcode::LDSPIY);
    assert_eq!(cpu.sp, 0x4423);
}

#[test]
fn test_run_pushqq() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x22;
    cpu.regs[Reg::F] = 0x33;
    cpu.sp = 0x1007;
    cpu.run_op(Opcode::PUSHQQ(BigReg::AF));
    assert_eq!(cpu.mem[0x1006], 0x22);
    assert_eq!(cpu.mem[0x1005], 0x33);
    assert_eq!(cpu.sp, 0x1005);
}

#[test]
fn test_run_pushix() {
    let mut cpu = Z80::new();
    cpu.ix = 0x2233;
    cpu.sp = 0x1007;
    cpu.run_op(Opcode::PUSHIX);
    assert_eq!(cpu.mem[0x1006], 0x22);
    assert_eq!(cpu.mem[0x1005], 0x33);
    assert_eq!(cpu.sp, 0x1005);
}

#[test]
fn test_run_pushiy() {
    let mut cpu = Z80::new();
    cpu.iy = 0x2233;
    cpu.sp = 0x1007;
    cpu.run_op(Opcode::PUSHIY);
    assert_eq!(cpu.mem[0x1006], 0x22);
    assert_eq!(cpu.mem[0x1005], 0x33);
    assert_eq!(cpu.sp, 0x1005);
}

#[test]
fn test_run_popqq() {
    let mut cpu = Z80::new();
    cpu.mem[0x1006] = 0x22;
    cpu.mem[0x1005] = 0x33;
    cpu.sp = 0x1005;
    cpu.run_op(Opcode::POPQQ(BigReg::AF));
    assert_eq!(cpu.regs[Reg::A], 0x22);
    assert_eq!(cpu.regs[Reg::F], 0x33);
    assert_eq!(cpu.sp, 0x1007);
}

#[test]
fn test_run_popix() {
    let mut cpu = Z80::new();
    cpu.mem[0x1006] = 0x22;
    cpu.mem[0x1005] = 0x33;
    cpu.sp = 0x1005;
    cpu.run_op(Opcode::POPIX);
    assert_eq!(cpu.ix, 0x2233);
    assert_eq!(cpu.sp, 0x1007);
}

#[test]
fn test_run_popiy() {
    let mut cpu = Z80::new();
    cpu.mem[0x1006] = 0x22;
    cpu.mem[0x1005] = 0x33;
    cpu.sp = 0x1005;
    cpu.run_op(Opcode::POPIY);
    assert_eq!(cpu.iy, 0x2233);
    assert_eq!(cpu.sp, 0x1007);
}

#[test]
fn test_run_exdehl() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::D] = 0x28;
    cpu.regs[Reg::E] = 0x22;
    cpu.regs[Reg::H] = 0x49;
    cpu.regs[Reg::L] = 0x9A;
    cpu.run_op(Opcode::EXDEHL);
    assert_eq!(cpu.regs[Reg::D], 0x49);
    assert_eq!(cpu.regs[Reg::E], 0x9A);
    assert_eq!(cpu.regs[Reg::H], 0x28);
    assert_eq!(cpu.regs[Reg::L], 0x22);
}

#[test]
fn test_run_exafaf2() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x99;
    cpu.regs[Reg::F] = 0x00;
    cpu.regs[Reg::A2] = 0x59;
    cpu.regs[Reg::F2] = 0x44;
    cpu.run_op(Opcode::EXAFAF2);
    assert_eq!(cpu.regs[Reg::A], 0x59);
    assert_eq!(cpu.regs[Reg::F], 0x44);
    assert_eq!(cpu.regs[Reg::A2], 0x99);
    assert_eq!(cpu.regs[Reg::F2], 0x00);
}

#[test]
fn test_run_exx() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::B] = 0x44;
    cpu.regs[Reg::C] = 0x5A;
    cpu.regs[Reg::D] = 0x3D;
    cpu.regs[Reg::E] = 0xA2;
    cpu.regs[Reg::H] = 0x88;
    cpu.regs[Reg::L] = 0x59;
    cpu.regs[Reg::B2] = 0x09;
    cpu.regs[Reg::C2] = 0x88;
    cpu.regs[Reg::D2] = 0x93;
    cpu.regs[Reg::E2] = 0x00;
    cpu.regs[Reg::H2] = 0x00;
    cpu.regs[Reg::L2] = 0xE7;
    cpu.run_op(Opcode::EXX);
    assert_eq!(cpu.regs[Reg::B], 0x09);
    assert_eq!(cpu.regs[Reg::C], 0x88);
    assert_eq!(cpu.regs[Reg::D], 0x93);
    assert_eq!(cpu.regs[Reg::E], 0x00);
    assert_eq!(cpu.regs[Reg::H], 0x00);
    assert_eq!(cpu.regs[Reg::L], 0xE7);
    assert_eq!(cpu.regs[Reg::B2], 0x44);
    assert_eq!(cpu.regs[Reg::C2], 0x5A);
    assert_eq!(cpu.regs[Reg::D2], 0x3D);
    assert_eq!(cpu.regs[Reg::E2], 0xA2);
    assert_eq!(cpu.regs[Reg::H2], 0x88);
    assert_eq!(cpu.regs[Reg::L2], 0x59);
}

#[test]
fn test_run_exsphl() {
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x70;
    cpu.regs[Reg::L] = 0x12;
    cpu.sp = 0x8856;
    cpu.mem[0x8856] = 0x11;
    cpu.mem[0x8857] = 0x22;
    cpu.run_op(Opcode::EXSPHL);
    assert_eq!(cpu.regs[Reg::H], 0x22);
    assert_eq!(cpu.regs[Reg::L], 0x11);
    assert_eq!(cpu.mem[0x8856], 0x12);
    assert_eq!(cpu.mem[0x8857], 0x70);
}

#[test]
fn test_run_exspix() {
    let mut cpu = Z80::new();
    cpu.ix  = 0x3988;
    cpu.sp = 0x0100;
    cpu.mem[0x0100] = 0x90;
    cpu.mem[0x0101] = 0x48;
    cpu.run_op(Opcode::EXSPIX);
    assert_eq!(cpu.ix, 0x4890);
    assert_eq!(cpu.mem[0x0100], 0x88);
    assert_eq!(cpu.mem[0x0101], 0x39);
}

#[test]
fn test_run_exspiy() {
    let mut cpu = Z80::new();
    cpu.iy  = 0x3988;
    cpu.sp = 0x0100;
    cpu.mem[0x0100] = 0x90;
    cpu.mem[0x0101] = 0x48;
    cpu.run_op(Opcode::EXSPIY);
    assert_eq!(cpu.iy, 0x4890);
    assert_eq!(cpu.mem[0x0100], 0x88);
    assert_eq!(cpu.mem[0x0101], 0x39);
}

#[test]
fn test_run_ldi() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.mem[0x1111] = 0x88;
    cpu.regs[Reg::D] = 0x22;
    cpu.regs[Reg::E] = 0x22;
    cpu.mem[0x2222] = 0x66;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x07;
    cpu.regs[Reg::F] = 0b11111111;

    cpu.run_op(Opcode::LDI);
    assert_eq!(cpu.mem[0x1111], 0x88);
    assert_eq!(cpu.mem[0x2222], 0x88);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x12);
    assert_eq!(cpu.regs[Reg::D], 0x22);
    assert_eq!(cpu.regs[Reg::E], 0x23);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x06);
    assert_eq!(cpu.regs[Reg::F], 0b11101001);
}

#[test]
fn test_run_ldir() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.mem[0x1111] = 0x88;
    cpu.mem[0x1112] = 0x36;
    cpu.mem[0x1113] = 0x5A;
    cpu.regs[Reg::D] = 0x22;
    cpu.regs[Reg::E] = 0x22;
    cpu.mem[0x2222] = 0x66;
    cpu.mem[0x2223] = 0x59;
    cpu.mem[0x2224] = 0xC5;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x03;
    cpu.regs[Reg::F] = 0b11111111;

    cpu.run_op(Opcode::LDIR);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x14);
    assert_eq!(cpu.regs[Reg::D], 0x22);
    assert_eq!(cpu.regs[Reg::E], 0x25);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x00);
    assert_eq!(cpu.mem[0x1111], 0x88);
    assert_eq!(cpu.mem[0x1112], 0x36);
    assert_eq!(cpu.mem[0x1113], 0x5A);
    assert_eq!(cpu.mem[0x2222], 0x88);
    assert_eq!(cpu.mem[0x2223], 0x36);
    assert_eq!(cpu.mem[0x2224], 0x5A);
    assert_eq!(cpu.regs[Reg::F], 0b11101001);
}

#[test]
fn test_run_ldd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.mem[0x1111] = 0x88;
    cpu.regs[Reg::D] = 0x22;
    cpu.regs[Reg::E] = 0x22;
    cpu.mem[0x2222] = 0x66;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x07;

    cpu.run_op(Opcode::LDD);
    assert_eq!(cpu.mem[0x1111], 0x88);
    assert_eq!(cpu.mem[0x2222], 0x88);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x10);
    assert_eq!(cpu.regs[Reg::D], 0x22);
    assert_eq!(cpu.regs[Reg::E], 0x21);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x06);
}

#[test]
fn test_run_lddr() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x14;
    cpu.mem[0x1112] = 0x88;
    cpu.mem[0x1113] = 0x36;
    cpu.mem[0x1114] = 0x5A;
    cpu.regs[Reg::D] = 0x22;
    cpu.regs[Reg::E] = 0x25;
    cpu.mem[0x2223] = 0x66;
    cpu.mem[0x2224] = 0x59;
    cpu.mem[0x2225] = 0xC5;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x03;

    cpu.run_op(Opcode::LDDR);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x11);
    assert_eq!(cpu.regs[Reg::D], 0x22);
    assert_eq!(cpu.regs[Reg::E], 0x22);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x00);
    assert_eq!(cpu.mem[0x1112], 0x88);
    assert_eq!(cpu.mem[0x1113], 0x36);
    assert_eq!(cpu.mem[0x1114], 0x5A);
    assert_eq!(cpu.mem[0x2223], 0x88);
    assert_eq!(cpu.mem[0x2224], 0x36);
    assert_eq!(cpu.mem[0x2225], 0x5A);
}

#[test]
fn test_run_cpi() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.mem[0x1111] = 0x3B;
    cpu.regs[Reg::A] = 0x3B;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x01;

    cpu.run_op(Opcode::CPI);
    assert_eq!(cpu.mem[0x1111], 0x3B);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x12);
    assert_eq!(cpu.regs[Reg::A], 0x3B);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x00);
}

#[test]
fn test_run_cpir() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.regs[Reg::A] = 0xF3;
    cpu.mem[0x1111] = 0x52;
    cpu.mem[0x1112] = 0x00;
    cpu.mem[0x1113] = 0xF3;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x07;

    cpu.run_op(Opcode::CPIR);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x14);
    assert_eq!(cpu.regs[Reg::A], 0xF3);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x04);
    assert_eq!(cpu.mem[0x1111], 0x52);
    assert_eq!(cpu.mem[0x1112], 0x00);
    assert_eq!(cpu.mem[0x1113], 0xF3);
}

#[test]
fn test_run_cpd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x11;
    cpu.mem[0x1111] = 0x3B;
    cpu.regs[Reg::A] = 0x3B;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x01;

    cpu.run_op(Opcode::CPD);
    assert_eq!(cpu.mem[0x1111], 0x3B);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x10);
    assert_eq!(cpu.regs[Reg::A], 0x3B);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x00);
}

#[test]
fn test_run_cpdr() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x11;
    cpu.regs[Reg::L] = 0x18;
    cpu.regs[Reg::A] = 0xF3;
    cpu.mem[0x1118] = 0x52;
    cpu.mem[0x1117] = 0x00;
    cpu.mem[0x1116] = 0xF3;
    cpu.regs[Reg::B] = 0x00;
    cpu.regs[Reg::C] = 0x07;

    cpu.run_op(Opcode::CPDR);
    assert_eq!(cpu.regs[Reg::H], 0x11);
    assert_eq!(cpu.regs[Reg::L], 0x15);
    assert_eq!(cpu.regs[Reg::A], 0xF3);
    assert_eq!(cpu.regs[Reg::B], 0x00);
    assert_eq!(cpu.regs[Reg::C], 0x04);
    assert_eq!(cpu.mem[0x1118], 0x52);
    assert_eq!(cpu.mem[0x1117], 0x00);
    assert_eq!(cpu.mem[0x1116], 0xF3);
}

#[test]
fn test_run_addar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.regs[Reg::B] = 0x11;
    cpu.run_op(Opcode::ADDAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0x55);
    assert_eq!(cpu.regs[Reg::B], 0x11);
}

#[test]
fn test_run_addan() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.run_op(Opcode::ADDAN(0x11));
    assert_eq!(cpu.regs[Reg::A], 0x55);
}

#[test]
fn test_run_addahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xA0;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x08;
    cpu.run_op(Opcode::ADDAHL);
    assert_eq!(cpu.regs[Reg::A], 0xA8);
}

#[test]
fn test_run_addaixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x11;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x22;
    cpu.run_op(Opcode::ADDAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x33);
}

#[test]
fn test_run_addaiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x11;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x22;
    cpu.run_op(Opcode::ADDAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x33);
}

#[test]
fn test_run_subar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.regs[Reg::B] = 0x11;
    cpu.run_op(Opcode::SUBAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0x33);
    assert_eq!(cpu.regs[Reg::B], 0x11);
}

#[test]
fn test_run_suban() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.run_op(Opcode::SUBAN(0x11));
    assert_eq!(cpu.regs[Reg::A], 0x33);
}

#[test]
fn test_run_subahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xA8;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x08;
    cpu.run_op(Opcode::SUBAHL);
    assert_eq!(cpu.regs[Reg::A], 0xA0);
}

#[test]
fn test_run_subaixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x33;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x11;
    cpu.run_op(Opcode::SUBAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x22);
}

#[test]
fn test_run_subaiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x33;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x11;
    cpu.run_op(Opcode::SUBAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x22);
}

#[test]
fn test_run_sbcar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.regs[Reg::F] = 0b00000001;
    cpu.regs[Reg::B] = 0x11;
    cpu.run_op(Opcode::SBCAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0x32);
    assert_eq!(cpu.regs[Reg::B], 0x11);
}

#[test]
fn test_run_sbcan() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x44;
    cpu.regs[Reg::F] = 0b00000001;
    cpu.run_op(Opcode::SBCAN(0x11));
    assert_eq!(cpu.regs[Reg::A], 0x32);
}

#[test]
fn test_run_sbcahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xA8;
    cpu.regs[Reg::F] = 0b00000001;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x08;
    cpu.run_op(Opcode::SBCAHL);
    assert_eq!(cpu.regs[Reg::A], 0x9F);
}

#[test]
fn test_run_sbcaixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x33;
    cpu.regs[Reg::F] = 0b00000001;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x11;
    cpu.run_op(Opcode::SBCAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x21);
}

#[test]
fn test_run_sbcaiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x33;
    cpu.regs[Reg::F] = 0b00000001;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x11;
    cpu.run_op(Opcode::SBCAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x21);
}

#[test]
fn test_run_andar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xC3;
    cpu.regs[Reg::B] = 0x7B;
    cpu.run_op(Opcode::ANDAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0x43);
}

#[test]
fn test_run_andan() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xC3;
    cpu.run_op(Opcode::ANDAN(0x7B));
    assert_eq!(cpu.regs[Reg::A], 0x43);
}

#[test]
fn test_run_andahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xC3;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x7B;
    cpu.run_op(Opcode::ANDAHL);
    assert_eq!(cpu.regs[Reg::A], 0x43);
}

#[test]
fn test_run_andaixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xC3;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x7B;
    cpu.run_op(Opcode::ANDAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x43);
}

#[test]
fn test_run_andaiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0xC3;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x7B;
    cpu.run_op(Opcode::ANDAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x43);
}

#[test]
fn test_run_orar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x12;
    cpu.regs[Reg::B] = 0x48;
    cpu.run_op(Opcode::ORAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0x5A);
}

#[test]
fn test_run_oran() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x12;
    cpu.run_op(Opcode::ORAN(0x48));
    assert_eq!(cpu.regs[Reg::A], 0x5A);
}

#[test]
fn test_run_orahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x12;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x48;
    cpu.run_op(Opcode::ORAHL);
    assert_eq!(cpu.regs[Reg::A], 0x5A);
}

#[test]
fn test_run_oraixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x12;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x48;
    cpu.run_op(Opcode::ORAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x5A);
}

#[test]
fn test_run_oraiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x12;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x48;
    cpu.run_op(Opcode::ORAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0x5A);
}

#[test]
fn test_run_xorar() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x96;
    cpu.regs[Reg::B] = 0x5D;
    cpu.run_op(Opcode::XORAR(Reg::B));
    assert_eq!(cpu.regs[Reg::A], 0xCB);
}

#[test]
fn test_run_xoran() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x96;
    cpu.run_op(Opcode::XORAN(0x5D));
    assert_eq!(cpu.regs[Reg::A], 0xCB);
}

#[test]
fn test_run_xorahl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x96;
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x5D;
    cpu.run_op(Opcode::XORAHL);
    assert_eq!(cpu.regs[Reg::A], 0xCB);
}

#[test]
fn test_run_xoraixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x96;
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::XORAIXD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0xCB);
}

#[test]
fn test_run_xoraiyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::A] = 0x96;
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::XORAIYD(0x5));
    assert_eq!(cpu.regs[Reg::A], 0xCB);
}

#[test]
fn test_run_incr() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::B] = 0x5D;
    cpu.run_op(Opcode::INCR(Reg::B));
    assert_eq!(cpu.regs[Reg::B], 0x5E);
}

#[test]
fn test_run_inchl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x5D;
    cpu.run_op(Opcode::INCHL);
    assert_eq!(cpu.mem[0x2323], 0x5E);
}

#[test]
fn test_run_incixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::INCIXD(0x5));
    assert_eq!(cpu.mem[0x1005], 0x5E);
}

#[test]
fn test_run_inciyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::INCIYD(0x5));
    assert_eq!(cpu.mem[0x1005], 0x5E);
}

#[test]
fn test_run_decr() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::B] = 0x5D;
    cpu.run_op(Opcode::DECR(Reg::B));
    assert_eq!(cpu.regs[Reg::B], 0x5C);
}

#[test]
fn test_run_dechl() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.regs[Reg::H] = 0x23;
    cpu.regs[Reg::L] = 0x23;
    cpu.mem[0x2323] = 0x5D;
    cpu.run_op(Opcode::DECHL);
    assert_eq!(cpu.mem[0x2323], 0x5C);
}

#[test]
fn test_run_decixd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.ix = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::DECIXD(0x5));
    assert_eq!(cpu.mem[0x1005], 0x5C);
}

#[test]
fn test_run_deciyd() {
    // TODO: Review the "Condition Bits Affected" from z80 user manual
    let mut cpu = Z80::new();
    cpu.iy = 0x1000;
    cpu.mem[0x1005] = 0x5D;
    cpu.run_op(Opcode::DECIYD(0x5));
    assert_eq!(cpu.mem[0x1005], 0x5C);
}
