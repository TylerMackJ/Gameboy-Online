use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Reg8 { A, F, B, C, D, E, H, L, }
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Reg16 { AF, BC, DE, HL, SP, PC, }

// Flags values = offset into f register
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
struct Split {
    pub bot: u8,
    pub top: u8,
}

#[derive(Copy, Clone)]
union Register {
    pub half: Split,
    pub all: u16,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Registers {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: u16,
    pc: u16,
}

#[wasm_bindgen]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: Register {all: 0u16},
            bc: Register {all: 0u16},
            de: Register {all: 0u16},
            hl: Register {all: 0u16},
            sp: 0u16,
            pc: 0x0100,
        }
    }

    // General register get/set
    pub fn get_reg_8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => return self.get_a(),
            Reg8::F => return self.get_f(),
            Reg8::B => return self.get_b(),
            Reg8::C => return self.get_c(),
            Reg8::D => return self.get_d(),
            Reg8::E => return self.get_e(),
            Reg8::H => return self.get_h(),
            Reg8::L => return self.get_l(),
        }
    }
    pub fn set_reg_8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.set_a(value),
            Reg8::F => self.set_f(value),
            Reg8::B => self.set_b(value),
            Reg8::C => self.set_c(value),
            Reg8::D => self.set_d(value),
            Reg8::E => self.set_e(value),
            Reg8::H => self.set_h(value),
            Reg8::L => self.set_l(value),
        }
    }

    pub fn get_reg_16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => return self.get_af(),
            Reg16::BC => return self.get_bc(),
            Reg16::DE => return self.get_de(),
            Reg16::HL => return self.get_hl(),
            Reg16::SP => return self.get_sp(),
            Reg16::PC => return self.get_pc(),
        }
    }
    pub fn set_reg_16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => self.set_af(value),
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
            Reg16::SP => self.set_sp(value),
            Reg16::PC => self.set_pc(value),
        }
    }


    // AF
    pub fn get_a(&self) -> u8 {
        unsafe { self.af.half.top }
    }
    pub fn set_a(&mut self, a: u8) {
        self.af.half.top = a
    }

    pub fn get_f(&self) -> u8 {
        unsafe { self.af.half.bot }
    }
    pub fn set_f(&mut self, f: u8) {
        self.af.half.bot = f
    }

    pub fn get_af(&self) -> u16 {
        unsafe { self.af.all }
    }
    pub fn set_af(&mut self, af: u16) {
        self.af.all = af
    }

    // BC
    pub fn get_b(&self) -> u8 {
        unsafe { self.bc.half.top }
    }
    pub fn set_b(&mut self, b: u8) {
        self.bc.half.top = b
    }

    pub fn get_c(&self) -> u8 {
        unsafe { self.bc.half.bot }
    }
    pub fn set_c(&mut self, c: u8) {
        self.bc.half.bot = c
    }

    pub fn get_bc(&self) -> u16 {
        unsafe { self.bc.all }
    }
    pub fn set_bc(&mut self, bc: u16) {
        self.bc.all = bc
    }

    //DE
    pub fn get_d(&self) -> u8 {
        unsafe { self.de.half.top }
    }
    pub fn set_d(&mut self, d: u8) {
        self.de.half.top = d
    }

    pub fn get_e(&self) -> u8 {
        unsafe { self.de.half.bot }
    }
    pub fn set_e(&mut self, e: u8) {
        self.de.half.bot = e
    }

    pub fn get_de(&self) -> u16 {
        unsafe { self.de.all }
    }
    pub fn set_de(&mut self, de: u16) {
        self.de.all = de
    }

    // HL
    pub fn get_h(&self) -> u8 {
        unsafe { self.hl.half.top }
    }
    pub fn set_h(&mut self, h: u8) {
        self.hl.half.top = h
    }

    pub fn get_l(&self) -> u8 {
        unsafe { self.hl.half.bot }
    }
    pub fn set_l(&mut self, l: u8) {
        self.hl.half.bot = l
    }

    pub fn get_hl(&self) -> u16 {
        unsafe { self.hl.all }
    }
    pub fn set_hl(&mut self, hl: u16) {
        self.hl.all = hl
    }

    // SP
    pub fn get_sp(&self) -> u16 {
        self.sp
    }
    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    // PC
    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    // Flags
    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        let f: u8 = self.get_f();
        let mut flag_set: u8 = 0x1;
        flag_set <<= flag as u8;

        if set {
            self.set_f(f | flag_set);
        } else {
            self.set_f(f & (!flag_set));
        }
    }
    pub fn get_flag(&self, flag: Flag) -> bool {
        if (self.get_f() >> (flag as u8)) & 0x1 == 0x1 {
            true
        } else {
            false
        }
    }

}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
         .field("AF", &format_args!("0x{:04X}", unsafe { &self.af.all} ))
         .field("BC", &format_args!("0x{:04X}", unsafe { &self.bc.all} ))
         .field("DE", &format_args!("0x{:04X}", unsafe { &self.de.all} ))
         .field("HL", &format_args!("0x{:04X}", unsafe { &self.hl.all} ))
         .field("SP", &format_args!("0x{:04X}", &self.sp ))
         .field("PC", &format_args!("0x{:04X}", &self.pc ))
         .finish()
    }
}
