//8253/8254 - Programmable Interval Timer (PIT)
//https://stanislavs.org/helppc/8253.html
//https://wiki.osdev.org/Pit

use super::shared::FlipFlop;
use std::sync::{mpsc, Arc, Mutex};
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
  #[default]
  LSB,
  MSB,
  LSBThenMSB,
}

#[derive(Debug, Default)]
struct Mutexed {
  output_latch: u16,
  is_latched: bool,
  is_interrupt_mode: bool,
}

#[derive(Debug)]
struct Counter {
  access: Access,
  trigger_count_register: mpsc::Sender<u16>,
  mutex: Arc<Mutex<Mutexed>>,
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

pub fn start(messenger: mpsc::Sender<crate::Msg>) -> PIT {
  PIT {
    counter_0: new_counter(0, messenger.clone()),
    counter_1: new_counter(1, messenger.clone()),
    counter_2: new_counter(2, messenger.clone()),
  }
}

/// Datasheet:
/// Note that the CE (counting_element) cannot be written into; whenever a
/// count is written, it is written into the CR (count_register).
fn new_counter(select_counter: u8, messenger: mpsc::Sender<crate::Msg>) -> Counter {
  let mutex_arc = Arc::new(Mutex::new(Mutexed { ..Default::default() }));
  let mutex = Arc::clone(&mutex_arc);
  
  let (trigger_count_register, new_count_register) = mpsc::channel();
  
  thread::spawn(move || {
    let mut old_time = time::SystemTime::now();
    let mut enabled = false;
    let mut counting_element = 0u16;
    let mut initial_count_register = 0u16;
    loop {
      if !enabled {
        //When not enabled, we block on read.
        if let Ok(count_register) = new_count_register.recv() {
          old_time = time::SystemTime::now();
          trace!("PIT Enabled");
          initial_count_register = count_register;
          counting_element = count_register;
          enabled = true;
        }
      } else {
        //Enabled. Check if there is a new count, but don't block on read.
        if let Ok(count_register) = new_count_register.try_recv() {
          initial_count_register = count_register;
          counting_element = count_register;
        }
        let mut count_big = counting_element as u128;
        if count_big == 0 { count_big = 0x10000; }
        thread::yield_now();  //Windows sleep is known to be extremely inaccurate, so I just yield a little and calculate how much time has passed.
        let nanos = old_time.elapsed().unwrap().as_nanos();
        old_time = time::SystemTime::now();
        let ticks = nanos / 838;  //How many times the real hardware would have ticked.

        let mut trigger_interrupt = false;
        {
          let mut locked = mutex.lock().unwrap();
          if count_big > ticks {
            counting_element = (count_big - ticks) as u16;
          } else {
            if (*locked).is_interrupt_mode {
              counting_element = 0;
              enabled = false;
              trigger_interrupt = true;
            } else {
              //Start over again
              counting_element = initial_count_register;
            }
          }
          if !(*locked).is_latched {
            (*locked).output_latch = counting_element;
          }
        }
        if trigger_interrupt {
          trace!("Interrupting PIC");
          let msg = crate::Msg::PIC(crate::PICMsg::PIT{select_counter});
          messenger.send(msg).unwrap();
        }
      }
    }
  });
  
  Counter {
    trigger_count_register,
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
      let mut locked = counter.mutex.lock().unwrap();
      (*locked).is_latched = true;
      debug!("Counter {}: Latched! Output Latch: {}", select_counter, (*locked).output_latch);
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
      {
        let mut locked = counter.mutex.lock().unwrap();
        (*locked).is_interrupt_mode = matches!(counter.mode, Mode::Interrupt);
      }
      
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
      counter.trigger_count_register.send(count).unwrap();
    } else {
      debug!("Counter {}'s was given a flipflop low count {:X}", select_counter, counter.low_count);
    }
  }
  pub fn get_count(&mut self, select_counter: u8) -> u8 {
    let counter = self.get_counter(select_counter);
    
    let mut release_latch = true;
    let mut locked = counter.mutex.lock().unwrap();
    let count_u8 = match counter.access {
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
    } as u8;
    if release_latch {
      (*locked).is_latched = false;
    }
    
    debug!("Read Counter {}'s count {:X}", select_counter, count_u8);
    count_u8
  }
}