use std::sync::mpsc;

pub struct Memory {
  pub es: u16,  //Extra
  pub cs: u16,  //Code
  pub ss: u16,  //Stack
  pub ds: u16,  //Data
  
  pub ip: u16,  //Instruction
  
  pub current_segment: Segment,

  pub messenger: mpsc::Sender<crate::Msg>,

  pub current_instruction: u64,

}

pub enum Segment {
  ES, CS, SS, DS,
}

pub fn calculate_addr(segment: u16, offset: u16) -> usize {
  let segment = (segment as usize) << 4;
  segment + offset as usize
}

impl Memory {
  
  pub fn set_byte_msg(&mut self, addr: usize, value: u8) {
    let msg = crate::Msg::Memory(crate::MemoryMsg::SetByte{addr, value});
    self.messenger.send(msg).unwrap();
  }
  pub fn set_word_msg(&mut self, addr: usize, value: u16) {
    let msg = crate::Msg::Memory(crate::MemoryMsg::SetWord{addr, value});
    self.messenger.send(msg).unwrap();
  }

  pub fn get_byte_msg(&self, addr: usize) -> u8 {
    let (socket, rx) = mpsc::channel();
    let msg = crate::Msg::Memory(crate::MemoryMsg::GetByte{addr, socket});
    self.messenger.send(msg).unwrap();
    rx.recv().unwrap()
  }
  pub fn get_word_msg(&self, addr: usize) -> u16 {
    let (socket, rx) = mpsc::channel();
    let msg = crate::Msg::Memory(crate::MemoryMsg::GetWord{addr, socket});
    self.messenger.send(msg).unwrap();
    rx.recv().unwrap()
  }
  
  pub fn next_byte(&mut self) -> u8 {
    (*self).ip += 1;
    let byte = self.current_instruction & 0xFF;
    self.current_instruction >>= 8;
    byte as u8
  }

  pub fn next_word(&mut self) -> u16 {
    (*self).ip += 2;
    let word = self.current_instruction & 0xFFFF;
    self.current_instruction >>= 2*8;
    word as u16
  }

  pub fn set_byte(&mut self, offset: u16, byte: u8) {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.set_byte_msg(addr, byte);
  }
  
  pub fn get_byte(&self, offset: u16) -> u8 {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.get_byte_msg(addr)
  }

  pub fn set_word(&mut self, offset: u16, word: u16) {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.set_word_msg(addr, word);
  }

  pub fn get_word(&self, offset: u16) -> u16 {
    let addr = calculate_addr(self.get_seg(&self.current_segment), offset);
    self.get_word_msg(addr)
  }
  
  pub fn get_current_address(&self) -> usize {
    calculate_addr(self.cs, self.ip)
  }
  
  pub fn set_seg(&mut self, seg: &Segment, value: u16) {
    match seg {
      Segment::ES => self.es = value, Segment::CS => self.cs = value, Segment::SS => self.ss = value, Segment::DS => self.ds = value,
    }
  }
  pub fn get_seg(&self, seg: &Segment) -> u16 {
    match seg {
      Segment::ES => self.es, Segment::CS => self.cs, Segment::SS => self.ss, Segment::DS => self.ds,
    }
  }
}