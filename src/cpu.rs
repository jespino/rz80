use ops::Opcode;
use ops::Reg;
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
    fn run_op(&mut self, op: Opcode) {
        match op {
            Opcode::LDRR(reg1, reg2) => self.regs[reg1] = self.regs[reg2],
            Opcode::LDRN(reg1, value) => self.regs[reg1] = value,
            Opcode::LDRHL(reg1) => {
                let idx = ((self.regs[Reg::H] as u16) << 8) + self.regs[Reg::L] as u16;
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
                let idx = ((self.regs[Reg::H] as u16) << 8) + self.regs[Reg::L] as u16;
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
                let idx = ((self.regs[Reg::H] as u16) << 8) + self.regs[Reg::L] as u16;
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
                let idx = ((self.regs[Reg::B] as u16) << 8) + self.regs[Reg::C] as u16;
                self.regs[Reg::A] = self.mem[idx as usize];
            },
            Opcode::LDADE => {
                let idx = ((self.regs[Reg::D] as u16) << 8) + self.regs[Reg::E] as u16;
                self.regs[Reg::A] = self.mem[idx as usize];
            },
            Opcode::LDANN(idx) => self.regs[Reg::A] = self.mem[idx as usize],
            Opcode::LDBCA => {
                let idx = ((self.regs[Reg::B] as u16) << 8) + self.regs[Reg::C] as u16;
                self.mem[idx as usize] = self.regs[Reg::A];
            },
            Opcode::LDDEA => {
                let idx = ((self.regs[Reg::D] as u16) << 8) + self.regs[Reg::E] as u16;
                self.mem[idx as usize] = self.regs[Reg::A];
            },
            Opcode::LDNNA(idx) => self.mem[idx as usize] = self.regs[Reg::A],
            Opcode::LDAI => self.regs[Reg::A] = self.i,
            Opcode::LDAR => self.regs[Reg::A] = self.r,
            Opcode::LDIA => self.i = self.regs[Reg::A],
            Opcode::LDRA => self.r = self.regs[Reg::A],
            _ => ()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Z80;
    use ops::Opcode;
    use ops::Reg;

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
}
