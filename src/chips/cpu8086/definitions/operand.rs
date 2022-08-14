use super::memory::Memory;
use super::register::Registers;
use super::memory;
use super::register;

pub enum Pair {
  Bytes(Byte, Byte),
  Words(Word, Word),
}

pub enum Byte {
  Mem(u16, String),
  Reg(register::Byte),
  Imm(u8),
}

pub enum Word {
  Mem(u16, String),
  Reg(register::Word),
  Seg(memory::Segment),
  Imm(u16),
}

impl Byte {
  pub fn reg_index(nib: u8) -> Byte {
    let reg = match nib & 0b111 { //Doing & 111b is not necessary. I am doing it just in case.
      0 => register::Byte::AL,
      1 => register::Byte::CL,
      2 => register::Byte::DL,
      3 => register::Byte::BL,
      4 => register::Byte::AH,
      5 => register::Byte::CH,
      6 => register::Byte::DH,
      7 => register::Byte::BH,
      _ => unreachable!(),
    };
    Byte::Reg(reg)
  }
  
  //Gb
  pub fn general(op1: u8) -> Byte {
    let index = (op1 & 0b111000) >> 3;
    Byte::reg_index(index)
  }
  
  //Ob
  pub fn address(memory: &mut Memory) -> Byte {
    let addr = memory.next_word();
    let label = format!("{:X}", addr);
    Byte::Mem(addr, label)
  }
  
  //Eb
  pub fn extended(memory: &mut Memory, regs: &Registers, op1: u8) -> Byte {
    let ind1 = op1 & 7;
    match op1 {
      0x00..=0xBF => {
        let (mut addr, mut label) = match ind1 {
          0 => (regs.bx + regs.si, "BX+SI".to_string()),
          1 => (regs.bx + regs.di, "BX+DI".to_string()),
          2 => (regs.bp + regs.si, "BP+SI".to_string()),
          3 => (regs.bp + regs.di, "BP+DI".to_string()),
          4 => (regs.si, "SI".to_string()),
          5 => (regs.di, "DI".to_string()),
          6 => (regs.bp, "BP".to_string()),
          7 => (regs.bx, "BX".to_string()),
          _ => unreachable!(),
        };
        match op1 {
          0x00..=0x3F => {
            if ind1 == 0x6 { //Special case replacing bp
              addr = memory.next_word();
              label = format!("{:X}", addr);
            }
          },
          0x40..=0x7F => {
            let offset = memory.next_byte() as u16;
            addr += offset;
            label = format!("{}+{:X}", label, offset);
          },
          0x80..=0xBF => {
            let offset = memory.next_word();
            addr += offset;
            label = format!("{}+{:X}", label, offset);
          },
          _ => unreachable!(),
        };
        Byte::Mem(addr, label)
      },
      0xC0..=0xFF => {
        Byte::reg_index(ind1)
      }
    }
  }


}

impl Word {
  pub fn reg_index(index: u8) -> Word {
    let reg = match index & 7 { //Doing & 7 is not necessary. I am doing it just in case.
      0 => register::Word::AX,
      1 => register::Word::CX,
      2 => register::Word::DX,
      3 => register::Word::BX,
      4 => register::Word::SP,
      5 => register::Word::BP,
      6 => register::Word::SI,
      7 => register::Word::DI,
      _ => unreachable!(),
    };
    Word::Reg(reg)
  }
  
  pub fn seg_index(index: u8) -> Word {
    let seg = match index & 0b11 { //Doing & 0b11 is not necessary. I am doing it just in case.
      0 => memory::Segment::ES,
      1 => memory::Segment::CS,
      2 => memory::Segment::SS,
      3 => memory::Segment::DS,
      _ => unreachable!(),
    };
    Word::Seg(seg)
  }
  
  //Gw or Gv
  pub fn general(op1: u8) -> Word {
    let index = (op1 & 0b111000) >> 3;
    Word::reg_index(index)
  }
  
  //Ov
  pub fn address(memory: &mut Memory) -> Word {
    let addr = memory.next_word();
    let label = format!("{:X}", addr);
    Word::Mem(addr, label)
  }
  
  //Ew or Ev
  pub fn extended(memory: &mut Memory, regs: &Registers, op1: u8) -> Word {
    let ind1 = op1 & 7;
    match op1 {
      0x00..=0xBF => {
        let (mut addr, mut label) = match ind1 {
          0 => (regs.bx + regs.si, "BX+SI".to_string()),
          1 => (regs.bx + regs.di, "BX+DI".to_string()),
          2 => (regs.bp + regs.si, "BP+SI".to_string()),
          3 => (regs.bp + regs.di, "BP+DI".to_string()),
          4 => (regs.si, "SI".to_string()),
          5 => (regs.di, "DI".to_string()),
          6 => (regs.bp, "BP".to_string()),
          7 => (regs.bx, "BX".to_string()),
          _ => unreachable!(),
        };
        match op1 {
          0x00..=0x3F => {
            if ind1 == 0x6 { //Special case replacing bp
              addr = memory.next_word();
              label = format!("{:X}", addr);
            }
          },
          0x40..=0x7F => {
            let offset = memory.next_byte() as u16;
            addr += offset;
            label = format!("{}+{:X}", label, offset);
          },
          0x80..=0xBF => {
            let offset = memory.next_word();
            addr += offset;
            label = format!("{}+{:X}", label, offset);
          },
          _ => unreachable!(),
        };
        Word::Mem(addr, label)
      },
      0xC0..=0xFF => {
        Word::reg_index(ind1)
      }
    }
  }
  
  //Sw
  pub fn segment(op1: u8) -> Word {
    let index = (op1 & 0b11000) >> 3;
    Word::seg_index(index)
  }
}
