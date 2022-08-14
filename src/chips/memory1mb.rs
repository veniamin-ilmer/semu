use crate::MemoryMsg;

pub struct Memory {
  ram: Vec<u8>,
}

pub fn start(bios_rom: &mut Vec<u8>, video_rom: &mut Vec<u8>) -> Memory {
  let mut memory = Memory {
    ram: vec![0u8; 0xC_0000],
  };
  
  memory.ram.append(video_rom);
  
  memory.ram.resize(0xF_0000, 0); //Fill up the remainder of space up to the bios.
  
  if bios_rom.len() != 0x1_0000 {
    panic!("The ROM size is wrong: {:X}. It must be size 0x10000.", bios_rom.len());
  }
  memory.ram.append(bios_rom);
  
  memory
}

impl Memory {
  pub fn process_msg(&mut self, msg: MemoryMsg) {
    match msg {
      MemoryMsg::SetByte{addr, value} => {
        self.ram[addr] = value;
      },
      MemoryMsg::SetWord{addr, value} => {
        let bytes = value.to_le_bytes();
        let (_left, right) = self.ram.split_at_mut(addr);
        let (middle, _) = right.split_at_mut(2);
        middle.copy_from_slice(&bytes[..]);
      },
      MemoryMsg::GetByte{addr, socket} => {
        socket.send(self.ram[addr]).unwrap();
      },
      MemoryMsg::GetWord{addr, socket} => {
        let slice = &self.ram[addr..addr+2];
        let word = u16::from_le_bytes(slice.try_into().unwrap());
        socket.send(word).unwrap();
      },
      MemoryMsg::GetBytes8{addr, socket} => {
        let slice = &self.ram[addr..addr+8];
        let bytes8 = u64::from_le_bytes(slice.try_into().unwrap());
        socket.send(bytes8).unwrap();
      },
    }
  }
}