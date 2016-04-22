mod tests;

use ops::opcodes::Opcode;
use ops::opcodes::Reg;
use ops::opcodes::BigReg;
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

    iff1: bool,
    iff2: bool,
}

impl Z80 {
    fn new() -> Z80 {
        Z80 {
            regs: [0; 16],
            i: 0, r: 0, ix: 0, iy: 0, sp: 0, pc:0,
            mem: [0;65536],
            iff1: false, iff2: false,
        }
    }

    fn set_carry(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b00000001; }
        else { self.regs[Reg::F] &= 0b11111110; }
    }

    fn set_add_subtract(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b00000010; }
        else { self.regs[Reg::F] &= 0b11111101; }
    }

    fn set_parity_overflow(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b00000100; }
        else { self.regs[Reg::F] &= 0b11111011; }
    }

    fn set_half_carry(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b00010000; }
        else { self.regs[Reg::F] &= 0b11101111; }
    }

    fn set_zero(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b01000000; }
        else { self.regs[Reg::F] &= 0b10111111; }
    }

    fn set_sign(&mut self, value: bool) {
        if value { self.regs[Reg::F] |= 0b10000000; }
        else { self.regs[Reg::F] &= 0b01111111; }
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
            Opcode::LDAI => {
                self.regs[Reg::A] = self.i;
                if (self.i & 0b10000000) > 0 { self.set_sign(true); }
                if self.i == 0 { self.set_zero(true); }
                self.set_half_carry(false);
                self.set_add_subtract(false);
                let iff = self.iff2;
                self.set_parity_overflow(iff);
            },
            Opcode::LDAR => {
                self.regs[Reg::A] = self.r;
                if (self.r & 0b10000000) > 0 { self.set_sign(true); }
                if self.r == 0 { self.set_zero(true); }
                self.set_half_carry(false);
                self.set_add_subtract(false);
                let iff = self.iff2;
                self.set_parity_overflow(iff);
            },
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
            Opcode::EXDEHL => {
                let d = self.regs[Reg::D];
                self.regs[Reg::D] = self.regs[Reg::H];
                self.regs[Reg::H] = d;
                let e = self.regs[Reg::E];
                self.regs[Reg::E] = self.regs[Reg::L];
                self.regs[Reg::L] = e;
            },
            Opcode::EXAFAF2 => {
                let a = self.regs[Reg::A];
                self.regs[Reg::A] = self.regs[Reg::A2];
                self.regs[Reg::A2] = a;
                let f = self.regs[Reg::F];
                self.regs[Reg::F] = self.regs[Reg::F2];
                self.regs[Reg::F2] = f;
            },
            Opcode::EXX => {
                let b = self.regs[Reg::B];
                self.regs[Reg::B] = self.regs[Reg::B2];
                self.regs[Reg::B2] = b;
                let c = self.regs[Reg::C];
                self.regs[Reg::C] = self.regs[Reg::C2];
                self.regs[Reg::C2] = c;

                let d = self.regs[Reg::D];
                self.regs[Reg::D] = self.regs[Reg::D2];
                self.regs[Reg::D2] = d;
                let e = self.regs[Reg::E];
                self.regs[Reg::E] = self.regs[Reg::E2];
                self.regs[Reg::E2] = e;

                let h = self.regs[Reg::H];
                self.regs[Reg::H] = self.regs[Reg::H2];
                self.regs[Reg::H2] = h;
                let l = self.regs[Reg::L];
                self.regs[Reg::L] = self.regs[Reg::L2];
                self.regs[Reg::L2] = l;
            },
            Opcode::EXSPHL => {
                let address = self.sp;

                let mut reg_value = self.get_big_reg(BigReg::HL);
                reg_value = self.flip_u16(reg_value);

                let mut mem_value = self.get_mem_u16(address);
                mem_value = self.flip_u16(mem_value);

                self.set_mem_u16(address, reg_value);
                self.set_big_reg(BigReg::HL, mem_value);
            },
            Opcode::EXSPIX => {
                let address = self.sp;

                let mut reg_value = self.ix;
                reg_value = self.flip_u16(reg_value);

                let mut mem_value = self.get_mem_u16(address);
                mem_value = self.flip_u16(mem_value);

                self.set_mem_u16(address, reg_value);
                self.ix = mem_value;
            },
            Opcode::EXSPIY => {
                let address = self.sp;

                let mut reg_value = self.iy;
                reg_value = self.flip_u16(reg_value);

                let mut mem_value = self.get_mem_u16(address);
                mem_value = self.flip_u16(mem_value);

                self.set_mem_u16(address, reg_value);
                self.iy = mem_value;
            },
            Opcode::LDI => {
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let address_de = self.get_reg_pair(Reg::D, Reg::E);
                self.mem[address_de as usize] = self.mem[address_hl as usize];
                self.set_reg_pair(Reg::H, Reg::L, address_hl + 1);
                self.set_reg_pair(Reg::D, Reg::E, address_de + 1);
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - 1);

                self.set_half_carry(false);
                if value_bc - 1 == 0 { self.set_parity_overflow(true); }
                else { self.set_parity_overflow(false); }
                self.set_add_subtract(false);
            }
            Opcode::LDIR => {
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let address_de = self.get_reg_pair(Reg::D, Reg::E);
                let mut counter = 0;
                while value_bc > counter {
                    self.mem[(address_de + counter) as usize] = self.mem[(address_hl + counter) as usize];
                    counter += 1;
                }
                self.set_reg_pair(Reg::H, Reg::L, address_hl + counter);
                self.set_reg_pair(Reg::D, Reg::E, address_de + counter);
                self.set_reg_pair(Reg::B, Reg::C, 0);

                self.set_half_carry(false);
                self.set_parity_overflow(false);
                self.set_add_subtract(false);
            },
            Opcode::LDD => {
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let address_de = self.get_reg_pair(Reg::D, Reg::E);
                self.mem[address_de as usize] = self.mem[address_hl as usize];
                self.set_reg_pair(Reg::H, Reg::L, address_hl - 1);
                self.set_reg_pair(Reg::D, Reg::E, address_de - 1);
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - 1);
                // TODO: Set flags
            },
            Opcode::LDDR => {
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let address_de = self.get_reg_pair(Reg::D, Reg::E);
                let mut counter = 0;
                while value_bc > counter {
                    self.mem[(address_de - counter) as usize] = self.mem[(address_hl - counter) as usize];
                    counter += 1;
                }
                self.set_reg_pair(Reg::H, Reg::L, address_hl - counter);
                self.set_reg_pair(Reg::D, Reg::E, address_de - counter);
                self.set_reg_pair(Reg::B, Reg::C, 0);
                // TODO: Set flags
            },
            Opcode::CPI => {
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                self.set_reg_pair(Reg::H, Reg::L, address_hl + 1);
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - 1);
                // TODO: Set flags
            },
            Opcode::CPIR => {
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let mut counter = 0;

                // TODO: This behavior doesn't emulate the behavior of the CPU (Review the Z80 User
                // manual description of the instruction)
                loop {
                    if self.mem[(address_hl + counter) as usize] == self.regs[Reg::A] {
                        break;
                    }
                    if value_bc - counter == 0 { break; }
                    counter += 1;
                }
                counter += 1;
                self.set_reg_pair(Reg::H, Reg::L, address_hl + counter);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - counter);

                // TODO: Set flags
            },
            Opcode::CPD => {
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                self.set_reg_pair(Reg::H, Reg::L, address_hl - 1);
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - 1);
                // TODO: Set flags
            },
            Opcode::CPDR => {
                let value_bc = self.get_reg_pair(Reg::B, Reg::C);
                let address_hl = self.get_reg_pair(Reg::H, Reg::L);
                let mut counter = 0;

                // TODO: This behavior doesn't emulate the behavior of the CPU (Review the Z80 User
                // manual description of the instruction)
                loop {
                    if self.mem[(address_hl - counter) as usize] == self.regs[Reg::A] {
                        break;
                    }
                    if value_bc - counter == 0 { break; }
                    counter += 1;
                }
                counter += 1;
                self.set_reg_pair(Reg::H, Reg::L, address_hl - counter);
                self.set_reg_pair(Reg::B, Reg::C, value_bc - counter);

                // TODO: Set flags
            },
            Opcode::ADDAR(reg) => {
                self.regs[Reg::A] += self.regs[reg];
                // TODO: Set flags
            },
            Opcode::ADDAN(value) => {
                self.regs[Reg::A] += value;
                // TODO: Set flags
            },
            Opcode::ADDAHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] += self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ADDAIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] += self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ADDAIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] += self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::SUBAR(reg) => {
                self.regs[Reg::A] -= self.regs[reg];
                // TODO: Set flags
            },
            Opcode::SUBAN(value) => {
                self.regs[Reg::A] -= value;
                // TODO: Set flags
            },
            Opcode::SUBAHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] -= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::SUBAIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] -= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::SUBAIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] -= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::SBCAR(reg) => {
                let carry = self.regs[Reg::F] & 0b00000001;
                self.regs[Reg::A] -= self.regs[reg];
                self.regs[Reg::A] -= carry;
                // TODO: Set flags
            },
            Opcode::SBCAN(value) => {
                let carry = self.regs[Reg::F] & 0b00000001;
                self.regs[Reg::A] -= value;
                self.regs[Reg::A] -= carry;
                // TODO: Set flags
            },
            Opcode::SBCAHL => {
                let carry = self.regs[Reg::F] & 0b00000001;
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] -= self.mem[address as usize];
                self.regs[Reg::A] -= carry;
                // TODO: Set flags
            },
            Opcode::SBCAIXD(displacement) => {
                let carry = self.regs[Reg::F] & 0b00000001;
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] -= self.mem[address as usize];
                self.regs[Reg::A] -= carry;
                // TODO: Set flags
            },
            Opcode::SBCAIYD(displacement) => {
                let carry = self.regs[Reg::F] & 0b00000001;
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] -= self.mem[address as usize];
                self.regs[Reg::A] -= carry;
                // TODO: Set flags
            },
            Opcode::ANDAR(reg) => {
                self.regs[Reg::A] &= self.regs[reg];
                // TODO: Set flags
            },
            Opcode::ANDAN(value) => {
                self.regs[Reg::A] &= value;
                // TODO: Set flags
            },
            Opcode::ANDAHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] &= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ANDAIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] &= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ANDAIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] &= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ORAR(reg) => {
                self.regs[Reg::A] |= self.regs[reg];
                // TODO: Set flags
            },
            Opcode::ORAN(value) => {
                self.regs[Reg::A] |= value;
                // TODO: Set flags
            },
            Opcode::ORAHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] |= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ORAIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] |= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::ORAIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] |= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::XORAR(reg) => {
                self.regs[Reg::A] ^= self.regs[reg];
                // TODO: Set flags
            },
            Opcode::XORAN(value) => {
                self.regs[Reg::A] ^= value;
                // TODO: Set flags
            },
            Opcode::XORAHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.regs[Reg::A] ^= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::XORAIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.regs[Reg::A] ^= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::XORAIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.regs[Reg::A] ^= self.mem[address as usize];
                // TODO: Set flags
            },
            Opcode::INCR(reg) => {
                self.regs[reg] += 1;
                // TODO: Set flags
            },
            Opcode::INCHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.mem[address as usize] += 1;
                // TODO: Set flags
            },
            Opcode::INCIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.mem[address as usize] += 1;
                // TODO: Set flags
            },
            Opcode::INCIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.mem[address as usize] += 1;
                // TODO: Set flags
            },
            Opcode::DECR(reg) => {
                self.regs[reg] -= 1;
                // TODO: Set flags
            },
            Opcode::DECHL => {
                let address = self.get_reg_pair(Reg::H, Reg::L);
                self.mem[address as usize] -= 1;
                // TODO: Set flags
            },
            Opcode::DECIXD(displacement) => {
                let address = self.ix + displacement as u16;
                self.mem[address as usize] -= 1;
                // TODO: Set flags
            },
            Opcode::DECIYD(displacement) => {
                let address = self.iy + displacement as u16;
                self.mem[address as usize] -= 1;
                // TODO: Set flags
            },
            _ => ()
        }
    }
}
