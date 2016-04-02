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
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.regs[Reg::H] = 0x8A;
        cpu.regs[Reg::E] = 0x10;
        cpu.run_op(Opcode::LDRR(Reg::H, Reg::E));
        assert!(cpu.regs[Reg::H] == 0x10);
        assert!(cpu.regs[Reg::E] == 0x10);
    }

    #[test]
    fn test_run_ldrn() {
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.regs[Reg::E] = 0x8A;
        cpu.run_op(Opcode::LDRN(Reg::E, 0x20));
        assert!(cpu.regs[Reg::E] == 0x20);
    }

    #[test]
    fn test_run_ldrhl() {
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.mem[0x75A1] = 0x58;
        cpu.regs[Reg::C] = 0;
        cpu.regs[Reg::H] = 0x75;
        cpu.regs[Reg::L] = 0xA1;
        cpu.run_op(Opcode::LDRHL(Reg::C));
        assert!(cpu.regs[Reg::C] == 0x58);
    }

    #[test]
    fn test_run_ldrixd() {
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.mem[0x25C8] = 0x39;
        cpu.ix = 0x25AF;
        cpu.regs[Reg::B] = 0;
        cpu.run_op(Opcode::LDRIXD(Reg::B, 0x19));
        assert!(cpu.regs[Reg::B] == 0x39);
    }

    #[test]
    fn test_run_ldriyd() {
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.mem[0x25C8] = 0x39;
        cpu.iy = 0x25AF;
        cpu.regs[Reg::B] = 0;
        cpu.run_op(Opcode::LDRIYD(Reg::B, 0x19));
        assert!(cpu.regs[Reg::B] == 0x39);
    }

    #[test]
    fn test_run_ldhlr() {
        let mut cpu = Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536]
        };
        cpu.mem[0x2146] = 0;
        cpu.regs[Reg::B] = 0x29;
        cpu.regs[Reg::H] = 0x21;
        cpu.regs[Reg::L] = 0x46;
        cpu.run_op(Opcode::LDHLR(Reg::B));
        assert!(cpu.mem[0x2146] == 0x29);
    }
}
