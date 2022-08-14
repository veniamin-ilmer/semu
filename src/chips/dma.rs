//Intel 8237 / AMD 1980 - Direct Memory Access (DMA)
//https://wiki.osdev.org/ISA_DMA

use log::{debug,error};
use super::shared::FlipFlop;

#[derive(Default)]
pub struct DMA {
  enabled: bool,
  channel_0: Channel,
  channel_1: Channel,
  channel_2: Channel,
  channel_3: Channel,
}

#[derive(Default,Debug)]
enum TransferType {
  #[default]
  SelfTest,
  WriteToMemory,
  ReadFromMemory,
}

#[derive(Default,Debug)]
enum TransferMode {
  OnDemand,
  #[default]
  SingleDMA,
  BlockDMA,
  Cascade,
}

#[derive(Default)]
struct Channel {
  mask: bool,
  address: u16,
  count: u16,
  flip_flop: FlipFlop,
  transfer_type: TransferType,
  transfer_mode: TransferMode,
}

pub fn start() -> DMA {
  Default::default()
}

impl DMA {
  
  pub fn set_status(&mut self, register: u8) {
    self.enabled = matches!(register & 0b100, 0b100);
    debug!("Enabled: {}", self.enabled);
  }
  
  pub fn get_status(&mut self) -> u8 {
    //TODO - implement this
    debug!("Get Status");
    0
  }
  
  fn get_channel(&mut self, channel_index: u8) -> &mut Channel {
    match channel_index {
      0 => &mut self.channel_0,
      1 => &mut self.channel_1,
      2 => &mut self.channel_2,
      3 => &mut self.channel_3,
      _ => unreachable!(),
    }
  }
  pub fn get_count(&mut self, channel_index: u8) -> u8 {
    let channel = self.get_channel(channel_index);
    let count_piece = match channel.flip_flop {
      FlipFlop::Low => {
        channel.flip_flop = FlipFlop::High;
        channel.count
      },
      FlipFlop::High => {
        channel.flip_flop = FlipFlop::Low;
        channel.count >> 8
      },
    } as u8;
    debug!("Get Channel {} Count Piece: {:X}", channel_index, count_piece);
    count_piece
  }
  
  pub fn get_address(&mut self, channel_index: u8) -> u8 {
    let channel = self.get_channel(channel_index);
    let address_piece = match channel.flip_flop {
      FlipFlop::Low => {
        channel.flip_flop = FlipFlop::High;
        channel.address
      },
      FlipFlop::High => {
        channel.flip_flop = FlipFlop::Low;
        channel.address >> 8
      },
    } as u8;
    debug!("Get Channel {} Address Piece: {:X}", channel_index, address_piece);
    address_piece
  }

  
  pub fn set_address(&mut self, channel_index: u8, register: u8) {
    let channel = self.get_channel(channel_index);
    channel.address = match channel.flip_flop {
      FlipFlop::Low => {
        channel.flip_flop = FlipFlop::High;
        register as u16
      },
      FlipFlop::High => {
        channel.flip_flop = FlipFlop::Low;
        (channel.address & 0xFF) | ((register as u16) << 8)
      },
    };
    debug!("Set Channel {} Address: {:X}", channel_index, channel.address);
  }

  pub fn set_count(&mut self, channel_index: u8, register: u8) {
    let channel = self.get_channel(channel_index);
    channel.count = match channel.flip_flop {
      FlipFlop::Low => {
        channel.flip_flop = FlipFlop::High;
        register as u16
      },
      FlipFlop::High => {
        channel.flip_flop = FlipFlop::Low;
        (channel.count & 0xFF) | ((register as u16) << 8)
      },
    };
    debug!("Set Channel {} Count: {:X}", channel_index, channel.count);
  }

  pub fn reset_master(&mut self) {
    //TODO - Master Reset sets Flip-Flop low, clears Status, and sets all Mask bits ON.
    self.channel_0.mask = true;
    self.channel_0.flip_flop = FlipFlop::Low;
    self.channel_1.mask = true;
    self.channel_1.flip_flop = FlipFlop::Low;
    self.channel_2.mask = true;
    self.channel_2.flip_flop = FlipFlop::Low;
    self.channel_3.mask = true;
    self.channel_3.flip_flop = FlipFlop::Low;
    debug!("Master Reset");
  }
  
  pub fn reset_flip_flop(&mut self) {
    self.channel_0.flip_flop = FlipFlop::Low;
    self.channel_1.flip_flop = FlipFlop::Low;
    self.channel_2.flip_flop = FlipFlop::Low;
    self.channel_3.flip_flop = FlipFlop::Low;
    debug!("Flip-Flop Reset");
  }

  pub fn reset_mask(&mut self) {
    //Mask Reset sets all Mask bits OFF. 
    self.channel_0.mask = false;
    self.channel_1.mask = false;
    self.channel_2.mask = false;
    self.channel_3.mask = false;
    debug!("Mask Reset");
  }
  
  pub fn set_masks(&mut self, register: u8) {
    self.channel_0.mask = matches!(register & 0b1, 0b1);
    self.channel_1.mask = matches!(register & 0b10, 0b10);
    self.channel_2.mask = matches!(register & 0b100, 0b100);
    self.channel_3.mask = matches!(register & 0b1000, 0b1000);
    debug!("Mask 1: {}, Mask 2: {}, Mask 3: {}, Mask 4: {}", self.channel_0.mask, self.channel_1.mask, self.channel_2.mask, self.channel_3.mask);
  }
  
  pub fn set_mask(&mut self, register: u8) {
    let channel_index = register & 0b11;
    let channel = self.get_channel(channel_index);
    channel.mask = matches!(register & 0b100, 0b100);
    debug!("Mask {}: {}", channel_index, channel.mask);
  }
  
  pub fn set_mode(&mut self, register: u8) {
    let channel_index = register & 0b11;
    let channel = self.get_channel(channel_index);
    channel.transfer_type = match (register & 0b1100) >> 2 {
      0 => TransferType::SelfTest,
      1 => TransferType::WriteToMemory,
      2 => TransferType::ReadFromMemory,
      _ => {error!("Invalid Transfer type 3 chosen."); TransferType::SelfTest},
    };
    //TODO 0b1_0000 = AUTO
    //TODO 0b10_0000 = DOWN
    channel.transfer_mode = match (register & 0b1100_0000) >> 6 {
      0 => TransferMode::OnDemand,
      1 => TransferMode::SingleDMA,
      2 => TransferMode::BlockDMA,
      3 | _ => TransferMode::Cascade,
    };
    debug!("Channel {} {:?} {:?}", channel_index, channel.transfer_type, channel.transfer_mode);
  }
  
}