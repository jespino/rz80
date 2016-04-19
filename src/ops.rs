#[derive(Debug)]
pub enum Condition {
    NonZero = 0b000,
    Zero = 0b001,
    NoCarry = 0b010,
    Carry = 0b011,
    ParityOdd = 0b100,
    ParityEven = 0b101,
    PositiveSign = 0b110,
    NegativeSign = 0b111,
}

#[derive(Debug)]
pub enum Reg {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    H = 6,
    L = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    H2 = 14,
    L2 = 15,
}

#[derive(Debug)]
pub enum BigReg {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
    IX = 4,
    IY = 5,
    AF = 6,
}

pub type Bit = u8;
pub type IODevice = u8;
pub type Value = u8;
pub type Address = u16;
pub type Displacement = u8;

#[derive(Debug)]
pub enum Opcode {
    LDRR(Reg, Reg),
    LDRN(Reg, Value),
    LDRHL(Reg),
    LDRIXD(Reg, Displacement),
    LDRIYD(Reg, Displacement),

    LDHLR(Reg),
    LDIXDR(Displacement, Reg),
    LDIYDR(Displacement, Reg),
    LDHLN(Value),
    LDIXDN(Displacement, Value),
    LDIYDN(Displacement, Value),
    LDABC,
    LDADE,
    LDANN(Address),
    LDBCA,
    LDDEA,
    LDNNA(Address),
    LDAI,
    LDAR,
    LDIA,
    LDRA,

    LDDDNN(BigReg, Address),
    LDIXNN(Address),
    LDIYNN(Address),
    LDHLNN(Address),
    LDDDNN2(BigReg, Address),
    LDIXNN2(Address),
    LDIYNN2(Address),

    LDNNHL(Address),
    LDNNDD(Address, BigReg),
    LDNNIX(Address),
    LDNNIY(Address),
    LDSPHL,
    LDSPIX,
    LDSPIY,
    PUSHQQ(BigReg),
    PUSHIX,
    PUSHIY,
    POPQQ(BigReg),
    POPIX,
    POPIY,
    EXDEHL,
    EXAFAF2,
    EXX,
    EXSPHL,
    EXSPIX,
    EXSPIY,
    LDI,
    LDIR,
    LDD,
    LDDR,
    CPI,
    CPIR,
    CPD,
    CPDR,
    ADDAR(Reg),
    ADDAN(Value),
    ADDAHL,
    ADDAIXD(Displacement),
    ADDAIYD(Displacement),
    ADCAR(Reg),
    ADCAN(Value),
    ADCAHL(Value),
    ADCAIXD(Displacement),
    ADCAIYD(Displacement),
    SUBAR(Reg),
    SUBAN(Value),
    SUBAHL,
    SUBAIXD(Displacement),
    SUBAIYD(Displacement),
    SBCAR(Reg),
    SBCAN(Value),
    SBCAHL,
    SBCAIXD(Displacement),
    SBCAIYD(Displacement),
    ANDAR(Reg),
    ANDAN(Value),
    ANDAHL,
    ANDAIXD(Displacement),
    ANDAIYD(Displacement),
    ORAR(Reg),
    ORAN(Value),
    ORAHL,
    ORAIXD(Displacement),
    ORAIYD(Displacement),
    XORAR(Reg),
    XORAN(Value),
    XORAHL,
    XORAIXD(Displacement),
    XORAIYD(Displacement),
    CPAR(Reg),
    CPAN(Value),
    CPAHL,
    CPAIXD(Displacement),
    CPAIYD(Displacement),
    INCR(Reg),
    INCHL,
    INCIXD(Displacement),
    INCIYD(Displacement),
    DECR(Reg),
    DECHL,
    DECIXD(Displacement),
    DECIYD(Displacement),
    DAA,
    CPL,
    NEG,
    CCF,
    SCF,
    NOP,
    HALT,
    DI,
    EI,
    IM0,
    IM1,
    IM2,
    ADDHLSS(BigReg),
    ADCHLSS(BigReg),
    SBCHLSS(BigReg),
    ADDIXPP(BigReg),
    ADDIYRR(BigReg),
    INCSS(BigReg),
    INCIX,
    INCIY,
    DECSS(BigReg),
    DECIX,
    DECIY,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLCR(Reg),
    RLCHL,
    RLCIXD(Displacement),
    RLCIYD(Displacement),
    RRCR(Reg),
    RRCHL,
    RRCIXD(Displacement),
    RRCIYD(Displacement),
    RRR(Reg),
    RRHL,
    RRIXD(Displacement),
    RRIYD(Displacement),
    SLAR(Reg),
    SLAHL,
    SLAIXD(Displacement),
    SLAIYD(Displacement),
    SRAR(Reg),
    SRAHL,
    SRAIXD(Displacement),
    SRAIYD(Displacement),
    SRLR(Reg),
    SRLHL,
    SRLIXD(Displacement),
    SRLIYD(Displacement),
    RLD,
    RRD,
    BITBR(Bit, Reg),
    BITBHL(Bit),
    BITBIXD(Bit, Displacement),
    BITBIYD(Bit, Displacement),
    SETBR(Bit, Reg),
    SETBHL(Bit),
    SETBIXD(Bit, Displacement),
    SETBIYD(Bit, Displacement),
    RESBR(Bit, Reg),
    RESBHL(Bit),
    RESBIXD(Bit, Displacement),
    RESBIYD(Bit, Displacement),
    JPNN(Address),
    JPCCNN(Condition, Address),
    JRE(Value),
    JRCE(Value),
    JRNCE(Value),
    JRZE(Value),
    JRNZE(Value),
    JPHL,
    JPIX,
    JPIY,
    DJNZE(Value),
    CALLNN(Address),
    CALLCCNN(Condition, Address),
    RET,
    RETCC(Condition),
    RETI,
    RETN,
    RETP(Value),
    INAN(IODevice),
    INRC(Reg),
    INI,
    INIR,
    IND,
    INDR,
    OUTNA(IODevice),
    OUTCR(Reg),
    OUTI,
    OTIR,
    OUTD,
    OTDR,
}

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


fn parse_op(code: &mut Iterator<Item=u8>) -> (u8, Opcode) {
    match byte_to_bits(code.next().unwrap()) {
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


#[test]
fn parse_ldrr() {
    let data = vec![0b01111000];
    let (bytes, op) = parse_op(&mut data.into_iter());

    assert_eq!(bytes, 1);
    match op {
        Opcode::LDRR(Reg::A, Reg::B) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn parse_ldrn() {
    let data = vec![0b00111110, 0b00000001];
    let (bytes, op) = parse_op(&mut data.into_iter());

    assert_eq!(bytes, 2);
    match op {
        Opcode::LDRN(Reg::A, 1) => assert!(true),
        _ => assert!(false)
    }
}
