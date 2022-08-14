trait Bytes {
  fn set_low(&mut self, value: u8);
  fn set_high(&mut self, value: u8);
  fn get_low(&self) -> u8;
  fn get_high(&self) -> u8;
}

impl Bytes for u16 {
  fn set_high(&mut self, high: u8) {
    let [low, _] = self.to_le_bytes();
    *self = Self::from_le_bytes([low, high]);
  }
  fn set_low(&mut self, low: u8) {
    let [_, high] = self.to_le_bytes();
    *self = Self::from_le_bytes([low, high]);
  }
  
  fn get_high(&self) -> u8 {
    let [_, high] = self.to_le_bytes();
    high
  }
  fn get_low(&self) -> u8 {
    let [low, _] = self.to_le_bytes();
    low
  }
}

pub enum Byte {
  AL, CL, DL, BL,
  AH, CH, DH, BH,
}

pub enum Word {
  AX, CX, DX, BX,
  SP, BP, SI, DI,
}

#[derive(Default)]
pub struct Registers {
  //Data Registers
  pub ax: u16,  //Accumulator
  pub cx: u16,  //Counter
  pub dx: u16,  //Data
  pub bx: u16,  //Base
  
  //Pointer Registers
  pub sp: u16,  //Stack
  pub bp: u16,  //Base
  
  //Index Registers
  pub si: u16,  //Source
  pub di: u16,  //Destination
}

impl Registers {
  pub fn set_byte(&mut self, reg: &Byte, value: u8) {
    match reg {
      Byte::AL => self.ax.set_low(value),  Byte::CL => self.cx.set_low(value),  Byte::DL => self.dx.set_low(value),  Byte::BL => self.bx.set_low(value),
      Byte::AH => self.ax.set_high(value), Byte::CH => self.cx.set_high(value), Byte::DH => self.dx.set_high(value), Byte::BH => self.bx.set_high(value),
    }
  }
  pub fn get_byte(&self, reg: &Byte) -> u8 {
    match reg {
      Byte::AL => self.ax.get_low(),  Byte::CL => self.cx.get_low(),  Byte::DL => self.dx.get_low(),  Byte::BL => self.bx.get_low(),
      Byte::AH => self.ax.get_high(), Byte::CH => self.cx.get_high(), Byte::DH => self.dx.get_high(), Byte::BH => self.bx.get_high(),
    }
  }
  
  pub fn set_word(&mut self, reg: &Word, value: u16) {
    match reg {
      Word::AX => self.ax = value, Word::CX => self.cx = value, Word::DX => self.dx = value, Word::BX => self.bx = value,
      Word::SP => self.sp = value, Word::BP => self.bp = value, Word::SI => self.si = value, Word::DI => self.di = value,
    }
  }
  pub fn get_word(&self, reg: &Word) -> u16 {
    match reg {
      Word::AX => self.ax, Word::CX => self.cx, Word::DX => self.dx, Word::BX => self.bx,
      Word::SP => self.sp, Word::BP => self.bp, Word::SI => self.si, Word::DI => self.di,
    }
  }
}