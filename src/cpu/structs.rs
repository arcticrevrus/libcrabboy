use crate::cpu::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AccumulatorAndFlags {
    pub accumulator: u8,
    pub flags: Flags,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flags {
    pub bits: u8,
}
#[allow(dead_code)]
impl Flags {
    pub fn new() -> Self {
        Flags { bits: 0 }
    }

    pub fn set(&mut self, flag: Flag, value: bool) {
        const Z_BIT: u8 = 0b1000_0000;
        const N_BIT: u8 = 0b0100_0000;
        const H_BIT: u8 = 0b0010_0000;
        const C_BIT: u8 = 0b0001_0000;
        let flag_bit = match flag {
            Flag::Z => Z_BIT,
            Flag::N => N_BIT,
            Flag::H => H_BIT,
            Flag::C => C_BIT,
        };
        if value {
            self.bits |= flag_bit;
        } else {
            self.bits &= !flag_bit;
        }
    }

    pub fn get(&self, flag: Flag) -> bool {
        const Z_BIT: u8 = 0b1000_0000;
        const N_BIT: u8 = 0b0100_0000;
        const H_BIT: u8 = 0b0010_0000;
        const C_BIT: u8 = 0b0001_0000;
        let flag_bit = match flag {
            Flag::Z => Z_BIT,
            Flag::N => N_BIT,
            Flag::H => H_BIT,
            Flag::C => C_BIT,
        };
        (self.bits & flag_bit) != 0
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BAndC {
    pub b: u8,
    pub c: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DAndE {
    pub d: u8,
    pub e: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HAndL {
    pub h: u8,
    pub l: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StackPointer {
    pub stackpointer: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProgramCounter {
    pub programcounter: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InterruptMasterEnable {
    pub ime: bool,
    pub has_waited: bool,
}
impl InterruptMasterEnable {
    pub fn check_interrupts(cpu: &mut Cpu, memory: &mut MemoryMap) {
        let interrupt_flags = memory.read(0xFFFF);
        let joypad = interrupt_flags & 0b0001_0000;
        let serial = interrupt_flags & 0b0000_1000;
        let timer = interrupt_flags & 0b0000_0100;
        let lcd = interrupt_flags & 0b0000_0010;
        let vblank = interrupt_flags & 0b0000_0001;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Registers {
    pub af: AccumulatorAndFlags,
    pub bc: BAndC,
    pub de: DAndE,
    pub hl: HAndL,
    pub sp: StackPointer,
    pub pc: ProgramCounter,
    pub ime: InterruptMasterEnable,
}
impl Registers {
    #[allow(dead_code)]
    pub fn read_u16(&mut self, register: Register) -> u16 {
        match register {
            Register::AF => ((self.af.accumulator as u16) << 8) | (self.af.flags.bits as u16),
            Register::BC => ((self.bc.b as u16) << 8) | (self.bc.c as u16),
            Register::DE => ((self.de.d as u16) << 8) | (self.de.e as u16),
            Register::HL(hlm) => match hlm {
                HLMode::Normal => ((self.hl.h as u16) << 8) | (self.hl.l as u16),
                HLMode::Increment => {
                    let return_value = ((self.hl.h as u16) << 8) | (self.hl.l as u16);
                    self.hl.l = self.hl.l.wrapping_add(1);
                    return_value
                }
                HLMode::Decrement => {
                    let return_value = ((self.hl.h as u16) << 8) | (self.hl.l as u16);
                    self.hl.l = self.hl.l.wrapping_sub(1);
                    return_value
                }
            },
            Register::SP => self.sp.stackpointer,
            Register::PC => self.pc.programcounter,
            _ => panic!("8 bit register given to 16 bit read operation"),
        }
    }

    #[allow(dead_code)]
    pub fn write_u16(&mut self, register: Register, value: u16) {
        let high_byte = (&value >> 8) as u8;
        let low_byte = (&value & 0xFF) as u8;
        match register {
            Register::AF => {
                self.af.accumulator = high_byte;
                self.af.flags.bits = low_byte;
            }
            Register::BC => {
                self.bc.b = high_byte;
                self.bc.c = low_byte;
            }
            Register::DE => {
                self.de.d = high_byte;
                self.de.e = low_byte;
            }
            Register::HL(_) => {
                self.hl.h = high_byte;
                self.hl.l = low_byte;
            }
            Register::SP => {
                self.sp.stackpointer = value;
            }
            Register::PC => {
                self.pc.programcounter = value;
            }
            _ => panic!("Attempted to write a 16 bit value to an 8 bit register"),
        }
    }
}
