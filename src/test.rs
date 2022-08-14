//Initial Speed:
//1,054,878 Hz
//Without yield_now:
//20,200,000 Hz
//Moving away most of the mutexes
//19,572,257 Hz

//8253/8254 - Programmable Interval Timer (PIT)
//https://stanislavs.org/helppc/8253.html
//https://wiki.osdev.org/Pit

#[derive(Debug,Default)]
pub enum FlipFlop {
  #[default]
  Low,
  High,
}

use std::sync::{mpsc, Arc};
use std::thread;
use std::time;

use log::{debug, trace};

#[derive(Debug, Default)]
enum Mode {
  Interrupt,
  OneShot,
  RateGenerator,
  #[default]
  SquareWave,
  SoftwareStrobe,
  HardwareStrobe,
}

#[derive(Debug, Default)]
enum Access {
  LSB,
  #[default]
  MSB,
  LSBThenMSB,
}

#[derive(Debug, Default)]
struct Mutexed {
  output_latch: u16,
}

#[derive(Debug)]
struct Counter {
  access: Access,
  to_processor: mpsc::Sender<ChipMsg>,
  output_latch: u16,
  flip_flop: FlipFlop,  //This should only be true we read/wrote the LSB and are still waiting on MSB.
  low_count: u16,       //This should only be set with set_count(..) flip_flop Low.
  mode: Mode,
}
  

#[derive(Debug)]
pub struct PIT {
  counter_0: Counter, //Time of day clock. Normally mode SquareWave
  counter_1: Counter, //RAM refresher. Normally mode RateGenerator
  counter_2: Counter, //Misc / Sound.
}

pub enum Msg {
  Motherboard(MotherboardMsg),
  Memory(MemoryMsg),
  PIC(PICMsg),
  CPU(CPUMsg),
}

pub enum MotherboardMsg {
  OutByte {port: u16, value: u8},
  OutWord {port: u16, value: u16},
  InByte {port: u16, socket: mpsc::Sender<u8>},
  InWord {port: u16, socket: mpsc::Sender<u16>},
}
pub enum MemoryMsg {
  SetByte{addr: usize, value: u8},
  SetWord{addr: usize, value: u16},
  GetByte{addr: usize, socket: mpsc::Sender<u8>},
  GetWord{addr: usize, socket: mpsc::Sender<u16>},
  GetBytes8{addr: usize, socket: mpsc::Sender<u64>},
}

pub enum PICMsg {
  PIT{select_counter: u8},
}

pub enum CPUMsg {
  Interrupt(u8),
}

fn main() {
  let (tx, rx) = mpsc::channel();
  let mut pit = start(tx);
  pit.set_count(1,0xFF);
  loop {
    rx.recv().unwrap();
  }
}

pub fn start(messenger: mpsc::Sender<crate::Msg>) -> PIT {
  PIT {
    counter_0: new_counter(0, messenger.clone()),
    counter_1: new_counter(1, messenger.clone()),
    counter_2: new_counter(2, messenger.clone()),
  }
}

enum ChipMsg {
  EnableProcessor{count_register: u16},
  ControlWord{is_interrupt_mode: bool},
  SetLatch(bool),
}

/// Datasheet:
/// Note that the CE (counting_element) cannot be written into; whenever a
/// count is written, it is written into the CR (count_register).
fn new_counter(select_counter: u8, messenger: mpsc::Sender<crate::Msg>) -> Counter {
  let mutex_arc = Arc::new(Mutex::new(Mutexed { ..Default::default() }));
  let mutex = Arc::clone(&mutex_arc);
  
  let (to_processor, from_controller) = mpsc::channel();
  
  thread::spawn(move || {
    let mut init_time = time::SystemTime::now();
    let mut ticks = 0 as u128;
    let mut enabled = false;
    let mut is_latched = false;
    let mut is_interrupt_mode = false;
    let mut counting_element = 0u16;
    let mut initial_count_register = 0u16;
    loop {
      if !enabled {
        //When not enabled, we block on read.
        if let Ok(msg) = from_controller.recv() {
          match msg {
            ChipMsg::EnableProcessor{count_register} => {
              ticks = 0;
              println!("PIT Enabled");
              initial_count_register = count_register;
              counting_element = count_register;
              enabled = true;
              init_time = time::SystemTime::now();
            },
            ChipMsg::ControlWord{is_interrupt_mode: interrupt_mode} => is_interrupt_mode = interrupt_mode,
            ChipMsg::SetLatch(_) => (), //Latching a disabled Counter is undefined behavior.
          }
        }
      } else {
        //Enabled. Check if there is a new count, but don't block on read.
        if let Ok(msg) = from_controller.try_recv() {
          match msg {
            ChipMsg::EnableProcessor{count_register} => {
              initial_count_register = count_register;
              counting_element = count_register;
            },
            ChipMsg::ControlWord{is_interrupt_mode: interrupt_mode} => is_interrupt_mode = interrupt_mode,
            ChipMsg::SetLatch(latch) => is_latched = latch,
          }
        }
        let mut count_big = counting_element as u128;
        if count_big == 0 { count_big = 0x10000; }
//        thread::yield_now();  //Windows sleep is known to be extremely inaccurate, so I just yield a little and calculate how much time has passed.
        ticks += 1;
        let secs = init_time.elapsed().unwrap().as_secs() as u128;
        if secs >= 10 {
          panic!("{}", ticks/secs);
        }

        let mut trigger_interrupt = false;
        {
          if count_big > ticks {
            counting_element = (count_big - ticks) as u16;
          } else {
            if is_interrupt_mode {
              counting_element = 0;
              enabled = false;
              trigger_interrupt = true;
            } else {
              //Start over again
              counting_element = initial_count_register;
            }
          }
          if !is_latched {
            let mut locked = mutex.lock().unwrap();
            (*locked).output_latch = counting_element;
          }
        }
        if trigger_interrupt {
          println!("Interrupting PIC");
          let msg = crate::Msg::PIC(crate::PICMsg::PIT{select_counter});
          messenger.send(msg).unwrap();
        }
      }
    }
  });
  
  Counter {
    to_processor,
    access: Default::default(),
    flip_flop: Default::default(),
    mode: Default::default(),
    mutex: Arc::clone(&mutex_arc),
    low_count: 0,
  }
}

