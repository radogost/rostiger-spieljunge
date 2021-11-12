use std::cell::RefCell;
use std::rc::Rc;

use log::{error, trace};

use crate::alu;
use crate::mmu::Mmu;
use crate::registers::Registers;

pub(crate) struct Cpu {
    registers: Registers,
    mmu: Rc<RefCell<Mmu>>,
}

impl Cpu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Cpu {
            registers: Registers::new(),
            mmu,
        }
    }

    pub fn no_boot(mmu: Rc<RefCell<Mmu>>) -> Self {
        Cpu {
            registers: Registers::no_boot(),
            mmu,
        }
    }

    pub fn step(&mut self) -> u8 {
        self.execute()
    }

    fn fetch_byte(&mut self) -> u8 {
        let pc = self.registers.pc();
        self.registers.inc_pc(1);
        self.mmu.borrow().read_byte(pc)
    }

    fn fetch_word(&mut self) -> u16 {
        let pc = self.registers.pc();
        self.registers.inc_pc(2);
        self.mmu.borrow().read_word(pc)
    }

    fn push(&mut self, value: u16) {
        let sp = self.registers.sp() - 2;
        self.mmu.borrow_mut().write_word(sp, value);
        self.registers.set_sp(sp);
    }

    fn pop(&mut self) -> u16 {
        let sp = self.registers.sp();
        let value = self.mmu.borrow().read_word(sp);
        self.registers.set_sp(sp + 2);
        value
    }

    fn execute(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        let res = match opcode {
            0x00 => self.op_0000(),
            0x01 => self.op_0001(),
            0x02 => self.op_0002(),
            0x03 => self.op_0003(),
            0x04 => self.op_0004(),
            0x05 => self.op_0005(),
            0x06 => self.op_0006(),
            0x07 => self.op_0007(),
            0x08 => self.op_0008(),
            0x09 => self.op_0009(),
            0x0a => self.op_000a(),
            0x0b => self.op_000b(),
            0x0c => self.op_000c(),
            0x0d => self.op_000d(),
            0x0e => self.op_000e(),
            0x0f => self.op_000f(),
            0x10 => self.op_0010(),
            0x11 => self.op_0011(),
            0x12 => self.op_0012(),
            0x13 => self.op_0013(),
            0x14 => self.op_0014(),
            0x15 => self.op_0015(),
            0x16 => self.op_0016(),
            0x17 => self.op_0017(),
            0x18 => self.op_0018(),
            0x19 => self.op_0019(),
            0x1a => self.op_001a(),
            0x1b => self.op_001b(),
            0x1c => self.op_001c(),
            0x1d => self.op_001d(),
            0x1e => self.op_001e(),
            0x1f => self.op_001f(),
            0x20 => self.op_0020(),
            0x21 => self.op_0021(),
            0x22 => self.op_0022(),
            0x23 => self.op_0023(),
            0x24 => self.op_0024(),
            0x25 => self.op_0025(),
            0x26 => self.op_0026(),
            0x27 => self.op_0027(),
            0x28 => self.op_0028(),
            0x29 => self.op_0029(),
            0x2a => self.op_002a(),
            0x2b => self.op_002b(),
            0x2c => self.op_002c(),
            0x2d => self.op_002d(),
            0x2e => self.op_002e(),
            0x2f => self.op_002f(),
            0x30 => self.op_0030(),
            0x31 => self.op_0031(),
            0x32 => self.op_0032(),
            0x33 => self.op_0033(),
            0x34 => self.op_0034(),
            0x35 => self.op_0035(),
            0x36 => self.op_0036(),
            0x37 => self.op_0037(),
            0x38 => self.op_0038(),
            0x39 => self.op_0039(),
            0x3a => self.op_003a(),
            0x3b => self.op_003b(),
            0x3c => self.op_003c(),
            0x3d => self.op_003d(),
            0x3e => self.op_003e(),
            0x3f => self.op_003f(),
            0x40 => self.op_0040(),
            0x41 => self.op_0041(),
            0x42 => self.op_0042(),
            0x43 => self.op_0043(),
            0x44 => self.op_0044(),
            0x45 => self.op_0045(),
            0x46 => self.op_0046(),
            0x47 => self.op_0047(),
            0x48 => self.op_0048(),
            0x49 => self.op_0049(),
            0x4a => self.op_004a(),
            0x4b => self.op_004b(),
            0x4c => self.op_004c(),
            0x4d => self.op_004d(),
            0x4e => self.op_004e(),
            0x4f => self.op_004f(),
            0x50 => self.op_0050(),
            0x51 => self.op_0051(),
            0x52 => self.op_0052(),
            0x53 => self.op_0053(),
            0x54 => self.op_0054(),
            0x55 => self.op_0055(),
            0x56 => self.op_0056(),
            0x57 => self.op_0057(),
            0x58 => self.op_0058(),
            0x59 => self.op_0059(),
            0x5a => self.op_005a(),
            0x5b => self.op_005b(),
            0x5c => self.op_005c(),
            0x5d => self.op_005d(),
            0x5e => self.op_005e(),
            0x5f => self.op_005f(),
            0x60 => self.op_0060(),
            0x61 => self.op_0061(),
            0x62 => self.op_0062(),
            0x63 => self.op_0063(),
            0x64 => self.op_0064(),
            0x65 => self.op_0065(),
            0x66 => self.op_0066(),
            0x67 => self.op_0067(),
            0x68 => self.op_0068(),
            0x69 => self.op_0069(),
            0x6a => self.op_006a(),
            0x6b => self.op_006b(),
            0x6c => self.op_006c(),
            0x6d => self.op_006d(),
            0x6e => self.op_006e(),
            0x6f => self.op_006f(),
            0x70 => self.op_0070(),
            0x71 => self.op_0071(),
            0x72 => self.op_0072(),
            0x73 => self.op_0073(),
            0x74 => self.op_0074(),
            0x75 => self.op_0075(),
            0x76 => self.op_0076(),
            0x77 => self.op_0077(),
            0x78 => self.op_0078(),
            0x79 => self.op_0079(),
            0x7a => self.op_007a(),
            0x7b => self.op_007b(),
            0x7c => self.op_007c(),
            0x7d => self.op_007d(),
            0x7e => self.op_007e(),
            0x7f => self.op_007f(),
            0x80 => self.op_0080(),
            0x81 => self.op_0081(),
            0x82 => self.op_0082(),
            0x83 => self.op_0083(),
            0x84 => self.op_0084(),
            0x85 => self.op_0085(),
            0x86 => self.op_0086(),
            0x87 => self.op_0087(),
            0x88 => self.op_0088(),
            0x89 => self.op_0089(),
            0x8a => self.op_008a(),
            0x8b => self.op_008b(),
            0x8c => self.op_008c(),
            0x8d => self.op_008d(),
            0x8e => self.op_008e(),
            0x8f => self.op_008f(),
            0x90 => self.op_0090(),
            0x91 => self.op_0091(),
            0x92 => self.op_0092(),
            0x93 => self.op_0093(),
            0x94 => self.op_0094(),
            0x95 => self.op_0095(),
            0x96 => self.op_0096(),
            0x97 => self.op_0097(),
            0x98 => self.op_0098(),
            0x99 => self.op_0099(),
            0x9a => self.op_009a(),
            0x9b => self.op_009b(),
            0x9c => self.op_009c(),
            0x9d => self.op_009d(),
            0x9e => self.op_009e(),
            0x9f => self.op_009f(),
            0xa0 => self.op_00a0(),
            0xa1 => self.op_00a1(),
            0xa2 => self.op_00a2(),
            0xa3 => self.op_00a3(),
            0xa4 => self.op_00a4(),
            0xa5 => self.op_00a5(),
            0xa6 => self.op_00a6(),
            0xa7 => self.op_00a7(),
            0xa8 => self.op_00a8(),
            0xa9 => self.op_00a9(),
            0xaa => self.op_00aa(),
            0xab => self.op_00ab(),
            0xac => self.op_00ac(),
            0xad => self.op_00ad(),
            0xae => self.op_00ae(),
            0xaf => self.op_00af(),
            0xb0 => self.op_00b0(),
            0xb1 => self.op_00b1(),
            0xb2 => self.op_00b2(),
            0xb3 => self.op_00b3(),
            0xb4 => self.op_00b4(),
            0xb5 => self.op_00b5(),
            0xb6 => self.op_00b6(),
            0xb7 => self.op_00b7(),
            0xb8 => self.op_00b8(),
            0xb9 => self.op_00b9(),
            0xba => self.op_00ba(),
            0xbb => self.op_00bb(),
            0xbc => self.op_00bc(),
            0xbd => self.op_00bd(),
            0xbe => self.op_00be(),
            0xbf => self.op_00bf(),
            0xc0 => self.op_00c0(),
            0xc1 => self.op_00c1(),
            0xc2 => self.op_00c2(),
            0xc3 => self.op_00c3(),
            0xc4 => self.op_00c4(),
            0xc5 => self.op_00c5(),
            0xc6 => self.op_00c6(),
            0xc7 => self.op_00c7(),
            0xc8 => self.op_00c8(),
            0xc9 => self.op_00c9(),
            0xca => self.op_00ca(),
            0xcb => self.execute_cb(),
            0xcc => self.op_00cc(),
            0xcd => self.op_00cd(),
            0xce => self.op_00ce(),
            0xcf => self.op_00cf(),
            0xd0 => self.op_00d0(),
            0xd1 => self.op_00d1(),
            0xd2 => self.op_00d2(),
            0xd4 => self.op_00d4(),
            0xd5 => self.op_00d5(),
            0xd6 => self.op_00d6(),
            0xd7 => self.op_00d7(),
            0xd8 => self.op_00d8(),
            0xd9 => self.op_00d9(),
            0xda => self.op_00da(),
            0xdc => self.op_00dc(),
            0xde => self.op_00de(),
            0xdf => self.op_00df(),
            0xe0 => self.op_00e0(),
            0xe1 => self.op_00e1(),
            0xe2 => self.op_00e2(),
            0xe5 => self.op_00e5(),
            0xe6 => self.op_00e6(),
            0xe7 => self.op_00e7(),
            0xe8 => self.op_00e8(),
            0xe9 => self.op_00e9(),
            0xea => self.op_00ea(),
            0xee => self.op_00ee(),
            0xef => self.op_00ef(),
            0xf0 => self.op_00f0(),
            0xf1 => self.op_00f1(),
            0xf2 => self.op_00f2(),
            0xf3 => self.op_00f3(),
            0xf5 => self.op_00f5(),
            0xf6 => self.op_00f6(),
            0xf7 => self.op_00f7(),
            0xf8 => self.op_00f8(),
            0xf9 => self.op_00f9(),
            0xfa => self.op_00fa(),
            0xfb => self.op_00fb(),
            0xfe => self.op_00fe(),
            0xff => self.op_00ff(),
            invalid => {
                error!("Invalid opcode {:04x}", invalid);
                0
            }
        };
        res
    }

    fn execute_cb(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => self.op_cb00(),
            0x01 => self.op_cb01(),
            0x02 => self.op_cb02(),
            0x03 => self.op_cb03(),
            0x04 => self.op_cb04(),
            0x05 => self.op_cb05(),
            0x06 => self.op_cb06(),
            0x07 => self.op_cb07(),
            0x08 => self.op_cb08(),
            0x09 => self.op_cb09(),
            0x0a => self.op_cb0a(),
            0x0b => self.op_cb0b(),
            0x0c => self.op_cb0c(),
            0x0d => self.op_cb0d(),
            0x0e => self.op_cb0e(),
            0x0f => self.op_cb0f(),
            0x10 => self.op_cb10(),
            0x11 => self.op_cb11(),
            0x12 => self.op_cb12(),
            0x13 => self.op_cb13(),
            0x14 => self.op_cb14(),
            0x15 => self.op_cb15(),
            0x16 => self.op_cb16(),
            0x17 => self.op_cb17(),
            0x18 => self.op_cb18(),
            0x19 => self.op_cb19(),
            0x1a => self.op_cb1a(),
            0x1b => self.op_cb1b(),
            0x1c => self.op_cb1c(),
            0x1d => self.op_cb1d(),
            0x1e => self.op_cb1e(),
            0x1f => self.op_cb1f(),
            0x20 => self.op_cb20(),
            0x21 => self.op_cb21(),
            0x22 => self.op_cb22(),
            0x23 => self.op_cb23(),
            0x24 => self.op_cb24(),
            0x25 => self.op_cb25(),
            0x26 => self.op_cb26(),
            0x27 => self.op_cb27(),
            0x28 => self.op_cb28(),
            0x29 => self.op_cb29(),
            0x2a => self.op_cb2a(),
            0x2b => self.op_cb2b(),
            0x2c => self.op_cb2c(),
            0x2d => self.op_cb2d(),
            0x2e => self.op_cb2e(),
            0x2f => self.op_cb2f(),
            0x30 => self.op_cb30(),
            0x31 => self.op_cb31(),
            0x32 => self.op_cb32(),
            0x33 => self.op_cb33(),
            0x34 => self.op_cb34(),
            0x35 => self.op_cb35(),
            0x36 => self.op_cb36(),
            0x37 => self.op_cb37(),
            0x38 => self.op_cb38(),
            0x39 => self.op_cb39(),
            0x3a => self.op_cb3a(),
            0x3b => self.op_cb3b(),
            0x3c => self.op_cb3c(),
            0x3d => self.op_cb3d(),
            0x3e => self.op_cb3e(),
            0x3f => self.op_cb3f(),
            0x40 => self.op_cb40(),
            0x41 => self.op_cb41(),
            0x42 => self.op_cb42(),
            0x43 => self.op_cb43(),
            0x44 => self.op_cb44(),
            0x45 => self.op_cb45(),
            0x46 => self.op_cb46(),
            0x47 => self.op_cb47(),
            0x48 => self.op_cb48(),
            0x49 => self.op_cb49(),
            0x4a => self.op_cb4a(),
            0x4b => self.op_cb4b(),
            0x4c => self.op_cb4c(),
            0x4d => self.op_cb4d(),
            0x4e => self.op_cb4e(),
            0x4f => self.op_cb4f(),
            0x50 => self.op_cb50(),
            0x51 => self.op_cb51(),
            0x52 => self.op_cb52(),
            0x53 => self.op_cb53(),
            0x54 => self.op_cb54(),
            0x55 => self.op_cb55(),
            0x56 => self.op_cb56(),
            0x57 => self.op_cb57(),
            0x58 => self.op_cb58(),
            0x59 => self.op_cb59(),
            0x5a => self.op_cb5a(),
            0x5b => self.op_cb5b(),
            0x5c => self.op_cb5c(),
            0x5d => self.op_cb5d(),
            0x5e => self.op_cb5e(),
            0x5f => self.op_cb5f(),
            0x60 => self.op_cb60(),
            0x61 => self.op_cb61(),
            0x62 => self.op_cb62(),
            0x63 => self.op_cb63(),
            0x64 => self.op_cb64(),
            0x65 => self.op_cb65(),
            0x66 => self.op_cb66(),
            0x67 => self.op_cb67(),
            0x68 => self.op_cb68(),
            0x69 => self.op_cb69(),
            0x6a => self.op_cb6a(),
            0x6b => self.op_cb6b(),
            0x6c => self.op_cb6c(),
            0x6d => self.op_cb6d(),
            0x6e => self.op_cb6e(),
            0x6f => self.op_cb6f(),
            0x70 => self.op_cb70(),
            0x71 => self.op_cb71(),
            0x72 => self.op_cb72(),
            0x73 => self.op_cb73(),
            0x74 => self.op_cb74(),
            0x75 => self.op_cb75(),
            0x76 => self.op_cb76(),
            0x77 => self.op_cb77(),
            0x78 => self.op_cb78(),
            0x79 => self.op_cb79(),
            0x7a => self.op_cb7a(),
            0x7b => self.op_cb7b(),
            0x7c => self.op_cb7c(),
            0x7d => self.op_cb7d(),
            0x7e => self.op_cb7e(),
            0x7f => self.op_cb7f(),
            0x80 => self.op_cb80(),
            0x81 => self.op_cb81(),
            0x82 => self.op_cb82(),
            0x83 => self.op_cb83(),
            0x84 => self.op_cb84(),
            0x85 => self.op_cb85(),
            0x86 => self.op_cb86(),
            0x87 => self.op_cb87(),
            0x88 => self.op_cb88(),
            0x89 => self.op_cb89(),
            0x8a => self.op_cb8a(),
            0x8b => self.op_cb8b(),
            0x8c => self.op_cb8c(),
            0x8d => self.op_cb8d(),
            0x8e => self.op_cb8e(),
            0x8f => self.op_cb8f(),
            0x90 => self.op_cb90(),
            0x91 => self.op_cb91(),
            0x92 => self.op_cb92(),
            0x93 => self.op_cb93(),
            0x94 => self.op_cb94(),
            0x95 => self.op_cb95(),
            0x96 => self.op_cb96(),
            0x97 => self.op_cb97(),
            0x98 => self.op_cb98(),
            0x99 => self.op_cb99(),
            0x9a => self.op_cb9a(),
            0x9b => self.op_cb9b(),
            0x9c => self.op_cb9c(),
            0x9d => self.op_cb9d(),
            0x9e => self.op_cb9e(),
            0x9f => self.op_cb9f(),
            0xa0 => self.op_cba0(),
            0xa1 => self.op_cba1(),
            0xa2 => self.op_cba2(),
            0xa3 => self.op_cba3(),
            0xa4 => self.op_cba4(),
            0xa5 => self.op_cba5(),
            0xa6 => self.op_cba6(),
            0xa7 => self.op_cba7(),
            0xa8 => self.op_cba8(),
            0xa9 => self.op_cba9(),
            0xaa => self.op_cbaa(),
            0xab => self.op_cbab(),
            0xac => self.op_cbac(),
            0xad => self.op_cbad(),
            0xae => self.op_cbae(),
            0xaf => self.op_cbaf(),
            0xb0 => self.op_cbb0(),
            0xb1 => self.op_cbb1(),
            0xb2 => self.op_cbb2(),
            0xb3 => self.op_cbb3(),
            0xb4 => self.op_cbb4(),
            0xb5 => self.op_cbb5(),
            0xb6 => self.op_cbb6(),
            0xb7 => self.op_cbb7(),
            0xb8 => self.op_cbb8(),
            0xb9 => self.op_cbb9(),
            0xba => self.op_cbba(),
            0xbb => self.op_cbbb(),
            0xbc => self.op_cbbc(),
            0xbd => self.op_cbbd(),
            0xbe => self.op_cbbe(),
            0xbf => self.op_cbbf(),
            0xc0 => self.op_cbc0(),
            0xc1 => self.op_cbc1(),
            0xc2 => self.op_cbc2(),
            0xc3 => self.op_cbc3(),
            0xc4 => self.op_cbc4(),
            0xc5 => self.op_cbc5(),
            0xc6 => self.op_cbc6(),
            0xc7 => self.op_cbc7(),
            0xc8 => self.op_cbc8(),
            0xc9 => self.op_cbc9(),
            0xca => self.op_cbca(),
            0xcb => self.op_cbcb(),
            0xcc => self.op_cbcc(),
            0xcd => self.op_cbcd(),
            0xce => self.op_cbce(),
            0xcf => self.op_cbcf(),
            0xd0 => self.op_cbd0(),
            0xd1 => self.op_cbd1(),
            0xd2 => self.op_cbd2(),
            0xd3 => self.op_cbd3(),
            0xd4 => self.op_cbd4(),
            0xd5 => self.op_cbd5(),
            0xd6 => self.op_cbd6(),
            0xd7 => self.op_cbd7(),
            0xd8 => self.op_cbd8(),
            0xd9 => self.op_cbd9(),
            0xda => self.op_cbda(),
            0xdb => self.op_cbdb(),
            0xdc => self.op_cbdc(),
            0xdd => self.op_cbdd(),
            0xde => self.op_cbde(),
            0xdf => self.op_cbdf(),
            0xe0 => self.op_cbe0(),
            0xe1 => self.op_cbe1(),
            0xe2 => self.op_cbe2(),
            0xe3 => self.op_cbe3(),
            0xe4 => self.op_cbe4(),
            0xe5 => self.op_cbe5(),
            0xe6 => self.op_cbe6(),
            0xe7 => self.op_cbe7(),
            0xe8 => self.op_cbe8(),
            0xe9 => self.op_cbe9(),
            0xea => self.op_cbea(),
            0xeb => self.op_cbeb(),
            0xec => self.op_cbec(),
            0xed => self.op_cbed(),
            0xee => self.op_cbee(),
            0xef => self.op_cbef(),
            0xf0 => self.op_cbf0(),
            0xf1 => self.op_cbf1(),
            0xf2 => self.op_cbf2(),
            0xf3 => self.op_cbf3(),
            0xf4 => self.op_cbf4(),
            0xf5 => self.op_cbf5(),
            0xf6 => self.op_cbf6(),
            0xf7 => self.op_cbf7(),
            0xf8 => self.op_cbf8(),
            0xf9 => self.op_cbf9(),
            0xfa => self.op_cbfa(),
            0xfb => self.op_cbfb(),
            0xfc => self.op_cbfc(),
            0xfd => self.op_cbfd(),
            0xfe => self.op_cbfe(),
            0xff => self.op_cbff(),
        }
    }

    /// NOP
    fn op_0000(&mut self) -> u8 {
        trace!("NOP");

        4
    }

    /// LD BC,d16
    fn op_0001(&mut self) -> u8 {
        trace!("LD BC,d16");

        let val = self.fetch_word();
        self.registers.set_bc(val);

        12
    }

    /// LD (BC),A
    fn op_0002(&mut self) -> u8 {
        trace!("LD (BC),A");

        let val = self.registers.a();
        let addr = self.registers.bc();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// INC BC
    fn op_0003(&mut self) -> u8 {
        trace!("INC BC");

        let val = self.registers.bc().wrapping_add(1);
        self.registers.set_bc(val);

        8
    }

    fn inc_8bit(&mut self, val: u8) -> u8 {
        let (res, _, half_carry) = alu::add2_8bit(val, 1);

        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);

        res
    }

    /// INC B
    fn op_0004(&mut self) -> u8 {
        trace!("INC B");

        let val = self.registers.b();
        let res = self.inc_8bit(val);
        self.registers.set_b(res);

        4
    }

    fn dec_8bit(&mut self, val: u8) -> u8 {
        let (res, _, half_carry) = alu::sub2_8bit(val, 1);

        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);

        res
    }

    /// DEC B
    fn op_0005(&mut self) -> u8 {
        trace!("DEC B");

        let val = self.registers.b();
        let res = self.dec_8bit(val);
        self.registers.set_b(res);

        4
    }

    /// LD B,d8
    fn op_0006(&mut self) -> u8 {
        trace!("LD B,d8");

        let val = self.fetch_byte();
        self.registers.set_b(val);

        8
    }

    /// RLCA
    fn op_0007(&mut self) -> u8 {
        trace!("RLCA");

        let res = self.registers.a().rotate_left(1);
        let carry = (res & 0x01) != 0;

        self.registers.set_a(res);
        self.registers.set_zero_flag(false);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        4
    }

    /// LD (a16),SP
    fn op_0008(&mut self) -> u8 {
        trace!("LD (a16),SP");

        let addr = self.fetch_word();
        let sp = self.registers.sp();
        self.mmu.borrow_mut().write_word(addr, sp);

        20
    }

    /// ADD HL,BC
    fn op_0009(&mut self) -> u8 {
        trace!("ADD HL,BC");

        let hl = self.registers.hl();
        let bc = self.registers.bc();
        let (res, carry, half_carry) = alu::add2_16bit(hl, bc);

        self.registers.set_hl(res);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// LD A,(BC)
    fn op_000a(&mut self) -> u8 {
        trace!("LD A,(BC)");

        let addr = self.registers.bc();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        8
    }

    /// DEC BC
    fn op_000b(&mut self) -> u8 {
        trace!("DEC BC");

        let val = self.registers.bc().wrapping_sub(1);
        self.registers.set_bc(val);

        8
    }

    /// INC C
    fn op_000c(&mut self) -> u8 {
        trace!("INC C");

        let val = self.registers.c();
        let res = self.inc_8bit(val);
        self.registers.set_c(res);

        4
    }

    /// DEC C
    fn op_000d(&mut self) -> u8 {
        trace!("DEC C");

        let val = self.registers.c();
        let res = self.dec_8bit(val);
        self.registers.set_c(res);

        4
    }

    /// LD C,d8
    fn op_000e(&mut self) -> u8 {
        trace!("LD C,d8");

        let val = self.fetch_byte();
        self.registers.set_c(val);

        8
    }

    /// RRCA
    fn op_000f(&mut self) -> u8 {
        trace!("RRCA");

        let res = self.registers.a().rotate_right(1);

        self.registers.set_a(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        4
    }

    /// STOP 0
    fn op_0010(&mut self) -> u8 {
        trace!("STOP 0");

        // TODO

        4
    }

    /// LD DE,d16
    fn op_0011(&mut self) -> u8 {
        trace!("LD DE,d16");

        let val = self.fetch_word();
        self.registers.set_de(val);

        12
    }

    /// LD (DE),A
    fn op_0012(&mut self) -> u8 {
        trace!("LD (DE),A");

        let val = self.registers.a();
        let addr = self.registers.de();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// INC DE
    fn op_0013(&mut self) -> u8 {
        trace!("INC DE");

        let val = self.registers.de().wrapping_add(1);
        self.registers.set_de(val);

        8
    }

    /// INC D
    fn op_0014(&mut self) -> u8 {
        trace!("INC D");

        let val = self.registers.d();
        let res = self.inc_8bit(val);
        self.registers.set_d(res);

        4
    }

    /// DEC D
    fn op_0015(&mut self) -> u8 {
        trace!("DEC D");

        let val = self.registers.d();
        let res = self.dec_8bit(val);
        self.registers.set_d(res);

        4
    }

    /// LD D,d8
    fn op_0016(&mut self) -> u8 {
        trace!("LD D,d8");

        let val = self.fetch_byte();
        self.registers.set_d(val);

        8
    }

    /// RLA
    fn op_0017(&mut self) -> u8 {
        trace!("RLA");

        let a = self.registers.a();
        let carry = (a & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = a.wrapping_shl(1) | prev_carry;

        self.registers.set_a(res);
        self.registers.set_zero_flag(false);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);
        4
    }

    /// JR r8
    fn op_0018(&mut self) -> u8 {
        trace!("JR r8");

        let jump = alu::signed_byte_to_u16(self.fetch_byte());
        self.registers.inc_pc(jump);

        12
    }

    /// ADD HL,DE
    fn op_0019(&mut self) -> u8 {
        trace!("ADD HL,DE");

        let hl = self.registers.hl();
        let de = self.registers.de();
        let (res, carry, half_carry) = alu::add2_16bit(hl, de);

        self.registers.set_hl(res);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// LD A,(DE)
    fn op_001a(&mut self) -> u8 {
        trace!("LD A,(DE)");

        let addr = self.registers.de();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        8
    }

    /// DEC DE
    fn op_001b(&mut self) -> u8 {
        trace!("DEC DE");

        let val = self.registers.de().wrapping_sub(1);
        self.registers.set_de(val);

        8
    }

    /// INC E
    fn op_001c(&mut self) -> u8 {
        trace!("INC E");

        let val = self.registers.e();
        let res = self.inc_8bit(val);
        self.registers.set_e(res);

        4
    }

    /// DEC E
    fn op_001d(&mut self) -> u8 {
        trace!("DEC E");

        let val = self.registers.e();
        let res = self.dec_8bit(val);
        self.registers.set_e(res);

        4
    }

    /// LD E,d8
    fn op_001e(&mut self) -> u8 {
        trace!("LD E,d8");

        let val = self.fetch_byte();
        self.registers.set_e(val);

        8
    }

    /// RRA
    fn op_001f(&mut self) -> u8 {
        trace!("RRA");

        let val = self.registers.a();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_a(res);
        self.registers.set_zero_flag(false);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        4
    }

    /// JR NZ,r8
    fn op_0020(&mut self) -> u8 {
        trace!("JR NZ,r8");

        let jump = alu::signed_byte_to_u16(self.fetch_byte());
        if !self.registers.zero_flag() {
            self.registers.inc_pc(jump);
            12
        } else {
            8
        }
    }

    /// LD HL,d16
    fn op_0021(&mut self) -> u8 {
        trace!("LD HL,d16");

        let val = self.fetch_word();
        self.registers.set_hl(val);

        12
    }

    /// LD (HL+),A
    fn op_0022(&mut self) -> u8 {
        trace!("LD (HL+),A");

        let val = self.registers.a();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        let hl = addr.wrapping_add(1);
        self.registers.set_hl(hl);

        8
    }

    /// INC HL
    fn op_0023(&mut self) -> u8 {
        trace!("INC HL");

        let val = self.registers.hl().wrapping_add(1);
        self.registers.set_hl(val);

        8
    }

    /// INC H
    fn op_0024(&mut self) -> u8 {
        trace!("INC H");

        let val = self.registers.h();
        let res = self.inc_8bit(val);
        self.registers.set_h(res);

        4
    }

    /// DEC H
    fn op_0025(&mut self) -> u8 {
        trace!("DEC H");

        let val = self.registers.h();
        let res = self.dec_8bit(val);
        self.registers.set_h(res);

        4
    }

    /// LD H,d8
    fn op_0026(&mut self) -> u8 {
        trace!("LD H,d8");

        let val = self.fetch_byte();
        self.registers.set_h(val);

        8
    }

    /// DAA
    fn op_0027(&mut self) -> u8 {
        trace!("DAA");

        let value = self.registers.a();
        let half_carry = self.registers.half_carry_flag();
        let negative = self.registers.negative_flag();
        let carry = self.registers.carry_flag();

        let mut adjust = 0;
        if half_carry || (!negative && (value & 0xf) > 9) {
            adjust |= 0x6;
        }

        let new_carry = if carry || (!negative && value > 0x99) {
            adjust |= 0x60;
            true
        } else {
            false
        };

        let new_value = if negative {
            value - adjust
        } else {
            value + adjust
        } as u8;
        let new_zero = new_value == 0;

        self.registers.set_a(new_value);
        self.registers.set_zero_flag(new_zero);

        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(new_carry);

        4
    }

    /// JR Z,r8
    fn op_0028(&mut self) -> u8 {
        trace!("JR Z,r8");

        let jump = alu::signed_byte_to_u16(self.fetch_byte());
        if self.registers.zero_flag() {
            self.registers.inc_pc(jump);
            12
        } else {
            8
        }
    }

    /// ADD HL,HL
    fn op_0029(&mut self) -> u8 {
        trace!("ADD HL,HL");

        let hl = self.registers.hl();
        let (res, carry, half_carry) = alu::add2_16bit(hl, hl);

        self.registers.set_hl(res);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// LD A,(HL+)
    fn op_002a(&mut self) -> u8 {
        trace!("LD A,(HL+)");

        let addr = self.registers.hl();
        let hl = self.registers.hl().wrapping_add(1);
        self.registers.set_hl(hl);
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        8
    }

    /// DEC HL
    fn op_002b(&mut self) -> u8 {
        trace!("DEC HL");

        let val = self.registers.hl().wrapping_sub(1);
        self.registers.set_hl(val);

        8
    }

    /// INC L
    fn op_002c(&mut self) -> u8 {
        trace!("INC L");

        let val = self.registers.l();
        let res = self.inc_8bit(val);
        self.registers.set_l(res);

        4
    }

    /// DEC L
    fn op_002d(&mut self) -> u8 {
        trace!("DEC L");

        let val = self.registers.l();
        let res = self.dec_8bit(val);
        self.registers.set_l(res);

        4
    }

    /// LD L,d8
    fn op_002e(&mut self) -> u8 {
        trace!("LD L,d8");

        let val = self.fetch_byte();
        self.registers.set_l(val);

        8
    }

    /// CPL
    fn op_002f(&mut self) -> u8 {
        trace!("CPL");

        let a = !self.registers.a();

        self.registers.set_a(a);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(true);

        4
    }

    /// JR NC,r8
    fn op_0030(&mut self) -> u8 {
        trace!("JR NC,r8");

        let jump = alu::signed_byte_to_u16(self.fetch_byte());
        if !self.registers.carry_flag() {
            self.registers.inc_pc(jump);
            12
        } else {
            8
        }
    }

    /// LD SP,d16
    fn op_0031(&mut self) -> u8 {
        trace!("LD SP,d16");

        let val = self.fetch_word();
        self.registers.set_sp(val);

        12
    }

    /// LD (HL-),A
    fn op_0032(&mut self) -> u8 {
        trace!("LD (HL-),A");

        let val = self.registers.a();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        let hl = self.registers.hl().wrapping_sub(1);
        self.registers.set_hl(hl);

        8
    }

    /// INC SP
    fn op_0033(&mut self) -> u8 {
        trace!("INC SP");

        let val = self.registers.sp().wrapping_add(1);
        self.registers.set_sp(val);

        8
    }

    /// INC (HL)
    fn op_0034(&mut self) -> u8 {
        trace!("INC (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let res = self.inc_8bit(val);
        self.mmu.borrow_mut().write_byte(addr, res);

        12
    }

    /// DEC (HL)
    fn op_0035(&mut self) -> u8 {
        trace!("DEC (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let res = self.dec_8bit(val);
        self.mmu.borrow_mut().write_byte(addr, res);

        12
    }

    /// LD (HL),d8
    fn op_0036(&mut self) -> u8 {
        trace!("LD (HL),d8");

        let val = self.fetch_byte();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        12
    }

    /// SCF
    fn op_0037(&mut self) -> u8 {
        trace!("SCF");

        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(true);

        4
    }

    /// JR C,r8
    fn op_0038(&mut self) -> u8 {
        trace!("JR C,r8");

        let jump = alu::signed_byte_to_u16(self.fetch_byte());
        if self.registers.carry_flag() {
            self.registers.inc_pc(jump);
            12
        } else {
            8
        }
    }

    /// ADD HL,SP
    fn op_0039(&mut self) -> u8 {
        trace!("ADD HL,SP");

        let hl = self.registers.hl();
        let sp = self.registers.sp();
        let (res, carry, half_carry) = alu::add2_16bit(hl, sp);

        self.registers.set_hl(res);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// LD A,(HL-)
    fn op_003a(&mut self) -> u8 {
        trace!("LD A,(HL-)");

        let addr = self.registers.hl();
        let hl = self.registers.hl().wrapping_sub(1);
        self.registers.set_hl(hl);
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        8
    }

    /// DEC SP
    fn op_003b(&mut self) -> u8 {
        trace!("DEC SP");

        let val = self.registers.sp().wrapping_sub(1);
        self.registers.set_sp(val);

        8
    }

    /// INC A
    fn op_003c(&mut self) -> u8 {
        trace!("INC A");

        let val = self.registers.a();
        let res = self.inc_8bit(val);
        self.registers.set_a(res);

        4
    }

    /// DEC A
    fn op_003d(&mut self) -> u8 {
        trace!("DEC A");

        let val = self.registers.a();
        let res = self.dec_8bit(val);
        self.registers.set_a(res);

        4
    }

    /// LD A,d8
    fn op_003e(&mut self) -> u8 {
        trace!("LD A,d8");

        let val = self.fetch_byte();
        self.registers.set_a(val);

        8
    }

    /// CCF
    fn op_003f(&mut self) -> u8 {
        trace!("CCF");

        let carry = !self.registers.carry_flag();
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        4
    }

    /// LD B,B
    fn op_0040(&mut self) -> u8 {
        trace!("LD B,B");

        let val = self.registers.b();
        self.registers.set_b(val);

        4
    }

    /// LD B,C
    fn op_0041(&mut self) -> u8 {
        trace!("LD B,C");

        let val = self.registers.c();
        self.registers.set_b(val);

        4
    }

    /// LD B,D
    fn op_0042(&mut self) -> u8 {
        trace!("LD B,D");

        let val = self.registers.d();
        self.registers.set_b(val);

        4
    }

    /// LD B,E
    fn op_0043(&mut self) -> u8 {
        trace!("LD B,E");

        let val = self.registers.e();
        self.registers.set_b(val);

        4
    }

    /// LD B,H
    fn op_0044(&mut self) -> u8 {
        trace!("LD B,H");

        let val = self.registers.h();
        self.registers.set_b(val);

        4
    }

    /// LD B,L
    fn op_0045(&mut self) -> u8 {
        trace!("LD B,L");

        let val = self.registers.l();
        self.registers.set_b(val);

        4
    }

    /// LD B,(HL)
    fn op_0046(&mut self) -> u8 {
        trace!("LD B,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_b(val);

        8
    }

    /// LD B,A
    fn op_0047(&mut self) -> u8 {
        trace!("LD B,A");

        let val = self.registers.a();
        self.registers.set_b(val);

        4
    }

    /// LD C,B
    fn op_0048(&mut self) -> u8 {
        trace!("LD C,B");

        let val = self.registers.b();
        self.registers.set_c(val);

        4
    }

    /// LD C,C
    fn op_0049(&mut self) -> u8 {
        trace!("LD C,C");

        let val = self.registers.c();
        self.registers.set_c(val);

        4
    }

    /// LD C,D
    fn op_004a(&mut self) -> u8 {
        trace!("LD C,D");

        let val = self.registers.d();
        self.registers.set_c(val);

        4
    }

    /// LD C,E
    fn op_004b(&mut self) -> u8 {
        trace!("LD C,E");

        let val = self.registers.e();
        self.registers.set_c(val);

        4
    }

    /// LD C,H
    fn op_004c(&mut self) -> u8 {
        trace!("LD C,H");

        let val = self.registers.h();
        self.registers.set_c(val);

        4
    }

    /// LD C,L
    fn op_004d(&mut self) -> u8 {
        trace!("LD C,L");

        let val = self.registers.l();
        self.registers.set_c(val);

        4
    }

    /// LD C,(HL)
    fn op_004e(&mut self) -> u8 {
        trace!("LD C,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_c(val);

        8
    }

    /// LD C,A
    fn op_004f(&mut self) -> u8 {
        trace!("LD C,A");

        let val = self.registers.a();
        self.registers.set_c(val);

        4
    }

    /// LD D,B
    fn op_0050(&mut self) -> u8 {
        trace!("LD D,B");

        let val = self.registers.b();
        self.registers.set_d(val);

        4
    }

    /// LD D,C
    fn op_0051(&mut self) -> u8 {
        trace!("LD D,C");

        let val = self.registers.c();
        self.registers.set_d(val);

        4
    }

    /// LD D,D
    fn op_0052(&mut self) -> u8 {
        trace!("LD D,D");

        let val = self.registers.d();
        self.registers.set_d(val);

        4
    }

    /// LD D,E
    fn op_0053(&mut self) -> u8 {
        trace!("LD D,E");

        let val = self.registers.e();
        self.registers.set_d(val);

        4
    }

    /// LD D,H
    fn op_0054(&mut self) -> u8 {
        trace!("LD D,H");

        let val = self.registers.h();
        self.registers.set_d(val);

        4
    }

    /// LD D,L
    fn op_0055(&mut self) -> u8 {
        trace!("LD D,L");

        let val = self.registers.l();
        self.registers.set_d(val);

        4
    }

    /// LD D,(HL)
    fn op_0056(&mut self) -> u8 {
        trace!("LD D,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_d(val);

        8
    }

    /// LD D,A
    fn op_0057(&mut self) -> u8 {
        trace!("LD D,A");

        let val = self.registers.a();
        self.registers.set_d(val);

        4
    }

    /// LD E,B
    fn op_0058(&mut self) -> u8 {
        trace!("LD E,B");

        let val = self.registers.b();
        self.registers.set_e(val);

        4
    }

    /// LD E,C
    fn op_0059(&mut self) -> u8 {
        trace!("LD E,C");

        let val = self.registers.c();
        self.registers.set_e(val);

        4
    }

    /// LD E,D
    fn op_005a(&mut self) -> u8 {
        trace!("LD E,D");

        let val = self.registers.d();
        self.registers.set_e(val);

        4
    }

    /// LD E,E
    fn op_005b(&mut self) -> u8 {
        trace!("LD E,E");

        let val = self.registers.e();
        self.registers.set_e(val);

        4
    }

    /// LD E,H
    fn op_005c(&mut self) -> u8 {
        trace!("LD E,H");

        let val = self.registers.h();
        self.registers.set_e(val);

        4
    }

    /// LD E,L
    fn op_005d(&mut self) -> u8 {
        trace!("LD E,L");

        let val = self.registers.l();
        self.registers.set_e(val);

        4
    }

    /// LD E,(HL)
    fn op_005e(&mut self) -> u8 {
        trace!("LD E,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_e(val);

        8
    }

    /// LD E,A
    fn op_005f(&mut self) -> u8 {
        trace!("LD E,A");

        let val = self.registers.a();
        self.registers.set_e(val);

        4
    }

    /// LD H,B
    fn op_0060(&mut self) -> u8 {
        trace!("LD H,B");

        let val = self.registers.b();
        self.registers.set_h(val);

        4
    }

    /// LD H,C
    fn op_0061(&mut self) -> u8 {
        trace!("LD H,C");

        let val = self.registers.c();
        self.registers.set_h(val);

        4
    }

    /// LD H,D
    fn op_0062(&mut self) -> u8 {
        trace!("LD H,D");

        let val = self.registers.d();
        self.registers.set_h(val);

        4
    }

    /// LD H,E
    fn op_0063(&mut self) -> u8 {
        trace!("LD H,E");

        let val = self.registers.e();
        self.registers.set_h(val);

        4
    }

    /// LD H,H
    fn op_0064(&mut self) -> u8 {
        trace!("LD H,H");

        let val = self.registers.h();
        self.registers.set_h(val);

        4
    }

    /// LD H,L
    fn op_0065(&mut self) -> u8 {
        trace!("LD H,L");

        let val = self.registers.l();
        self.registers.set_h(val);

        4
    }

    /// LD H,(HL)
    fn op_0066(&mut self) -> u8 {
        trace!("LD H,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_h(val);

        8
    }

    /// LD H,A
    fn op_0067(&mut self) -> u8 {
        trace!("LD H,A");

        let val = self.registers.a();
        self.registers.set_h(val);

        4
    }

    /// LD L,B
    fn op_0068(&mut self) -> u8 {
        trace!("LD L,B");

        let val = self.registers.b();
        self.registers.set_l(val);

        4
    }

    /// LD L,C
    fn op_0069(&mut self) -> u8 {
        trace!("LD L,C");

        let val = self.registers.c();
        self.registers.set_l(val);

        4
    }

    /// LD L,D
    fn op_006a(&mut self) -> u8 {
        trace!("LD L,D");

        let val = self.registers.d();
        self.registers.set_l(val);

        4
    }

    /// LD L,E
    fn op_006b(&mut self) -> u8 {
        trace!("LD L,E");

        let val = self.registers.e();
        self.registers.set_l(val);

        4
    }

    /// LD L,H
    fn op_006c(&mut self) -> u8 {
        trace!("LD L,H");

        let val = self.registers.h();
        self.registers.set_l(val);

        4
    }

    /// LD L,L
    fn op_006d(&mut self) -> u8 {
        trace!("LD L,L");

        let val = self.registers.l();
        self.registers.set_l(val);

        4
    }

    /// LD L,(HL)
    fn op_006e(&mut self) -> u8 {
        trace!("LD L,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_l(val);

        8
    }

    /// LD L,A
    fn op_006f(&mut self) -> u8 {
        trace!("LD L,A");

        let val = self.registers.a();
        self.registers.set_l(val);

        4
    }

    /// LD (HL),B
    fn op_0070(&mut self) -> u8 {
        trace!("LD (HL),B");

        let val = self.registers.b();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD (HL),C
    fn op_0071(&mut self) -> u8 {
        trace!("LD (HL),C");

        let val = self.registers.c();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD (HL),D
    fn op_0072(&mut self) -> u8 {
        trace!("LD (HL),D");

        let val = self.registers.d();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD (HL),E
    fn op_0073(&mut self) -> u8 {
        trace!("LD (HL),E");

        let val = self.registers.e();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD (HL),H
    fn op_0074(&mut self) -> u8 {
        trace!("LD (HL),H");

        let val = self.registers.h();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD (HL),L
    fn op_0075(&mut self) -> u8 {
        trace!("LD (HL),L");

        let val = self.registers.l();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// HALT
    fn op_0076(&mut self) -> u8 {
        trace!("HALT");
        // TODO
        4
    }

    /// LD (HL),A
    fn op_0077(&mut self) -> u8 {
        trace!("LD (HL),A");

        let val = self.registers.a();
        let addr = self.registers.hl();
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// LD A,B
    fn op_0078(&mut self) -> u8 {
        trace!("LD A,B");

        let val = self.registers.b();
        self.registers.set_a(val);

        4
    }

    /// LD A,C
    fn op_0079(&mut self) -> u8 {
        trace!("LD A,C");

        let val = self.registers.c();
        self.registers.set_a(val);

        4
    }

    /// LD A,D
    fn op_007a(&mut self) -> u8 {
        trace!("LD A,D");

        let val = self.registers.d();
        self.registers.set_a(val);

        4
    }

    /// LD A,E
    fn op_007b(&mut self) -> u8 {
        trace!("LD A,E");

        let val = self.registers.e();
        self.registers.set_a(val);

        4
    }

    /// LD A,H
    fn op_007c(&mut self) -> u8 {
        trace!("LD A,H");

        let val = self.registers.h();
        self.registers.set_a(val);

        4
    }

    /// LD A,L
    fn op_007d(&mut self) -> u8 {
        trace!("LD A,L");

        let val = self.registers.l();
        self.registers.set_a(val);

        4
    }

    /// LD A,(HL)
    fn op_007e(&mut self) -> u8 {
        trace!("LD A,(HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        8
    }

    /// LD A,A
    fn op_007f(&mut self) -> u8 {
        trace!("LD A,A");

        let val = self.registers.a();
        self.registers.set_a(val);

        4
    }

    /// ADD A,B
    fn op_0080(&mut self) -> u8 {
        trace!("ADD A,B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,C
    fn op_0081(&mut self) -> u8 {
        trace!("ADD A,C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,D
    fn op_0082(&mut self) -> u8 {
        trace!("ADD A,D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,E
    fn op_0083(&mut self) -> u8 {
        trace!("ADD A,E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,H
    fn op_0084(&mut self) -> u8 {
        trace!("ADD A,H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,L
    fn op_0085(&mut self) -> u8 {
        trace!("ADD A,L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADD A,(HL)
    fn op_0086(&mut self) -> u8 {
        trace!("ADD A,(HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// ADD A,A
    fn op_0087(&mut self) -> u8 {
        trace!("ADD A,A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,B
    fn op_0088(&mut self) -> u8 {
        trace!("ADC A,B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,C
    fn op_0089(&mut self) -> u8 {
        trace!("ADC A,C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,D
    fn op_008a(&mut self) -> u8 {
        trace!("ADC A,D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,E
    fn op_008b(&mut self) -> u8 {
        trace!("ADC A,E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,H
    fn op_008c(&mut self) -> u8 {
        trace!("ADC A,H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,L
    fn op_008d(&mut self) -> u8 {
        trace!("ADC A,L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// ADC A,(HL)
    fn op_008e(&mut self) -> u8 {
        trace!("ADC A,(HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// ADC A,A
    fn op_008f(&mut self) -> u8 {
        trace!("ADC A,A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB B
    fn op_0090(&mut self) -> u8 {
        trace!("SUB B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB C
    fn op_0091(&mut self) -> u8 {
        trace!("SUB C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB D
    fn op_0092(&mut self) -> u8 {
        trace!("SUB D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB E
    fn op_0093(&mut self) -> u8 {
        trace!("SUB E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB H
    fn op_0094(&mut self) -> u8 {
        trace!("SUB H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB L
    fn op_0095(&mut self) -> u8 {
        trace!("SUB L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SUB (HL)
    fn op_0096(&mut self) -> u8 {
        trace!("SUB (HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SUB A
    fn op_0097(&mut self) -> u8 {
        trace!("SUB A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,B
    fn op_0098(&mut self) -> u8 {
        trace!("SBC A,B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,C
    fn op_0099(&mut self) -> u8 {
        trace!("SBC A,C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,D
    fn op_009a(&mut self) -> u8 {
        trace!("SBC A,D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,E
    fn op_009b(&mut self) -> u8 {
        trace!("SBC A,E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,H
    fn op_009c(&mut self) -> u8 {
        trace!("SBC A,H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,L
    fn op_009d(&mut self) -> u8 {
        trace!("SBC A,L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// SBC A,(HL)
    fn op_009e(&mut self) -> u8 {
        trace!("SBC A,(HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SBC A,A
    fn op_009f(&mut self) -> u8 {
        trace!("SBC A,A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// AND B
    fn op_00a0(&mut self) -> u8 {
        trace!("AND B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND C
    fn op_00a1(&mut self) -> u8 {
        trace!("AND C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND D
    fn op_00a2(&mut self) -> u8 {
        trace!("AND D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND E
    fn op_00a3(&mut self) -> u8 {
        trace!("AND E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND H
    fn op_00a4(&mut self) -> u8 {
        trace!("AND H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND L
    fn op_00a5(&mut self) -> u8 {
        trace!("AND L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// AND (HL)
    fn op_00a6(&mut self) -> u8 {
        trace!("AND (HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        8
    }

    /// AND A
    fn op_00a7(&mut self) -> u8 {
        trace!("AND A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR B
    fn op_00a8(&mut self) -> u8 {
        trace!("XOR B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR C
    fn op_00a9(&mut self) -> u8 {
        trace!("XOR C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR D
    fn op_00aa(&mut self) -> u8 {
        trace!("XOR D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR E
    fn op_00ab(&mut self) -> u8 {
        trace!("XOR E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR H
    fn op_00ac(&mut self) -> u8 {
        trace!("XOR H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR L
    fn op_00ad(&mut self) -> u8 {
        trace!("XOR L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// XOR (HL)
    fn op_00ae(&mut self) -> u8 {
        trace!("XOR (HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// XOR A
    fn op_00af(&mut self) -> u8 {
        trace!("XOR A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR B
    fn op_00b0(&mut self) -> u8 {
        trace!("OR B");

        let a = self.registers.a();
        let operand = self.registers.b();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR C
    fn op_00b1(&mut self) -> u8 {
        trace!("OR C");

        let a = self.registers.a();
        let operand = self.registers.c();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR D
    fn op_00b2(&mut self) -> u8 {
        trace!("OR D");

        let a = self.registers.a();
        let operand = self.registers.d();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR E
    fn op_00b3(&mut self) -> u8 {
        trace!("OR E");

        let a = self.registers.a();
        let operand = self.registers.e();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR H
    fn op_00b4(&mut self) -> u8 {
        trace!("OR H");

        let a = self.registers.a();
        let operand = self.registers.h();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR L
    fn op_00b5(&mut self) -> u8 {
        trace!("OR L");

        let a = self.registers.a();
        let operand = self.registers.l();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// OR (HL)
    fn op_00b6(&mut self) -> u8 {
        trace!("OR (HL)");

        let a = self.registers.a();
        let operand = self.mmu.borrow().read_byte(self.registers.hl());
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// OR A
    fn op_00b7(&mut self) -> u8 {
        trace!("OR A");

        let a = self.registers.a();
        let operand = self.registers.a();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        4
    }

    /// CP B
    fn op_00b8(&mut self) -> u8 {
        trace!("CP B");

        let a = self.registers.a();
        let b = self.registers.b();

        let (res, carry, half_carry) = alu::sub2_8bit(a, b);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// CP C
    fn op_00b9(&mut self) -> u8 {
        trace!("CP C");

        let a = self.registers.a();
        let c = self.registers.c();

        let (res, carry, half_carry) = alu::sub2_8bit(a, c);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);
        4
    }

    /// CP D
    fn op_00ba(&mut self) -> u8 {
        trace!("CP D");

        let a = self.registers.a();
        let d = self.registers.d();

        let (res, carry, half_carry) = alu::sub2_8bit(a, d);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// CP E
    fn op_00bb(&mut self) -> u8 {
        trace!("CP E");

        let a = self.registers.a();
        let e = self.registers.e();

        let (res, carry, half_carry) = alu::sub2_8bit(a, e);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// CP H
    fn op_00bc(&mut self) -> u8 {
        trace!("CP H");

        let a = self.registers.a();
        let h = self.registers.h();

        let (res, carry, half_carry) = alu::sub2_8bit(a, h);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// CP L
    fn op_00bd(&mut self) -> u8 {
        trace!("CP L");

        let a = self.registers.a();
        let l = self.registers.l();

        let (res, carry, half_carry) = alu::sub2_8bit(a, l);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// CP (HL)
    fn op_00be(&mut self) -> u8 {
        trace!("CP (HL)");

        let a = self.registers.a();
        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);

        let (res, carry, half_carry) = alu::sub2_8bit(a, val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);
        8
    }

    /// CP A
    fn op_00bf(&mut self) -> u8 {
        trace!("CP A");

        let a = self.registers.a();

        let (res, carry, half_carry) = alu::sub2_8bit(a, a);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        4
    }

    /// RET NZ
    fn op_00c0(&mut self) -> u8 {
        trace!("RET NZ");

        if !self.registers.zero_flag() {
            let pc = self.pop();
            self.registers.set_pc(pc);
            20
        } else {
            8
        }
    }

    /// POP BC
    fn op_00c1(&mut self) -> u8 {
        trace!("POP BC");

        let val = self.pop();
        self.registers.set_bc(val);

        12
    }

    /// JP NZ,a16
    fn op_00c2(&mut self) -> u8 {
        trace!("JP NZ,a16");

        let addr = self.fetch_word();
        if !self.registers.zero_flag() {
            self.registers.set_pc(addr);
            16
        } else {
            12
        }
    }

    /// JP a16
    fn op_00c3(&mut self) -> u8 {
        trace!("JP a16");

        let addr = self.fetch_word();
        self.registers.set_pc(addr);

        16
    }

    /// CALL NZ,a16
    fn op_00c4(&mut self) -> u8 {
        trace!("CALL NZ,a16");

        let addr = self.fetch_word();
        if !self.registers.zero_flag() {
            let pc = self.registers.pc();
            self.push(pc);
            self.registers.set_pc(addr);
            24
        } else {
            12
        }
    }

    /// PUSH BC
    fn op_00c5(&mut self) -> u8 {
        trace!("PUSH BC");

        self.push(self.registers.bc());

        16
    }

    /// ADD A,d8
    fn op_00c6(&mut self) -> u8 {
        trace!("ADD A,d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let (res, carry, half_carry) = alu::add2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RST 00H
    fn op_00c7(&mut self) -> u8 {
        trace!("RST 00H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x00);

        16
    }

    /// RET Z
    fn op_00c8(&mut self) -> u8 {
        trace!("RET Z");

        if self.registers.zero_flag() {
            let pc = self.pop();
            self.registers.set_pc(pc);
            20
        } else {
            8
        }
    }

    /// RET
    fn op_00c9(&mut self) -> u8 {
        trace!("RET");

        let pc = self.pop();
        self.registers.set_pc(pc);

        16
    }

    /// JP Z,a16
    fn op_00ca(&mut self) -> u8 {
        trace!("JP Z,a16");

        let addr = self.fetch_word();
        if self.registers.zero_flag() {
            self.registers.set_pc(addr);
            16
        } else {
            12
        }
    }

    /// CALL Z,a16
    fn op_00cc(&mut self) -> u8 {
        trace!("CALL Z,a16");

        let addr = self.fetch_word();
        if self.registers.zero_flag() {
            let pc = self.registers.pc();
            self.push(pc);
            self.registers.set_pc(addr);
            24
        } else {
            12
        }
    }

    /// CALL a16
    fn op_00cd(&mut self) -> u8 {
        trace!("CALL a16");

        let addr = self.fetch_word();
        let pc = self.registers.pc();
        self.push(pc);
        self.registers.set_pc(addr);
        24
    }

    /// ADC A,d8
    fn op_00ce(&mut self) -> u8 {
        trace!("ADC A,d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let (res, carry, half_carry) =
            alu::add3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RST 08H
    fn op_00cf(&mut self) -> u8 {
        trace!("RST 08H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x08);

        16
    }

    /// RET NC
    fn op_00d0(&mut self) -> u8 {
        trace!("RET NC");

        if !self.registers.carry_flag() {
            let pc = self.pop();
            self.registers.set_pc(pc);
            20
        } else {
            8
        }
    }

    /// POP DE
    fn op_00d1(&mut self) -> u8 {
        trace!("POP DE");

        let val = self.pop();
        self.registers.set_de(val);

        12
    }

    /// JP NC,a16
    fn op_00d2(&mut self) -> u8 {
        trace!("JP NC,a16");

        let addr = self.fetch_word();
        if !self.registers.carry_flag() {
            self.registers.set_pc(addr);
            16
        } else {
            12
        }
    }

    /// CALL NC,a16
    fn op_00d4(&mut self) -> u8 {
        trace!("CALL NC,a16");

        let addr = self.fetch_word();
        if !self.registers.carry_flag() {
            let pc = self.registers.pc();
            self.push(pc);
            self.registers.set_pc(addr);
            24
        } else {
            12
        }
    }

    /// PUSH DE
    fn op_00d5(&mut self) -> u8 {
        trace!("PUSH DE");

        self.push(self.registers.de());

        16
    }

    /// SUB d8
    fn op_00d6(&mut self) -> u8 {
        trace!("SUB d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let (res, carry, half_carry) = alu::sub2_8bit(a, operand);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RST 10H
    fn op_00d7(&mut self) -> u8 {
        trace!("RST 10H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x10);

        16
    }

    /// RET C
    fn op_00d8(&mut self) -> u8 {
        trace!("RET C");

        if self.registers.carry_flag() {
            let pc = self.pop();
            self.registers.set_pc(pc);
            20
        } else {
            8
        }
    }

    /// RETI
    fn op_00d9(&mut self) -> u8 {
        trace!("RETI");

        let pc = self.pop();
        self.registers.set_pc(pc);
        // TODO: Enable interrupts

        16
    }

    /// JP C,a16
    fn op_00da(&mut self) -> u8 {
        trace!("JP C,a16");

        let addr = self.fetch_word();
        if self.registers.carry_flag() {
            self.registers.set_pc(addr);
            16
        } else {
            12
        }
    }

    /// CALL C,a16
    fn op_00dc(&mut self) -> u8 {
        trace!("CALL C,a16");

        let addr = self.fetch_word();
        if self.registers.carry_flag() {
            let pc = self.registers.pc();
            self.push(pc);
            self.registers.set_pc(addr);
            24
        } else {
            12
        }
    }

    /// SBC A,d8
    fn op_00de(&mut self) -> u8 {
        trace!("SBC A,d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let (res, carry, half_carry) =
            alu::sub3_8bit(a, operand, self.registers.carry_flag() as u8);
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RST 18H
    fn op_00df(&mut self) -> u8 {
        trace!("RST 18H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x18);

        16
    }

    /// LDH (a8),A
    fn op_00e0(&mut self) -> u8 {
        trace!("LDH (a8),A");

        let offset = self.fetch_byte();
        let addr = 0xff00 | offset as u16;
        self.mmu.borrow_mut().write_byte(addr, self.registers.a());

        12
    }

    /// POP HL
    fn op_00e1(&mut self) -> u8 {
        trace!("POP HL");

        let val = self.pop();
        self.registers.set_hl(val);

        12
    }

    /// LD (C),A
    fn op_00e2(&mut self) -> u8 {
        trace!("LD (C),A");

        let val = self.registers.a();

        let addr = 0xff00 | self.registers.c() as u16;
        self.mmu.borrow_mut().write_byte(addr, val);

        8
    }

    /// PUSH HL
    fn op_00e5(&mut self) -> u8 {
        trace!("PUSH HL");

        self.push(self.registers.hl());

        16
    }

    /// AND d8
    fn op_00e6(&mut self) -> u8 {
        trace!("AND d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let res = a & operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);
        self.registers.set_carry_flag(false);

        8
    }

    /// RST 20H
    fn op_00e7(&mut self) -> u8 {
        trace!("RST 20H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x20);

        16
    }

    /// ADD SP,r8
    fn op_00e8(&mut self) -> u8 {
        trace!("ADD SP,r8");

        let operand = alu::signed_byte_to_u16(self.fetch_byte());
        let (res, carry, half_carry) = alu::add2_16bit(self.registers.sp(), operand);

        self.registers.set_sp(res);
        self.registers.set_zero_flag(false);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        16
    }

    /// JP (HL)
    fn op_00e9(&mut self) -> u8 {
        trace!("JP (HL)");

        let val = self.registers.hl();
        self.registers.set_pc(val);

        4
    }

    /// LD (a16),A
    fn op_00ea(&mut self) -> u8 {
        trace!("LD (a16),A");

        let val = self.registers.a();

        let addr = self.fetch_word();
        self.mmu.borrow_mut().write_byte(addr, val);

        16
    }

    /// XOR d8
    fn op_00ee(&mut self) -> u8 {
        trace!("XOR d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let res = a ^ operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// RST 28H
    fn op_00ef(&mut self) -> u8 {
        trace!("RST 28H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x28);

        16
    }

    /// LDH A,(a8)
    fn op_00f0(&mut self) -> u8 {
        trace!("LDH A,(a8)");

        let addr = 0xff00 | self.fetch_byte() as u16;
        let val = self.mmu.borrow().read_byte(addr);
        self.registers.set_a(val);

        12
    }

    /// POP AF
    fn op_00f1(&mut self) -> u8 {
        trace!("POP AF");

        let val = self.pop();
        self.registers.set_af(val);

        12
    }

    /// LD A,(C)
    fn op_00f2(&mut self) -> u8 {
        trace!("LD A,(C)");

        let addr = 0xff00 | self.registers.c() as u16;
        let val = self.mmu.borrow().read_byte(addr);

        self.registers.set_a(val);

        8
    }

    /// DI
    fn op_00f3(&mut self) -> u8 {
        trace!("DI");
        // TODO
        4
    }

    /// PUSH AF
    fn op_00f5(&mut self) -> u8 {
        trace!("PUSH AF");

        self.push(self.registers.af());

        16
    }

    /// OR d8
    fn op_00f6(&mut self) -> u8 {
        trace!("OR d8");

        let a = self.registers.a();
        let operand = self.fetch_byte();
        let res = a | operand;
        self.registers.set_a(res);

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// RST 30H
    fn op_00f7(&mut self) -> u8 {
        trace!("RST 30H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x30);

        16
    }

    /// LD HL,SP+r8
    fn op_00f8(&mut self) -> u8 {
        trace!("LD HL,SP+r8");

        let offset = alu::signed_byte_to_u16(self.fetch_byte());
        let (res, carry, half_carry) = alu::add2_16bit(self.registers.sp(), offset);

        self.registers.set_hl(res);
        self.registers.set_zero_flag(false);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        12
    }

    /// LD SP,HL
    fn op_00f9(&mut self) -> u8 {
        trace!("LD SP,HL");

        let val = self.registers.hl();
        self.registers.set_sp(val);

        8
    }

    /// LD A,(a16)
    fn op_00fa(&mut self) -> u8 {
        trace!("LD A,(a16)");

        let addr = self.fetch_word();
        let val = self.mmu.borrow().read_byte(addr);

        self.registers.set_a(val);

        16
    }

    /// EI
    fn op_00fb(&mut self) -> u8 {
        trace!("EI");
        // TODO
        4
    }

    /// CP d8
    fn op_00fe(&mut self) -> u8 {
        trace!("CP d8");

        let a = self.registers.a();
        let val = self.fetch_byte();

        let (res, carry, half_carry) = alu::sub2_8bit(a, val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(true);
        self.registers.set_half_carry_flag(half_carry);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RST 38H
    fn op_00ff(&mut self) -> u8 {
        trace!("RST 38H");

        self.push(self.registers.pc());
        self.registers.set_pc(0x38);

        16
    }

    /// RLC B
    fn op_cb00(&mut self) -> u8 {
        trace!("RLC B");

        let res = self.registers.b().rotate_left(1);

        self.registers.set_b(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC C
    fn op_cb01(&mut self) -> u8 {
        trace!("RLC C");

        let res = self.registers.c().rotate_left(1);

        self.registers.set_c(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC D
    fn op_cb02(&mut self) -> u8 {
        trace!("RLC D");

        let res = self.registers.d().rotate_left(1);

        self.registers.set_d(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC E
    fn op_cb03(&mut self) -> u8 {
        trace!("RLC E");

        let res = self.registers.e().rotate_left(1);

        self.registers.set_e(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC H
    fn op_cb04(&mut self) -> u8 {
        trace!("RLC H");

        let res = self.registers.h().rotate_left(1);

        self.registers.set_h(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC L
    fn op_cb05(&mut self) -> u8 {
        trace!("RLC L");

        let res = self.registers.l().rotate_left(1);

        self.registers.set_l(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RLC (HL)
    fn op_cb06(&mut self) -> u8 {
        trace!("RLC (HL)");

        let addr = self.registers.hl();
        let res = self.mmu.borrow().read_byte(addr).rotate_left(1);

        self.mmu.borrow_mut().write_byte(addr, res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        16
    }

    /// RLC A
    fn op_cb07(&mut self) -> u8 {
        trace!("RLC A");

        let res = self.registers.a().rotate_left(1);

        self.registers.set_a(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 1) != 0);

        8
    }

    /// RRC B
    fn op_cb08(&mut self) -> u8 {
        trace!("RRC B");

        let res = self.registers.b().rotate_right(1);

        self.registers.set_b(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC C
    fn op_cb09(&mut self) -> u8 {
        trace!("RRC C");

        let res = self.registers.c().rotate_right(1);

        self.registers.set_c(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC D
    fn op_cb0a(&mut self) -> u8 {
        trace!("RRC D");

        let res = self.registers.d().rotate_right(1);

        self.registers.set_d(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC E
    fn op_cb0b(&mut self) -> u8 {
        trace!("RRC E");

        let res = self.registers.e().rotate_right(1);

        self.registers.set_e(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC H
    fn op_cb0c(&mut self) -> u8 {
        trace!("RRC H");

        let res = self.registers.h().rotate_right(1);

        self.registers.set_h(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC L
    fn op_cb0d(&mut self) -> u8 {
        trace!("RRC L");

        let res = self.registers.l().rotate_right(1);

        self.registers.set_l(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RRC (HL)
    fn op_cb0e(&mut self) -> u8 {
        trace!("RRC (HL)");

        let addr = self.registers.hl();
        let res = self.mmu.borrow().read_byte(addr).rotate_right(1);

        self.mmu.borrow_mut().write_byte(addr, res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        16
    }

    /// RRC A
    fn op_cb0f(&mut self) -> u8 {
        trace!("RRC A");

        let res = self.registers.a().rotate_right(1);

        self.registers.set_a(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag((res & 0x80) != 0);

        8
    }

    /// RL B
    fn op_cb10(&mut self) -> u8 {
        trace!("RL B");

        let b = self.registers.b();
        let carry = (b & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = b.wrapping_shl(1) | prev_carry;

        self.registers.set_b(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL C
    fn op_cb11(&mut self) -> u8 {
        trace!("RL C");

        let c = self.registers.c();
        let carry = (c & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = c.wrapping_shl(1) | prev_carry;

        self.registers.set_c(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL D
    fn op_cb12(&mut self) -> u8 {
        trace!("RL D");

        let d = self.registers.d();
        let carry = (d & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = d.wrapping_shl(1) | prev_carry;

        self.registers.set_d(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL E
    fn op_cb13(&mut self) -> u8 {
        trace!("RL E");

        let e = self.registers.e();
        let carry = (e & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = e.wrapping_shl(1) | prev_carry;

        self.registers.set_e(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL H
    fn op_cb14(&mut self) -> u8 {
        trace!("RL H");

        let h = self.registers.h();
        let carry = (h & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = h.wrapping_shl(1) | prev_carry;

        self.registers.set_h(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL L
    fn op_cb15(&mut self) -> u8 {
        trace!("RL L");

        let l = self.registers.l();
        let carry = (l & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = l.wrapping_shl(1) | prev_carry;

        self.registers.set_l(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RL (HL)
    fn op_cb16(&mut self) -> u8 {
        trace!("RL (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let carry = (val & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shl(1) | prev_carry;

        self.mmu.borrow_mut().write_byte(addr, val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        16
    }

    /// RL A
    fn op_cb17(&mut self) -> u8 {
        trace!("RL A");

        let a = self.registers.a();
        let carry = (a & 0x80) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = a.wrapping_shl(1) | prev_carry;

        self.registers.set_a(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR B
    fn op_cb18(&mut self) -> u8 {
        trace!("RR B");

        let val = self.registers.b();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_b(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR C
    fn op_cb19(&mut self) -> u8 {
        trace!("RR C");

        let val = self.registers.c();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_c(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR D
    fn op_cb1a(&mut self) -> u8 {
        trace!("RR D");

        let val = self.registers.d();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_d(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR E
    fn op_cb1b(&mut self) -> u8 {
        trace!("RR E");

        let val = self.registers.e();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_e(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR H
    fn op_cb1c(&mut self) -> u8 {
        trace!("RR H");

        let val = self.registers.h();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_h(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR L
    fn op_cb1d(&mut self) -> u8 {
        trace!("RR L");

        let val = self.registers.l();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_l(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// RR (HL)
    fn op_cb1e(&mut self) -> u8 {
        trace!("RR (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.mmu.borrow_mut().write_byte(addr, val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        16
    }

    /// RR A
    fn op_cb1f(&mut self) -> u8 {
        trace!("RR A");

        let val = self.registers.a();
        let carry = (val & 0x01) != 0;
        let prev_carry = self.registers.carry_flag() as u8;
        let res = val.wrapping_shr(1) | (prev_carry << 7);

        self.registers.set_a(res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA B
    fn op_cb20(&mut self) -> u8 {
        trace!("SLA B");

        let val = self.registers.b();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_b(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA C
    fn op_cb21(&mut self) -> u8 {
        trace!("SLA C");

        let val = self.registers.c();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_c(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA D
    fn op_cb22(&mut self) -> u8 {
        trace!("SLA D");

        let val = self.registers.d();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_d(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA E
    fn op_cb23(&mut self) -> u8 {
        trace!("SLA E");

        let val = self.registers.e();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_e(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA H
    fn op_cb24(&mut self) -> u8 {
        trace!("SLA H");

        let val = self.registers.h();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_h(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA L
    fn op_cb25(&mut self) -> u8 {
        trace!("SLA L");

        let val = self.registers.l();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_l(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SLA (HL)
    fn op_cb26(&mut self) -> u8 {
        trace!("SLA (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.mmu.borrow_mut().write_byte(addr, val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        16
    }

    /// SLA A
    fn op_cb27(&mut self) -> u8 {
        trace!("SLA A");

        let val = self.registers.a();
        let carry = (val & 0x80) != 0;
        let val = val.wrapping_shl(1);

        self.registers.set_a(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    fn sra(&mut self, val: u8) -> u8 {
        let carry = (val & 0x01) != 0;
        let highbit_mask = val & 0x80;
        let res = val.wrapping_shr(1) | highbit_mask;

        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        res
    }

    /// SRA B
    fn op_cb28(&mut self) -> u8 {
        trace!("SRA B");

        let val = self.registers.b();
        let res = self.sra(val);
        self.registers.set_b(res);

        8
    }

    /// SRA C
    fn op_cb29(&mut self) -> u8 {
        trace!("SRA C");

        let val = self.registers.c();
        let res = self.sra(val);
        self.registers.set_c(res);

        8
    }

    /// SRA D
    fn op_cb2a(&mut self) -> u8 {
        trace!("SRA D");

        let val = self.registers.c();
        let res = self.sra(val);
        self.registers.set_c(res);
        let val = self.registers.d().wrapping_shr(1);

        self.registers.set_d(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SRA E
    fn op_cb2b(&mut self) -> u8 {
        trace!("SRA E");

        let val = self.registers.c();
        let res = self.sra(val);
        self.registers.set_c(res);
        let val = self.registers.e().wrapping_shr(1);

        self.registers.set_e(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SRA H
    fn op_cb2c(&mut self) -> u8 {
        trace!("SRA H");

        let val = self.registers.h();
        let res = self.sra(val);
        self.registers.set_h(res);

        8
    }

    /// SRA L
    fn op_cb2d(&mut self) -> u8 {
        trace!("SRA L");

        let val = self.registers.l();
        let res = self.sra(val);
        self.registers.set_l(res);

        8
    }

    /// SRA (HL)
    fn op_cb2e(&mut self) -> u8 {
        trace!("SRA (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let res = self.sra(val);
        self.mmu.borrow_mut().write_byte(addr, res);

        16
    }

    /// SRA A
    fn op_cb2f(&mut self) -> u8 {
        trace!("SRA A");

        let val = self.registers.a();
        let res = self.sra(val);
        self.registers.set_a(res);

        8
    }

    /// SWAP B
    fn op_cb30(&mut self) -> u8 {
        trace!("SWAP B");

        let val = self.registers.b().rotate_left(4);
        self.registers.set_b(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP C
    fn op_cb31(&mut self) -> u8 {
        trace!("SWAP C");

        let val = self.registers.c().rotate_left(4);
        self.registers.set_c(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP D
    fn op_cb32(&mut self) -> u8 {
        trace!("SWAP D");

        let val = self.registers.d().rotate_left(4);
        self.registers.set_d(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP E
    fn op_cb33(&mut self) -> u8 {
        trace!("SWAP E");

        let val = self.registers.e().rotate_left(4);
        self.registers.set_e(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP H
    fn op_cb34(&mut self) -> u8 {
        trace!("SWAP H");

        let val = self.registers.h().rotate_left(4);
        self.registers.set_h(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP L
    fn op_cb35(&mut self) -> u8 {
        trace!("SWAP L");

        let val = self.registers.l().rotate_left(4);
        self.registers.set_l(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        8
    }

    /// SWAP (HL)
    fn op_cb36(&mut self) -> u8 {
        trace!("SWAP (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr).rotate_left(4);
        self.mmu.borrow_mut().write_byte(addr, val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);

        16
    }

    /// SWAP A
    fn op_cb37(&mut self) -> u8 {
        trace!("SWAP A");

        let val = self.registers.a().rotate_left(4);
        self.registers.set_a(val);
        self.registers.set_zero_flag(val == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(false);
        8
    }

    /// SRL B
    fn op_cb38(&mut self) -> u8 {
        trace!("SRL B");

        let val = self.registers.b();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_b(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL C
    fn op_cb39(&mut self) -> u8 {
        trace!("SRL C");

        let val = self.registers.c();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_c(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL D
    fn op_cb3a(&mut self) -> u8 {
        trace!("SRL D");

        let val = self.registers.d();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_d(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL E
    fn op_cb3b(&mut self) -> u8 {
        trace!("SRL E");

        let val = self.registers.e();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_e(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL H
    fn op_cb3c(&mut self) -> u8 {
        trace!("SRL H");

        let val = self.registers.h();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_h(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL L
    fn op_cb3d(&mut self) -> u8 {
        trace!("SRL L");

        let val = self.registers.l();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_l(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// SRL (HL)
    fn op_cb3e(&mut self) -> u8 {
        trace!("SRL (HL)");

        let addr = self.registers.hl();
        let val = self.mmu.borrow().read_byte(addr);
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.mmu.borrow_mut().write_byte(addr, res);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        16
    }

    /// SRL A
    fn op_cb3f(&mut self) -> u8 {
        trace!("SRL A");

        let val = self.registers.a();
        let carry = (val & 0x01) != 0;
        let res = val.wrapping_shr(1);

        self.registers.set_a(val);
        self.registers.set_zero_flag(res == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(false);
        self.registers.set_carry_flag(carry);

        8
    }

    /// BIT 0,B
    fn op_cb40(&mut self) -> u8 {
        trace!("BIT 0,B");

        let val = self.registers.b();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,C
    fn op_cb41(&mut self) -> u8 {
        trace!("BIT 0,C");

        let val = self.registers.c();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,D
    fn op_cb42(&mut self) -> u8 {
        trace!("BIT 0,D");

        let val = self.registers.d();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,E
    fn op_cb43(&mut self) -> u8 {
        trace!("BIT 0,E");

        let val = self.registers.e();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,H
    fn op_cb44(&mut self) -> u8 {
        trace!("BIT 0,H");

        let val = self.registers.h();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,L
    fn op_cb45(&mut self) -> u8 {
        trace!("BIT 0,L");

        let val = self.registers.l();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 0,(HL)
    fn op_cb46(&mut self) -> u8 {
        trace!("BIT 0,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 0,A
    fn op_cb47(&mut self) -> u8 {
        trace!("BIT 0,A");

        let val = self.registers.a();
        let zf = val & (1 << 0);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,B
    fn op_cb48(&mut self) -> u8 {
        trace!("BIT 1,B");

        let val = self.registers.b();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,C
    fn op_cb49(&mut self) -> u8 {
        trace!("BIT 1,C");

        let val = self.registers.c();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,D
    fn op_cb4a(&mut self) -> u8 {
        trace!("BIT 1,D");

        let val = self.registers.d();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,E
    fn op_cb4b(&mut self) -> u8 {
        trace!("BIT 1,E");

        let val = self.registers.e();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,H
    fn op_cb4c(&mut self) -> u8 {
        trace!("BIT 1,H");

        let val = self.registers.h();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,L
    fn op_cb4d(&mut self) -> u8 {
        trace!("BIT 1,L");

        let val = self.registers.l();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 1,(HL)
    fn op_cb4e(&mut self) -> u8 {
        trace!("BIT 1,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 1,A
    fn op_cb4f(&mut self) -> u8 {
        trace!("BIT 1,A");

        let val = self.registers.a();
        let zf = val & (1 << 1);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,B
    fn op_cb50(&mut self) -> u8 {
        trace!("BIT 2,B");

        let val = self.registers.b();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,C
    fn op_cb51(&mut self) -> u8 {
        trace!("BIT 2,C");

        let val = self.registers.c();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,D
    fn op_cb52(&mut self) -> u8 {
        trace!("BIT 2,D");

        let val = self.registers.d();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,E
    fn op_cb53(&mut self) -> u8 {
        trace!("BIT 2,E");

        let val = self.registers.e();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,H
    fn op_cb54(&mut self) -> u8 {
        trace!("BIT 2,H");

        let val = self.registers.h();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,L
    fn op_cb55(&mut self) -> u8 {
        trace!("BIT 2,L");

        let val = self.registers.l();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 2,(HL)
    fn op_cb56(&mut self) -> u8 {
        trace!("BIT 2,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 2,A
    fn op_cb57(&mut self) -> u8 {
        trace!("BIT 2,A");

        let val = self.registers.a();
        let zf = val & (1 << 2);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,B
    fn op_cb58(&mut self) -> u8 {
        trace!("BIT 3,B");

        let val = self.registers.b();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,C
    fn op_cb59(&mut self) -> u8 {
        trace!("BIT 3,C");

        let val = self.registers.c();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,D
    fn op_cb5a(&mut self) -> u8 {
        trace!("BIT 3,D");

        let val = self.registers.d();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,E
    fn op_cb5b(&mut self) -> u8 {
        trace!("BIT 3,E");

        let val = self.registers.e();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,H
    fn op_cb5c(&mut self) -> u8 {
        trace!("BIT 3,H");

        let val = self.registers.h();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,L
    fn op_cb5d(&mut self) -> u8 {
        trace!("BIT 3,L");

        let val = self.registers.l();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 3,(HL)
    fn op_cb5e(&mut self) -> u8 {
        trace!("BIT 3,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 3,A
    fn op_cb5f(&mut self) -> u8 {
        trace!("BIT 3,A");

        let val = self.registers.a();
        let zf = val & (1 << 3);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,B
    fn op_cb60(&mut self) -> u8 {
        trace!("BIT 4,B");

        let val = self.registers.b();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,C
    fn op_cb61(&mut self) -> u8 {
        trace!("BIT 4,C");

        let val = self.registers.c();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,D
    fn op_cb62(&mut self) -> u8 {
        trace!("BIT 4,D");

        let val = self.registers.d();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,E
    fn op_cb63(&mut self) -> u8 {
        trace!("BIT 4,E");

        let val = self.registers.e();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,H
    fn op_cb64(&mut self) -> u8 {
        trace!("BIT 4,H");

        let val = self.registers.h();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,L
    fn op_cb65(&mut self) -> u8 {
        trace!("BIT 4,L");

        let val = self.registers.l();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 4,(HL)
    fn op_cb66(&mut self) -> u8 {
        trace!("BIT 4,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 4,A
    fn op_cb67(&mut self) -> u8 {
        trace!("BIT 4,A");

        let val = self.registers.a();
        let zf = val & (1 << 4);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,B
    fn op_cb68(&mut self) -> u8 {
        trace!("BIT 5,B");

        let val = self.registers.b();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,C
    fn op_cb69(&mut self) -> u8 {
        trace!("BIT 5,C");

        let val = self.registers.c();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,D
    fn op_cb6a(&mut self) -> u8 {
        trace!("BIT 5,D");

        let val = self.registers.d();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,E
    fn op_cb6b(&mut self) -> u8 {
        trace!("BIT 5,E");

        let val = self.registers.e();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,H
    fn op_cb6c(&mut self) -> u8 {
        trace!("BIT 5,H");

        let val = self.registers.h();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,L
    fn op_cb6d(&mut self) -> u8 {
        trace!("BIT 5,L");

        let val = self.registers.l();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 5,(HL)
    fn op_cb6e(&mut self) -> u8 {
        trace!("BIT 5,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 5,A
    fn op_cb6f(&mut self) -> u8 {
        trace!("BIT 5,A");

        let val = self.registers.a();
        let zf = val & (1 << 5);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,B
    fn op_cb70(&mut self) -> u8 {
        trace!("BIT 6,B");

        let val = self.registers.b();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,C
    fn op_cb71(&mut self) -> u8 {
        trace!("BIT 6,C");

        let val = self.registers.c();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,D
    fn op_cb72(&mut self) -> u8 {
        trace!("BIT 6,D");

        let val = self.registers.d();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,E
    fn op_cb73(&mut self) -> u8 {
        trace!("BIT 6,E");

        let val = self.registers.e();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,H
    fn op_cb74(&mut self) -> u8 {
        trace!("BIT 6,H");

        let val = self.registers.h();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,L
    fn op_cb75(&mut self) -> u8 {
        trace!("BIT 6,L");

        let val = self.registers.l();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 6,(HL)
    fn op_cb76(&mut self) -> u8 {
        trace!("BIT 6,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 6,A
    fn op_cb77(&mut self) -> u8 {
        trace!("BIT 6,A");

        let val = self.registers.a();
        let zf = val & (1 << 6);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,B
    fn op_cb78(&mut self) -> u8 {
        trace!("BIT 7,B");

        let val = self.registers.b();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,C
    fn op_cb79(&mut self) -> u8 {
        trace!("BIT 7,C");

        let val = self.registers.c();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,D
    fn op_cb7a(&mut self) -> u8 {
        trace!("BIT 7,D");

        let val = self.registers.d();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,E
    fn op_cb7b(&mut self) -> u8 {
        trace!("BIT 7,E");

        let val = self.registers.e();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,H
    fn op_cb7c(&mut self) -> u8 {
        trace!("BIT 7,H");

        let val = self.registers.h();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,L
    fn op_cb7d(&mut self) -> u8 {
        trace!("BIT 7,L");

        let val = self.registers.l();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// BIT 7,(HL)
    fn op_cb7e(&mut self) -> u8 {
        trace!("BIT 7,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        16
    }

    /// BIT 7,A
    fn op_cb7f(&mut self) -> u8 {
        trace!("BIT 7,A");

        let val = self.registers.a();
        let zf = val & (1 << 7);
        self.registers.set_zero_flag(zf == 0);
        self.registers.set_negative_flag(false);
        self.registers.set_half_carry_flag(true);

        8
    }

    /// RES 0,B
    fn op_cb80(&mut self) -> u8 {
        trace!("RES 0,B");

        let val = self.registers.b();
        let res = val & !(1 << 0);
        self.registers.set_b(res);

        8
    }

    /// RES 0,C
    fn op_cb81(&mut self) -> u8 {
        trace!("RES 0,C");

        let val = self.registers.c();
        let res = val & !(1 << 0);
        self.registers.set_c(res);

        8
    }

    /// RES 0,D
    fn op_cb82(&mut self) -> u8 {
        trace!("RES 0,D");

        let val = self.registers.d();
        let res = val & !(1 << 0);
        self.registers.set_d(res);

        8
    }

    /// RES 0,E
    fn op_cb83(&mut self) -> u8 {
        trace!("RES 0,E");

        let val = self.registers.e();
        let res = val & !(1 << 0);
        self.registers.set_e(res);

        8
    }

    /// RES 0,H
    fn op_cb84(&mut self) -> u8 {
        trace!("RES 0,H");

        let val = self.registers.h();
        let res = val & !(1 << 0);
        self.registers.set_h(res);

        8
    }

    /// RES 0,L
    fn op_cb85(&mut self) -> u8 {
        trace!("RES 0,L");

        let val = self.registers.l();
        let res = val & !(1 << 0);
        self.registers.set_l(res);

        8
    }

    /// RES 0,(HL)
    fn op_cb86(&mut self) -> u8 {
        trace!("RES 0,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 0);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 0,A
    fn op_cb87(&mut self) -> u8 {
        trace!("RES 0,A");

        let val = self.registers.a();
        let res = val & !(1 << 0);
        self.registers.set_a(res);

        8
    }

    /// RES 1,B
    fn op_cb88(&mut self) -> u8 {
        trace!("RES 1,B");

        let val = self.registers.b();
        let res = val & !(1 << 1);
        self.registers.set_b(res);

        8
    }

    /// RES 1,C
    fn op_cb89(&mut self) -> u8 {
        trace!("RES 1,C");

        let val = self.registers.c();
        let res = val & !(1 << 1);
        self.registers.set_c(res);

        8
    }

    /// RES 1,D
    fn op_cb8a(&mut self) -> u8 {
        trace!("RES 1,D");

        let val = self.registers.d();
        let res = val & !(1 << 1);
        self.registers.set_d(res);

        8
    }

    /// RES 1,E
    fn op_cb8b(&mut self) -> u8 {
        trace!("RES 1,E");

        let val = self.registers.e();
        let res = val & !(1 << 1);
        self.registers.set_e(res);

        8
    }

    /// RES 1,H
    fn op_cb8c(&mut self) -> u8 {
        trace!("RES 1,H");

        let val = self.registers.h();
        let res = val & !(1 << 1);
        self.registers.set_h(res);

        8
    }

    /// RES 1,L
    fn op_cb8d(&mut self) -> u8 {
        trace!("RES 1,L");

        let val = self.registers.l();
        let res = val & !(1 << 1);
        self.registers.set_l(res);

        8
    }

    /// RES 1,(HL)
    fn op_cb8e(&mut self) -> u8 {
        trace!("RES 1,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 1);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 1,A
    fn op_cb8f(&mut self) -> u8 {
        trace!("RES 1,A");

        let val = self.registers.a();
        let res = val & !(1 << 1);
        self.registers.set_a(res);

        8
    }

    /// RES 2,B
    fn op_cb90(&mut self) -> u8 {
        trace!("RES 2,B");

        let val = self.registers.b();
        let res = val & !(1 << 2);
        self.registers.set_b(res);

        8
    }

    /// RES 2,C
    fn op_cb91(&mut self) -> u8 {
        trace!("RES 2,C");

        let val = self.registers.c();
        let res = val & !(1 << 2);
        self.registers.set_c(res);

        8
    }

    /// RES 2,D
    fn op_cb92(&mut self) -> u8 {
        trace!("RES 2,D");

        let val = self.registers.d();
        let res = val & !(1 << 2);
        self.registers.set_d(res);

        8
    }

    /// RES 2,E
    fn op_cb93(&mut self) -> u8 {
        trace!("RES 2,E");

        let val = self.registers.e();
        let res = val & !(1 << 2);
        self.registers.set_e(res);

        8
    }

    /// RES 2,H
    fn op_cb94(&mut self) -> u8 {
        trace!("RES 2,H");

        let val = self.registers.h();
        let res = val & !(1 << 2);
        self.registers.set_h(res);

        8
    }

    /// RES 2,L
    fn op_cb95(&mut self) -> u8 {
        trace!("RES 2,L");

        let val = self.registers.l();
        let res = val & !(1 << 2);
        self.registers.set_l(res);

        8
    }

    /// RES 2,(HL)
    fn op_cb96(&mut self) -> u8 {
        trace!("RES 2,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 2);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 2,A
    fn op_cb97(&mut self) -> u8 {
        trace!("RES 2,A");

        let val = self.registers.a();
        let res = val & !(1 << 2);
        self.registers.set_a(res);

        8
    }

    /// RES 3,B
    fn op_cb98(&mut self) -> u8 {
        trace!("RES 3,B");

        let val = self.registers.b();
        let res = val & !(1 << 3);
        self.registers.set_b(res);

        8
    }

    /// RES 3,C
    fn op_cb99(&mut self) -> u8 {
        trace!("RES 3,C");

        let val = self.registers.c();
        let res = val & !(1 << 3);
        self.registers.set_c(res);

        8
    }

    /// RES 3,D
    fn op_cb9a(&mut self) -> u8 {
        trace!("RES 3,D");

        let val = self.registers.d();
        let res = val & !(1 << 3);
        self.registers.set_d(res);

        8
    }

    /// RES 3,E
    fn op_cb9b(&mut self) -> u8 {
        trace!("RES 3,E");

        let val = self.registers.e();
        let res = val & !(1 << 3);
        self.registers.set_e(res);

        8
    }

    /// RES 3,H
    fn op_cb9c(&mut self) -> u8 {
        trace!("RES 3,H");

        let val = self.registers.h();
        let res = val & !(1 << 3);
        self.registers.set_h(res);

        8
    }

    /// RES 3,L
    fn op_cb9d(&mut self) -> u8 {
        trace!("RES 3,L");

        let val = self.registers.l();
        let res = val & !(1 << 3);
        self.registers.set_l(res);

        8
    }

    /// RES 3,(HL)
    fn op_cb9e(&mut self) -> u8 {
        trace!("RES 3,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 3);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 3,A
    fn op_cb9f(&mut self) -> u8 {
        trace!("RES 3,A");

        let val = self.registers.a();
        let res = val & !(1 << 3);
        self.registers.set_a(res);

        8
    }

    /// RES 4,B
    fn op_cba0(&mut self) -> u8 {
        trace!("RES 4,B");

        let val = self.registers.b();
        let res = val & !(1 << 4);
        self.registers.set_b(res);

        8
    }

    /// RES 4,C
    fn op_cba1(&mut self) -> u8 {
        trace!("RES 4,C");

        let val = self.registers.c();
        let res = val & !(1 << 4);
        self.registers.set_c(res);

        8
    }

    /// RES 4,D
    fn op_cba2(&mut self) -> u8 {
        trace!("RES 4,D");

        let val = self.registers.d();
        let res = val & !(1 << 4);
        self.registers.set_d(res);

        8
    }

    /// RES 4,E
    fn op_cba3(&mut self) -> u8 {
        trace!("RES 4,E");

        let val = self.registers.e();
        let res = val & !(1 << 4);
        self.registers.set_e(res);

        8
    }

    /// RES 4,H
    fn op_cba4(&mut self) -> u8 {
        trace!("RES 4,H");

        let val = self.registers.h();
        let res = val & !(1 << 4);
        self.registers.set_h(res);

        8
    }

    /// RES 4,L
    fn op_cba5(&mut self) -> u8 {
        trace!("RES 4,L");

        let val = self.registers.l();
        let res = val & !(1 << 4);
        self.registers.set_l(res);

        8
    }

    /// RES 4,(HL)
    fn op_cba6(&mut self) -> u8 {
        trace!("RES 4,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 4);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 4,A
    fn op_cba7(&mut self) -> u8 {
        trace!("RES 4,A");

        let val = self.registers.a();
        let res = val & !(1 << 4);
        self.registers.set_a(res);

        8
    }

    /// RES 5,B
    fn op_cba8(&mut self) -> u8 {
        trace!("RES 5,B");

        let val = self.registers.b();
        let res = val & !(1 << 5);
        self.registers.set_b(res);

        8
    }

    /// RES 5,C
    fn op_cba9(&mut self) -> u8 {
        trace!("RES 5,C");

        let val = self.registers.c();
        let res = val & !(1 << 5);
        self.registers.set_c(res);

        8
    }

    /// RES 5,D
    fn op_cbaa(&mut self) -> u8 {
        trace!("RES 5,D");

        let val = self.registers.d();
        let res = val & !(1 << 5);
        self.registers.set_d(res);

        8
    }

    /// RES 5,E
    fn op_cbab(&mut self) -> u8 {
        trace!("RES 5,E");

        let val = self.registers.e();
        let res = val & !(1 << 5);
        self.registers.set_e(res);

        8
    }

    /// RES 5,H
    fn op_cbac(&mut self) -> u8 {
        trace!("RES 5,H");

        let val = self.registers.h();
        let res = val & !(1 << 5);
        self.registers.set_h(res);

        8
    }

    /// RES 5,L
    fn op_cbad(&mut self) -> u8 {
        trace!("RES 5,L");

        let val = self.registers.l();
        let res = val & !(1 << 5);
        self.registers.set_l(res);

        8
    }

    /// RES 5,(HL)
    fn op_cbae(&mut self) -> u8 {
        trace!("RES 5,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 5);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 5,A
    fn op_cbaf(&mut self) -> u8 {
        trace!("RES 5,A");

        let val = self.registers.a();
        let res = val & !(1 << 5);
        self.registers.set_a(res);

        8
    }

    /// RES 6,B
    fn op_cbb0(&mut self) -> u8 {
        trace!("RES 6,B");

        let val = self.registers.b();
        let res = val & !(1 << 6);
        self.registers.set_b(res);

        8
    }

    /// RES 6,C
    fn op_cbb1(&mut self) -> u8 {
        trace!("RES 6,C");

        let val = self.registers.c();
        let res = val & !(1 << 6);
        self.registers.set_c(res);

        8
    }

    /// RES 6,D
    fn op_cbb2(&mut self) -> u8 {
        trace!("RES 6,D");

        let val = self.registers.d();
        let res = val & !(1 << 6);
        self.registers.set_d(res);

        8
    }

    /// RES 6,E
    fn op_cbb3(&mut self) -> u8 {
        trace!("RES 6,E");

        let val = self.registers.e();
        let res = val & !(1 << 6);
        self.registers.set_e(res);

        8
    }

    /// RES 6,H
    fn op_cbb4(&mut self) -> u8 {
        trace!("RES 6,H");

        let val = self.registers.h();
        let res = val & !(1 << 6);
        self.registers.set_h(res);

        8
    }

    /// RES 6,L
    fn op_cbb5(&mut self) -> u8 {
        trace!("RES 6,L");

        let val = self.registers.l();
        let res = val & !(1 << 6);
        self.registers.set_l(res);

        8
    }

    /// RES 6,(HL)
    fn op_cbb6(&mut self) -> u8 {
        trace!("RES 6,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 6);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 6,A
    fn op_cbb7(&mut self) -> u8 {
        trace!("RES 6,A");

        let val = self.registers.a();
        let res = val & !(1 << 6);
        self.registers.set_a(res);

        8
    }

    /// RES 7,B
    fn op_cbb8(&mut self) -> u8 {
        trace!("RES 7,B");

        let val = self.registers.b();
        let res = val & !(1 << 7);
        self.registers.set_b(res);

        8
    }

    /// RES 7,C
    fn op_cbb9(&mut self) -> u8 {
        trace!("RES 7,C");

        let val = self.registers.c();
        let res = val & !(1 << 7);
        self.registers.set_c(res);

        8
    }

    /// RES 7,D
    fn op_cbba(&mut self) -> u8 {
        trace!("RES 7,D");

        let val = self.registers.d();
        let res = val & !(1 << 7);
        self.registers.set_d(res);

        8
    }

    /// RES 7,E
    fn op_cbbb(&mut self) -> u8 {
        trace!("RES 7,E");

        let val = self.registers.e();
        let res = val & !(1 << 7);
        self.registers.set_e(res);

        8
    }

    /// RES 7,H
    fn op_cbbc(&mut self) -> u8 {
        trace!("RES 7,H");

        let val = self.registers.h();
        let res = val & !(1 << 7);
        self.registers.set_h(res);

        8
    }

    /// RES 7,L
    fn op_cbbd(&mut self) -> u8 {
        trace!("RES 7,L");

        let val = self.registers.l();
        let res = val & !(1 << 7);
        self.registers.set_l(res);

        8
    }

    /// RES 7,(HL)
    fn op_cbbe(&mut self) -> u8 {
        trace!("RES 7,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val & !(1 << 7);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// RES 7,A
    fn op_cbbf(&mut self) -> u8 {
        trace!("RES 7,A");

        let val = self.registers.a();
        let res = val & !(1 << 7);
        self.registers.set_a(res);

        8
    }

    /// SET 0,B
    fn op_cbc0(&mut self) -> u8 {
        trace!("SET 0,B");

        let val = self.registers.b();
        let res = val | (1 << 0);
        self.registers.set_b(res);

        8
    }

    /// SET 0,C
    fn op_cbc1(&mut self) -> u8 {
        trace!("SET 0,C");

        let val = self.registers.c();
        let res = val | (1 << 0);
        self.registers.set_c(res);

        8
    }

    /// SET 0,D
    fn op_cbc2(&mut self) -> u8 {
        trace!("SET 0,D");

        let val = self.registers.d();
        let res = val | (1 << 0);
        self.registers.set_d(res);

        8
    }

    /// SET 0,E
    fn op_cbc3(&mut self) -> u8 {
        trace!("SET 0,E");

        let val = self.registers.e();
        let res = val | (1 << 0);
        self.registers.set_e(res);

        8
    }

    /// SET 0,H
    fn op_cbc4(&mut self) -> u8 {
        trace!("SET 0,H");

        let val = self.registers.h();
        let res = val | (1 << 0);
        self.registers.set_h(res);

        8
    }

    /// SET 0,L
    fn op_cbc5(&mut self) -> u8 {
        trace!("SET 0,L");

        let val = self.registers.l();
        let res = val | (1 << 0);
        self.registers.set_l(res);

        8
    }

    /// SET 0,(HL)
    fn op_cbc6(&mut self) -> u8 {
        trace!("SET 0,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 0);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 0,A
    fn op_cbc7(&mut self) -> u8 {
        trace!("SET 0,A");

        let val = self.registers.a();
        let res = val | (1 << 0);
        self.registers.set_a(res);

        8
    }

    /// SET 1,B
    fn op_cbc8(&mut self) -> u8 {
        trace!("SET 1,B");

        let val = self.registers.b();
        let res = val | (1 << 1);
        self.registers.set_b(res);

        8
    }

    /// SET 1,C
    fn op_cbc9(&mut self) -> u8 {
        trace!("SET 1,C");

        let val = self.registers.c();
        let res = val | (1 << 1);
        self.registers.set_c(res);

        8
    }

    /// SET 1,D
    fn op_cbca(&mut self) -> u8 {
        trace!("SET 1,D");

        let val = self.registers.d();
        let res = val | (1 << 1);
        self.registers.set_d(res);

        8
    }

    /// SET 1,E
    fn op_cbcb(&mut self) -> u8 {
        trace!("SET 1,E");

        let val = self.registers.e();
        let res = val | (1 << 1);
        self.registers.set_e(res);

        8
    }

    /// SET 1,H
    fn op_cbcc(&mut self) -> u8 {
        trace!("SET 1,H");

        let val = self.registers.h();
        let res = val | (1 << 1);
        self.registers.set_h(res);

        8
    }

    /// SET 1,L
    fn op_cbcd(&mut self) -> u8 {
        trace!("SET 1,L");

        let val = self.registers.l();
        let res = val | (1 << 1);
        self.registers.set_l(res);

        8
    }

    /// SET 1,(HL)
    fn op_cbce(&mut self) -> u8 {
        trace!("SET 1,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 1);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 1,A
    fn op_cbcf(&mut self) -> u8 {
        trace!("SET 1,A");

        let val = self.registers.a();
        let res = val | (1 << 1);
        self.registers.set_a(res);

        8
    }

    /// SET 2,B
    fn op_cbd0(&mut self) -> u8 {
        trace!("SET 2,B");

        let val = self.registers.b();
        let res = val | (1 << 2);
        self.registers.set_b(res);

        8
    }

    /// SET 2,C
    fn op_cbd1(&mut self) -> u8 {
        trace!("SET 2,C");

        let val = self.registers.c();
        let res = val | (1 << 2);
        self.registers.set_c(res);

        8
    }

    /// SET 2,D
    fn op_cbd2(&mut self) -> u8 {
        trace!("SET 2,D");

        let val = self.registers.d();
        let res = val | (1 << 2);
        self.registers.set_d(res);

        8
    }

    /// SET 2,E
    fn op_cbd3(&mut self) -> u8 {
        trace!("SET 2,E");

        let val = self.registers.e();
        let res = val | (1 << 2);
        self.registers.set_e(res);

        8
    }

    /// SET 2,H
    fn op_cbd4(&mut self) -> u8 {
        trace!("SET 2,H");

        let val = self.registers.h();
        let res = val | (1 << 2);
        self.registers.set_h(res);

        8
    }

    /// SET 2,L
    fn op_cbd5(&mut self) -> u8 {
        trace!("SET 2,L");

        let val = self.registers.l();
        let res = val | (1 << 2);
        self.registers.set_l(res);

        8
    }

    /// SET 2,(HL)
    fn op_cbd6(&mut self) -> u8 {
        trace!("SET 2,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 2);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 2,A
    fn op_cbd7(&mut self) -> u8 {
        trace!("SET 2,A");

        let val = self.registers.a();
        let res = val | (1 << 2);
        self.registers.set_a(res);

        8
    }

    /// SET 3,B
    fn op_cbd8(&mut self) -> u8 {
        trace!("SET 3,B");

        let val = self.registers.b();
        let res = val | (1 << 3);
        self.registers.set_b(res);

        8
    }

    /// SET 3,C
    fn op_cbd9(&mut self) -> u8 {
        trace!("SET 3,C");

        let val = self.registers.c();
        let res = val | (1 << 3);
        self.registers.set_c(res);

        8
    }

    /// SET 3,D
    fn op_cbda(&mut self) -> u8 {
        trace!("SET 3,D");

        let val = self.registers.d();
        let res = val | (1 << 3);
        self.registers.set_d(res);

        8
    }

    /// SET 3,E
    fn op_cbdb(&mut self) -> u8 {
        trace!("SET 3,E");

        let val = self.registers.e();
        let res = val | (1 << 3);
        self.registers.set_e(res);

        8
    }

    /// SET 3,H
    fn op_cbdc(&mut self) -> u8 {
        trace!("SET 3,H");

        let val = self.registers.h();
        let res = val | (1 << 3);
        self.registers.set_h(res);

        8
    }

    /// SET 3,L
    fn op_cbdd(&mut self) -> u8 {
        trace!("SET 3,L");

        let val = self.registers.l();
        let res = val | (1 << 3);
        self.registers.set_l(res);

        8
    }

    /// SET 3,(HL)
    fn op_cbde(&mut self) -> u8 {
        trace!("SET 3,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 3);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 3,A
    fn op_cbdf(&mut self) -> u8 {
        trace!("SET 3,A");

        let val = self.registers.a();
        let res = val | (1 << 3);
        self.registers.set_a(res);

        8
    }

    /// SET 4,B
    fn op_cbe0(&mut self) -> u8 {
        trace!("SET 4,B");

        let val = self.registers.b();
        let res = val | (1 << 4);
        self.registers.set_b(res);

        8
    }

    /// SET 4,C
    fn op_cbe1(&mut self) -> u8 {
        trace!("SET 4,C");

        let val = self.registers.c();
        let res = val | (1 << 4);
        self.registers.set_c(res);

        8
    }

    /// SET 4,D
    fn op_cbe2(&mut self) -> u8 {
        trace!("SET 4,D");

        let val = self.registers.d();
        let res = val | (1 << 4);
        self.registers.set_d(res);

        8
    }

    /// SET 4,E
    fn op_cbe3(&mut self) -> u8 {
        trace!("SET 4,E");

        let val = self.registers.e();
        let res = val | (1 << 4);
        self.registers.set_e(res);

        8
    }

    /// SET 4,H
    fn op_cbe4(&mut self) -> u8 {
        trace!("SET 4,H");

        let val = self.registers.h();
        let res = val | (1 << 4);
        self.registers.set_h(res);

        8
    }

    /// SET 4,L
    fn op_cbe5(&mut self) -> u8 {
        trace!("SET 4,L");

        let val = self.registers.l();
        let res = val | (1 << 4);
        self.registers.set_l(res);

        8
    }

    /// SET 4,(HL)
    fn op_cbe6(&mut self) -> u8 {
        trace!("SET 4,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 4);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 4,A
    fn op_cbe7(&mut self) -> u8 {
        trace!("SET 4,A");

        let val = self.registers.a();
        let res = val | (1 << 4);
        self.registers.set_a(res);

        8
    }

    /// SET 5,B
    fn op_cbe8(&mut self) -> u8 {
        trace!("SET 5,B");

        let val = self.registers.b();
        let res = val | (1 << 5);
        self.registers.set_b(res);

        8
    }

    /// SET 5,C
    fn op_cbe9(&mut self) -> u8 {
        trace!("SET 5,C");

        let val = self.registers.c();
        let res = val | (1 << 5);
        self.registers.set_c(res);

        8
    }

    /// SET 5,D
    fn op_cbea(&mut self) -> u8 {
        trace!("SET 5,D");

        let val = self.registers.d();
        let res = val | (1 << 5);
        self.registers.set_d(res);

        8
    }

    /// SET 5,E
    fn op_cbeb(&mut self) -> u8 {
        trace!("SET 5,E");

        let val = self.registers.e();
        let res = val | (1 << 5);
        self.registers.set_e(res);

        8
    }

    /// SET 5,H
    fn op_cbec(&mut self) -> u8 {
        trace!("SET 5,H");

        let val = self.registers.h();
        let res = val | (1 << 5);
        self.registers.set_h(res);

        8
    }

    /// SET 5,L
    fn op_cbed(&mut self) -> u8 {
        trace!("SET 5,L");

        let val = self.registers.l();
        let res = val | (1 << 5);
        self.registers.set_l(res);

        8
    }

    /// SET 5,(HL)
    fn op_cbee(&mut self) -> u8 {
        trace!("SET 5,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 5);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 5,A
    fn op_cbef(&mut self) -> u8 {
        trace!("SET 5,A");

        let val = self.registers.a();
        let res = val | (1 << 5);
        self.registers.set_a(res);

        8
    }

    /// SET 6,B
    fn op_cbf0(&mut self) -> u8 {
        trace!("SET 6,B");

        let val = self.registers.b();
        let res = val | (1 << 6);
        self.registers.set_b(res);

        8
    }

    /// SET 6,C
    fn op_cbf1(&mut self) -> u8 {
        trace!("SET 6,C");

        let val = self.registers.c();
        let res = val | (1 << 6);
        self.registers.set_c(res);

        8
    }

    /// SET 6,D
    fn op_cbf2(&mut self) -> u8 {
        trace!("SET 6,D");

        let val = self.registers.d();
        let res = val | (1 << 6);
        self.registers.set_d(res);

        8
    }

    /// SET 6,E
    fn op_cbf3(&mut self) -> u8 {
        trace!("SET 6,E");

        let val = self.registers.e();
        let res = val | (1 << 6);
        self.registers.set_e(res);

        8
    }

    /// SET 6,H
    fn op_cbf4(&mut self) -> u8 {
        trace!("SET 6,H");

        let val = self.registers.h();
        let res = val | (1 << 6);
        self.registers.set_h(res);

        8
    }

    /// SET 6,L
    fn op_cbf5(&mut self) -> u8 {
        trace!("SET 6,L");

        let val = self.registers.l();
        let res = val | (1 << 6);
        self.registers.set_l(res);

        8
    }

    /// SET 6,(HL)
    fn op_cbf6(&mut self) -> u8 {
        trace!("SET 6,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 6);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 6,A
    fn op_cbf7(&mut self) -> u8 {
        trace!("SET 6,A");

        let val = self.registers.a();
        let res = val | (1 << 6);
        self.registers.set_a(res);

        8
    }

    /// SET 7,B
    fn op_cbf8(&mut self) -> u8 {
        trace!("SET 7,B");

        let val = self.registers.b();
        let res = val | (1 << 7);
        self.registers.set_b(res);

        8
    }

    /// SET 7,C
    fn op_cbf9(&mut self) -> u8 {
        trace!("SET 7,C");

        let val = self.registers.c();
        let res = val | (1 << 7);
        self.registers.set_c(res);

        8
    }

    /// SET 7,D
    fn op_cbfa(&mut self) -> u8 {
        trace!("SET 7,D");

        let val = self.registers.d();
        let res = val | (1 << 7);
        self.registers.set_d(res);

        8
    }

    /// SET 7,E
    fn op_cbfb(&mut self) -> u8 {
        trace!("SET 7,E");

        let val = self.registers.e();
        let res = val | (1 << 7);
        self.registers.set_e(res);

        8
    }

    /// SET 7,H
    fn op_cbfc(&mut self) -> u8 {
        trace!("SET 7,H");

        let val = self.registers.h();
        let res = val | (1 << 7);
        self.registers.set_h(res);

        8
    }

    /// SET 7,L
    fn op_cbfd(&mut self) -> u8 {
        trace!("SET 7,L");

        let val = self.registers.l();
        let res = val | (1 << 7);
        self.registers.set_l(res);

        8
    }

    /// SET 7,(HL)
    fn op_cbfe(&mut self) -> u8 {
        trace!("SET 7,(HL)");

        let hl = self.registers.hl();
        let val = self.mmu.borrow().read_byte(hl);
        let res = val | (1 << 7);
        self.mmu.borrow_mut().write_byte(hl, res);

        16
    }

    /// SET 7,A
    fn op_cbff(&mut self) -> u8 {
        trace!("SET 7,A");

        let val = self.registers.a();
        let res = val | (1 << 7);
        self.registers.set_a(res);

        8
    }
}

#[cfg(test)]
mod tests {

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::mmu::Mmu;
    use crate::ppu::Ppu;

    use super::Cpu;

    #[test]
    fn push_pop() {
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let mmu = Rc::new(RefCell::new(Mmu::new(ppu)));
        let mut cpu = Cpu::new(mmu);
        cpu.registers.set_sp(100);

        let values = vec![0, 1, 2, 3, 4, 5];
        for val in values.iter() {
            cpu.push(*val);
        }

        let mut popped_values = Vec::new();
        for _ in 0..values.len() {
            let val = cpu.pop();
            popped_values.push(val);
        }
        popped_values.reverse();

        assert_eq!(values, popped_values);
    }
}
