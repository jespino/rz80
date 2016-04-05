use ops::Opcode;
use ops::Reg;
use ops::BigReg;
use std::ops::Index;
use std::ops::IndexMut;

type Memory = [u8; 65536];

impl Index<Reg> for [u8] {
    type Output = u8;

    fn index(&self, index: Reg) -> &u8 {
        &self[index as usize]
    }
}

impl IndexMut<Reg> for [u8] {
    fn index_mut(&mut self, index: Reg) -> &mut u8 {
        &mut self[index as usize]
    }
}

struct Z80 {
    regs: [u8; 16],

    i: u8,
    r: u8,
    ix: u16,
    iy: u16,
    sp: u16,
    pc: u16,

    mem: Memory,
}

impl Z80 {
    fn new() -> Z80 {
        Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        }
    }

    fn get_reg_pair(&self, reg1: Reg, reg2: Reg) -> u16 {
        ((self.regs[reg1] as u16) << 8) + self.regs[reg2] as u16
    }

    fn get_big_reg(&self, reg: BigReg) -> u16 {
        match reg {
            BigReg::BC => self.get_reg_pair(Reg::B, Reg::C),
            BigReg::DE => self.get_reg_pair(Reg::D, Reg::E),
            BigReg::HL => self.get_reg_pair(Reg::H, Reg::L),
            BigReg::SP => self.sp,
            BigReg::IX => self.ix,
            BigReg::IY => self.iy,
            BigReg::AF => self.get_reg_pair(Reg::A, Reg::F),
        }
    }

    fn set_reg_pair(&mut self, reg1: Reg, reg2: Reg, value: u16) {
        self.regs[reg1] = (value & 0xFF00).wrapping_shr(8) as u8;
        self.regs[reg2] = (value & 0x00FF) as u8;
    }

    fn set_big_reg(&mut self, reg: BigReg, value: u16) {
        match reg {
            BigReg::BC => self.set_reg_pair(Reg::B, Reg::C, value),
            BigReg::DE => self.set_reg_pair(Reg::D, Reg::E, value),
            BigReg::HL => self.set_reg_pair(Reg::H, Reg::L, value),
            BigReg::SP => self.sp = value,
            BigReg::IX => self.ix = value,
            BigReg::IY => self.iy = value,
            BigReg::AF => self.set_reg_pair(Reg::A, Reg::F, value),
        };
    }

    fn get_mem_u16(&mut self, address: u16) -> u16 {
        ((self.mem[address as usize] as u16) << 8) + self.mem[(address + 1) as usize] as u16
    }

    fn set_mem_u16(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = (value & 0xFF00).wrapping_shr(8) as u8;
        self.mem[(address + 1) as usize] = (value & 0x00FF) as u8;
    }

    fn flip_u16(&self, value: u16) -> u16 {
        (value & 0xFF00).wrapping_shr(8) + (value & 0x00FF).wrapping_shl(8)
    }

    fn run_op(&mut self, op: Opcode) {
        match op {
            Opcode::LDRR(reg1, reg2) => self.regs[reg1] = self.regs[reg2],
            Opcode::LDRN(reg1, value) => self.regs[reg1] = value,
            Opcode::LDRHL(reg1) => {
                let idx = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[reg1] = self.mem[idx as usize];
            },
            Opcode::LDRIXD(reg1, displacement) => {
                let idx = self.ix + displacement as u16;
                self.regs[reg1] = self.mem[idx as usize];
            },
            Opcode::LDRIYD(reg1, displacement) => {
                let idx = self.iy + displacement as u16;
                self.regs[reg1] = self.mem[idx as usize];
            },
            Opcode::LDHLR(reg1) => {
                let idx = self.get_reg_pair(Reg::H, Reg::L);
                self.mem[idx as usize] = self.regs[reg1];
            },
            Opcode::LDIXDR(displacement, reg1) => {
                let idx = self.ix + displacement as u16;
                self.mem[idx as usize] = self.regs[reg1];
            },
            Opcode::LDIYDR(displacement, reg1) => {
                let idx = self.iy + displacement as u16;
                self.mem[idx as usize] = self.regs[reg1];
            },
            Opcode::LDHLN(value) => {
                let idx = self.get_reg_pair(Reg::H, Reg::L);
                self.mem[idx as usize] = value;
            },
            Opcode::LDIXDN(displacement, value) => {
                let idx = self.ix + displacement as u16;
                self.mem[idx as usize] = value;
            },
            Opcode::LDIYDN(displacement, value) => {
                let idx = self.iy + displacement as u16;
                self.mem[idx as usize] = value;
            },
            Opcode::LDABC => {
                let idx = self.get_reg_pair(Reg::B, Reg::C);
                self.regs[Reg::A] = self.mem[idx as usize];
            },
            Opcode::LDADE => {
                let idx = self.get_reg_pair(Reg::D, Reg::E);
                self.regs[Reg::A] = self.mem[idx as usize];
            },
            Opcode::LDANN(idx) => self.regs[Reg::A] = self.mem[idx as usize],
            Opcode::LDBCA => {
                let idx = self.get_reg_pair(Reg::B, Reg::C);
                self.mem[idx as usize] = self.regs[Reg::A];
            },
            Opcode::LDDEA => {
                let idx = self.get_reg_pair(Reg::D, Reg::E);
                self.mem[idx as usize] = self.regs[Reg::A];
            },
            Opcode::LDNNA(idx) => self.mem[idx as usize] = self.regs[Reg::A],
            Opcode::LDAI => self.regs[Reg::A] = self.i,
            Opcode::LDAR => self.regs[Reg::A] = self.r,
            Opcode::LDIA => self.i = self.regs[Reg::A],
            Opcode::LDRA => self.r = self.regs[Reg::A],
            Opcode::LDDDNN(big_reg, value) => self.set_big_reg(big_reg, value),
            Opcode::LDIXNN(value) => self.ix = value,
            Opcode::LDIYNN(value) => self.iy = value,
            Opcode::LDHLNN(address) => {
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.set_reg_pair(Reg::H, Reg::L, value);
            },
            Opcode::LDDDNN2(big_reg, address) => {
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.set_big_reg(big_reg, value);
            },
            Opcode::LDIXNN2(address) => {
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.ix = value;
            },
            Opcode::LDIYNN2(address) => {
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.iy = value;
            },
            Opcode::LDNNHL(address) => {
                let mut value = self.get_reg_pair(Reg::H, Reg::L);
                value = self.flip_u16(value);
                self.set_mem_u16(address, value);
            },
            Opcode::LDNNDD(address, big_reg) => {
                let mut value = self.get_big_reg(big_reg);
                value = self.flip_u16(value);
                self.set_mem_u16(address, value);
            },
            Opcode::LDNNIX(address) => {
                let mut value = self.ix;
                value = self.flip_u16(value);
                self.set_mem_u16(address, value);
            },
            Opcode::LDNNIY(address) => {
                let mut value = self.iy;
                value = self.flip_u16(value);
                self.set_mem_u16(address, value);
            },
            Opcode::LDSPHL => self.sp = self.get_reg_pair(Reg::H, Reg::L),
            Opcode::LDSPIX => self.sp = self.ix,
            Opcode::LDSPIY => self.sp = self.iy,
            Opcode::PUSHQQ(big_reg) => {
                let mut value = self.get_big_reg(big_reg);
                value = self.flip_u16(value);
                self.sp -= 2;
                let address = self.sp;
                self.set_mem_u16(address, value);
            },
            Opcode::PUSHIX => {
                let mut value = self.ix;
                value = self.flip_u16(value);
                self.sp -= 2;
                let address = self.sp;
                self.set_mem_u16(address, value);
            },
            Opcode::PUSHIY => {
                let mut value = self.iy;
                value = self.flip_u16(value);
                self.sp -= 2;
                let address = self.sp;
                self.set_mem_u16(address, value);
            },
            Opcode::POPQQ(big_reg) => {
                let address = self.sp;
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.sp += 2;
                self.set_big_reg(big_reg, value);
            },
            Opcode::POPIX => {
                let address = self.sp;
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.sp += 2;
                self.ix = value;
            },
            Opcode::POPIY => {
                let address = self.sp;
                let mut value = self.get_mem_u16(address);
                value = self.flip_u16(value);
                self.sp += 2;
                self.iy = value;
            },
            _ => ()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Z80;
    use ops::Opcode;
    use ops::Reg;
    use ops::BigReg;

    #[test]
    fn test_run_ldrr() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x8A;
        cpu.regs[Reg::E] = 0x10;
        cpu.run_op(Opcode::LDRR(Reg::H, Reg::E));
        assert!(cpu.regs[Reg::H] == 0x10);
        assert!(cpu.regs[Reg::E] == 0x10);
    }

    #[test]
    fn test_run_ldrn() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::E] = 0x8A;
        cpu.run_op(Opcode::LDRN(Reg::E, 0x20));
        assert!(cpu.regs[Reg::E] == 0x20);
    }

    #[test]
    fn test_run_ldrhl() {
        let mut cpu = Z80::new();
        cpu.mem[0x75A1] = 0x58;
        cpu.regs[Reg::C] = 0;
        cpu.regs[Reg::H] = 0x75;
        cpu.regs[Reg::L] = 0xA1;
        cpu.run_op(Opcode::LDRHL(Reg::C));
        assert!(cpu.regs[Reg::C] == 0x58);
    }

    #[test]
    fn test_run_ldrixd() {
        let mut cpu = Z80::new();
        cpu.mem[0x25C8] = 0x39;
        cpu.ix = 0x25AF;
        cpu.regs[Reg::B] = 0;
        cpu.run_op(Opcode::LDRIXD(Reg::B, 0x19));
        assert!(cpu.regs[Reg::B] == 0x39);
    }

    #[test]
    fn test_run_ldriyd() {
        let mut cpu = Z80::new();
        cpu.mem[0x25C8] = 0x39;
        cpu.iy = 0x25AF;
        cpu.regs[Reg::B] = 0;
        cpu.run_op(Opcode::LDRIYD(Reg::B, 0x19));
        assert!(cpu.regs[Reg::B] == 0x39);
    }

    #[test]
    fn test_run_ldhlr() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::B] = 0x29;
        cpu.regs[Reg::H] = 0x21;
        cpu.regs[Reg::L] = 0x46;
        cpu.run_op(Opcode::LDHLR(Reg::B));
        assert!(cpu.mem[0x2146] == 0x29);
    }

    #[test]
    fn test_run_ldixdr() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::C] = 0x1C;
        cpu.ix = 0x3100;
        cpu.run_op(Opcode::LDIXDR(0x6, Reg::C));
        assert!(cpu.mem[0x3106] == 0x1C);
    }

    #[test]
    fn test_run_ldiydr() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::C] = 0x48;
        cpu.iy = 0x2A11;
        cpu.run_op(Opcode::LDIYDR(0x4, Reg::C));
        assert!(cpu.mem[0x2A15] == 0x48);
    }

    #[test]
    fn test_run_ldhln() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x44;
        cpu.regs[Reg::L] = 0x44;
        cpu.run_op(Opcode::LDHLN(0x28));
        assert!(cpu.mem[0x4444] == 0x28);
    }

    #[test]
    fn test_run_ldixdn() {
        let mut cpu = Z80::new();
        cpu.ix = 0xA940;
        cpu.run_op(Opcode::LDIXDN(0x10, 0x97));
        assert!(cpu.mem[0xA950] == 0x97);
    }

    #[test]
    fn test_run_ldiydn() {
        let mut cpu = Z80::new();
        cpu.iy = 0xA940;
        cpu.run_op(Opcode::LDIYDN(0x10, 0x97));
        assert!(cpu.mem[0xA950] == 0x97);
    }

    #[test]
    fn test_run_ldabc() {
        let mut cpu = Z80::new();
        cpu.mem[0x4747] = 0x12;
        cpu.regs[Reg::B] = 0x47;
        cpu.regs[Reg::C] = 0x47;
        cpu.run_op(Opcode::LDABC);
        assert!(cpu.regs[Reg::A] == 0x12);
    }

    #[test]
    fn test_run_ldade() {
        let mut cpu = Z80::new();
        cpu.mem[0x30A2] = 0x22;
        cpu.regs[Reg::D] = 0x30;
        cpu.regs[Reg::E] = 0xA2;
        cpu.run_op(Opcode::LDADE);
        assert!(cpu.regs[Reg::A] == 0x22);
    }

    #[test]
    fn test_run_ldann() {
        let mut cpu = Z80::new();
        cpu.mem[0x8832] = 0x4;
        cpu.run_op(Opcode::LDANN(0x8832));
        assert!(cpu.regs[Reg::A] == 0x4);
    }

    #[test]
    fn test_run_ldbca() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0x7A;
        cpu.regs[Reg::B] = 0x12;
        cpu.regs[Reg::C] = 0x12;
        cpu.run_op(Opcode::LDBCA);
        assert!(cpu.mem[0x1212] == 0x7A);
    }

    #[test]
    fn test_run_lddea() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0xA0;
        cpu.regs[Reg::D] = 0x11;
        cpu.regs[Reg::E] = 0x28;
        cpu.run_op(Opcode::LDDEA);
        assert!(cpu.mem[0x1128] == 0xA0);
    }

    #[test]
    fn test_run_ldnna() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0xD7;
        cpu.run_op(Opcode::LDNNA(0x3141));
        assert!(cpu.mem[0x3141] == 0xD7);
    }

    #[test]
    fn test_run_ldai() {
        // TODO: Review the "Condition Bits Affected" from z80 user manual
        let mut cpu = Z80::new();
        cpu.i = 0xD7;
        cpu.run_op(Opcode::LDAI);
        assert!(cpu.regs[Reg::A] == 0xD7);
    }

    #[test]
    fn test_run_ldar() {
        // TODO: Review the "Condition Bits Affected" from z80 user manual
        let mut cpu = Z80::new();
        cpu.r = 0xD7;
        cpu.run_op(Opcode::LDAR);
        assert!(cpu.regs[Reg::A] == 0xD7);
    }

    #[test]
    fn test_run_ldia() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0xD7;
        cpu.run_op(Opcode::LDIA);
        assert!(cpu.i == 0xD7);
    }

    #[test]
    fn test_run_ldra() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0xD7;
        cpu.run_op(Opcode::LDRA);
        assert!(cpu.r == 0xD7);
    }

    #[test]
    fn test_run_ldddnn() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x99;
        cpu.regs[Reg::L] = 0x99;
        cpu.run_op(Opcode::LDDDNN(BigReg::HL, 0x5000));
        assert!(cpu.regs[Reg::H] == 0x50);
        assert!(cpu.regs[Reg::L] == 0x00);
    }

    #[test]
    fn test_run_ldixnn() {
        let mut cpu = Z80::new();
        cpu.run_op(Opcode::LDIXNN(0x45A2));
        assert!(cpu.ix == 0x45A2);
    }

    #[test]
    fn test_run_ldiynn() {
        let mut cpu = Z80::new();
        cpu.run_op(Opcode::LDIYNN(0x45A2));
        assert!(cpu.iy == 0x45A2);
    }

    #[test]
    fn test_run_ldhlnn() {
        let mut cpu = Z80::new();
        cpu.mem[0x4545] = 0x37;
        cpu.mem[0x4546] = 0xA1;
        cpu.run_op(Opcode::LDHLNN(0x4545));
        assert!(cpu.regs[Reg::H] == 0xA1);
        assert!(cpu.regs[Reg::L] == 0x37);
    }

    #[test]
    fn test_run_ldddnn2() {
        let mut cpu = Z80::new();
        cpu.mem[0x2130] = 0x65;
        cpu.mem[0x2131] = 0x78;
        cpu.run_op(Opcode::LDDDNN2(BigReg::HL, 0x2130));
        assert!(cpu.regs[Reg::H] == 0x78);
        assert!(cpu.regs[Reg::L] == 0x65);
    }

    #[test]
    fn test_run_ldixnn2() {
        let mut cpu = Z80::new();
        cpu.mem[0x6666] = 0x92;
        cpu.mem[0x6667] = 0xDA;
        cpu.run_op(Opcode::LDIXNN2(0x6666));
        assert!(cpu.ix == 0xDA92);
    }

    #[test]
    fn test_run_ldiynn2() {
        let mut cpu = Z80::new();
        cpu.mem[0x6666] = 0x92;
        cpu.mem[0x6667] = 0xDA;
        cpu.run_op(Opcode::LDIYNN2(0x6666));
        assert!(cpu.iy == 0xDA92);
    }

    #[test]
    fn test_run_ldnnhl() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x48;
        cpu.regs[Reg::L] = 0x3A;
        cpu.run_op(Opcode::LDNNHL(0xB229));
        assert!(cpu.mem[0xB229] == 0x3A);
        assert!(cpu.mem[0xB22A] == 0x48);
    }

    #[test]
    fn test_run_ldnndd() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x48;
        cpu.regs[Reg::L] = 0x3A;
        cpu.run_op(Opcode::LDNNDD(0xB229, BigReg::HL));
        assert!(cpu.mem[0xB229] == 0x3A);
        assert!(cpu.mem[0xB22A] == 0x48);
    }

    #[test]
    fn test_run_ldnnix() {
        let mut cpu = Z80::new();
        cpu.ix = 0x5A30;
        cpu.run_op(Opcode::LDNNIX(0x4392));
        assert!(cpu.mem[0x4392] == 0x30);
        assert!(cpu.mem[0x4393] == 0x5A);
    }

    #[test]
    fn test_run_ldnniy() {
        let mut cpu = Z80::new();
        cpu.iy = 0x5A30;
        cpu.run_op(Opcode::LDNNIY(0x4392));
        assert!(cpu.mem[0x4392] == 0x30);
        assert!(cpu.mem[0x4393] == 0x5A);
    }

    #[test]
    fn test_run_ldsphl() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::H] = 0x44;
        cpu.regs[Reg::L] = 0x23;
        cpu.run_op(Opcode::LDSPHL);
        assert!(cpu.sp == 0x4423);
    }

    #[test]
    fn test_run_ldspix() {
        let mut cpu = Z80::new();
        cpu.ix = 0x4423;
        cpu.run_op(Opcode::LDSPIX);
        assert!(cpu.sp == 0x4423);
    }

    #[test]
    fn test_run_ldspiy() {
        let mut cpu = Z80::new();
        cpu.iy = 0x4423;
        cpu.run_op(Opcode::LDSPIY);
        assert!(cpu.sp == 0x4423);
    }

    #[test]
    fn test_run_pushqq() {
        let mut cpu = Z80::new();
        cpu.regs[Reg::A] = 0x22;
        cpu.regs[Reg::F] = 0x33;
        cpu.sp = 0x1007;
        cpu.run_op(Opcode::PUSHQQ(BigReg::AF));
        assert!(cpu.mem[0x1006] == 0x22);
        assert!(cpu.mem[0x1005] == 0x33);
        assert!(cpu.sp == 0x1005);
    }

    #[test]
    fn test_run_pushix() {
        let mut cpu = Z80::new();
        cpu.ix = 0x2233;
        cpu.sp = 0x1007;
        cpu.run_op(Opcode::PUSHIX);
        assert!(cpu.mem[0x1006] == 0x22);
        assert!(cpu.mem[0x1005] == 0x33);
        assert!(cpu.sp == 0x1005);
    }

    #[test]
    fn test_run_pushiy() {
        let mut cpu = Z80::new();
        cpu.iy = 0x2233;
        cpu.sp = 0x1007;
        cpu.run_op(Opcode::PUSHIY);
        assert!(cpu.mem[0x1006] == 0x22);
        assert!(cpu.mem[0x1005] == 0x33);
        assert!(cpu.sp == 0x1005);
    }

    #[test]
    fn test_run_popqq() {
        let mut cpu = Z80::new();
        cpu.mem[0x1006] = 0x22;
        cpu.mem[0x1005] = 0x33;
        cpu.sp = 0x1005;
        cpu.run_op(Opcode::POPQQ(BigReg::AF));
        assert!(cpu.regs[Reg::A] == 0x22);
        assert!(cpu.regs[Reg::F] == 0x33);
        assert!(cpu.sp == 0x1007);
    }

    #[test]
    fn test_run_popix() {
        let mut cpu = Z80::new();
        cpu.mem[0x1006] = 0x22;
        cpu.mem[0x1005] = 0x33;
        cpu.sp = 0x1005;
        cpu.run_op(Opcode::POPIX);
        assert!(cpu.ix == 0x2233);
        assert!(cpu.sp == 0x1007);
    }

    #[test]
    fn test_run_popiy() {
        let mut cpu = Z80::new();
        cpu.mem[0x1006] = 0x22;
        cpu.mem[0x1005] = 0x33;
        cpu.sp = 0x1005;
        cpu.run_op(Opcode::POPIY);
        assert!(cpu.iy == 0x2233);
        assert!(cpu.sp == 0x1007);
    }
}
