const ZERO_FLAG: u8 = 0b10000000;
const NEGATIVE_FLAG: u8 = 0b01000000;
const HALF_CARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

pub(crate) struct Registers {
    a: u8, // accumulator
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,   // flag register
    sp: u16, // stack pointer
    pc: u16, // program counter
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn set_c(&mut self, c: u8) {
        self.c = c;
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn set_d(&mut self, d: u8) {
        self.d = d;
    }

    pub fn e(&self) -> u8 {
        self.c
    }

    pub fn set_e(&mut self, e: u8) {
        self.e = e;
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn set_h(&mut self, h: u8) {
        self.h = h;
    }

    pub fn l(&self) -> u8 {
        self.c
    }

    pub fn set_l(&mut self, l: u8) {
        self.l = l;
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f as u16) & 0xF0)
    }

    pub fn set_af(&mut self, af: u16) {
        self.a = ((af >> 8) & 0xff) as u8;
        self.f = (af & 0xff) as u8;
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.b = ((bc >> 8) & 0xff) as u8;
        self.c = (bc & 0xff) as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, de: u16) {
        self.d = ((de >> 8) & 0xff) as u8;
        self.e = (de & 0xff) as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.h = ((hl >> 8) & 0xff) as u8;
        self.l = (hl & 0xff) as u8;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn inc_pc(&mut self, by: u16) {
        self.pc = self.pc.wrapping_add(by);
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    fn flag(&self, mask: u8) -> bool {
        self.f & mask != 0
    }

    fn set_flag(&mut self, set: bool, mask: u8) {
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
    }

    pub fn zero_flag(&self) -> bool {
        self.flag(ZERO_FLAG)
    }

    pub fn set_zero_flag(&mut self, set: bool) {
        self.set_flag(set, ZERO_FLAG)
    }

    pub fn negative_flag(&self) -> bool {
        self.flag(NEGATIVE_FLAG)
    }

    pub fn set_negative_flag(&mut self, set: bool) {
        self.set_flag(set, NEGATIVE_FLAG)
    }

    pub fn half_carry_flag(&self) -> bool {
        self.flag(HALF_CARRY_FLAG)
    }

    pub fn set_half_carry_flag(&mut self, set: bool) {
        self.set_flag(set, HALF_CARRY_FLAG)
    }

    pub fn carry_flag(&self) -> bool {
        self.flag(CARRY_FLAG)
    }

    pub fn set_carry_flag(&mut self, set: bool) {
        self.set_flag(set, CARRY_FLAG)
    }
}