impl PIT {
  
  fn get_counter(&mut self, select_counter: u8) -> &mut Counter {
    match select_counter {
      0 => &mut self.counter_0,
      1 => &mut self.counter_1,
      2 => &mut self.counter_2,
      3 => unimplemented!(),  //TODO 8254 function "Read-back command"
      _ => unreachable!(),
    }
  }
  
  pub fn set_control_word(&mut self, register: u8) {
    let select_counter = (register & 0b1100_0000) >> 6;
    let counter = self.get_counter(select_counter);

    if (register & 0b11_0000) >> 4 == 0 {  //Latch mode
      counter.to_processor.send(ChipMsg::SetLatch(true)).unwrap();
      debug!("Counter {}: Latched!", select_counter);
    }
    else {  //Initialization mode
      //Hopefully I won't have to build this..
      if matches!(register & 0b1, 0b1) {
        unimplemented!("BCD requested for PIT..");
      }
      
      counter.mode = match (register & 0b1110) >> 1 {
        0 => Mode::Interrupt,
        1 => Mode::OneShot,
        2 | 6 => Mode::RateGenerator,
        3 | 7 => Mode::SquareWave,
        4 => Mode::SoftwareStrobe,
        5 | _ => Mode::HardwareStrobe,
      };
      counter.to_processor.send(ChipMsg::ControlWord{
        is_interrupt_mode: matches!(counter.mode, Mode::Interrupt)
      }).unwrap();
      
      counter.access = match (register & 0b11_0000) >> 4 {
        1 => Access::MSB,
        2 => Access::LSB,
        3 => {counter.flip_flop = FlipFlop::Low; Access::LSBThenMSB},
        _ => unreachable!(),
      };

      debug!("Counter {}: {:?}", select_counter, counter);
    }
  }
  
  /// Datasheet: if the Counter has been programmed for one
  /// byte counts (either most significant byte only or least
  /// significant byte only) the other byte will be zero.
  pub fn set_count(&mut self, select_counter: u8, register: u8) {
    let new_count = register as u16;
    let counter = self.get_counter(select_counter);

    let count_register: Option<u16> = match counter.access {
      Access::LSB => Some(new_count),
      Access::MSB => Some(new_count << 8),
      Access::LSBThenMSB => match counter.flip_flop {
        FlipFlop::Low => {
          counter.flip_flop = FlipFlop::High;
          counter.low_count = new_count;
          None  //Don't trigger yet. Wait for the next value to be given first.
        },
        FlipFlop::High => {
          counter.flip_flop = FlipFlop::Low;
          Some(counter.low_count + (new_count << 8))
        },
      },
    };
    if let Some(count) = count_register {
      debug!("Counter {}'s count_register was set to {:X}", select_counter, count);
      counter.to_processor.send(ChipMsg::EnableProcessor{count_register: count}).unwrap();
    } else {
      debug!("Counter {}'s was given a flipflop low count {:X}", select_counter, counter.low_count);
    }
  }
  pub fn get_count(&mut self, select_counter: u8) -> u8 {
    let counter = self.get_counter(select_counter);
    
    let mut release_latch = true;
    let count_u8 = {
      let locked = counter.mutex.lock().unwrap();
      match counter.access {
        Access::LSB => (*locked).output_latch & 0xFF,
        Access::MSB => (*locked).output_latch >> 8,
        Access::LSBThenMSB => match counter.flip_flop {
          FlipFlop::Low => {
            {
              release_latch = false;  //Don't release the latch yet, since we are only reading the low byte.
              counter.flip_flop = FlipFlop::High;
              (*locked).output_latch & 0xFF
            }
          },
          FlipFlop::High => {
            counter.flip_flop = FlipFlop::Low;
            (*locked).output_latch >> 8
          }
        },
      }
    } as u8;
    if release_latch {
      counter.to_processor.send(ChipMsg::SetLatch(false)).unwrap();
    }
    
    debug!("Read Counter {}'s count {:X}", select_counter, count_u8);
    count_u8
  }
}