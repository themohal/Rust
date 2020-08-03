#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln,prelude::*, Delay};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init();
    let mut delay :Delay = gpioe.2;
    

   // let mut delay:Delay = aux7::init().2;
    let half_period = 500_u32;
        //   // Turn on the North LED
        //   gpioe.1.bsrr.write(|w| w.bs9().set_bit());
        //   delay.delay_ms(half_period);
        //   // Turn on the East LED
        //   gpioe.1.bsrr.write(|w| w.bs11().set_bit());
        //     delay.delay_ms(half_period);
      
        //   // Turn off the North LED
        //   gpioe.1.bsrr.write(|w| w.br9().set_bit());
        //   delay.delay_ms(half_period);
      
        //   // Turn off the East LED
        //   gpioe.1.bsrr.write(|w| w.br11().set_bit());
        //     //  delay.delay_ms(half_period);
    loop {
   
  // Turn on the North LED
  gpioe.1.bsrr.write(|w| w.bs9().set_bit());
  delay.delay_ms(half_period);
  // Turn on the East LED
  gpioe.1.bsrr.write(|w| w.bs11().set_bit());
    delay.delay_ms(half_period);

  // Turn off the North LED
  gpioe.1.bsrr.write(|w| w.br9().set_bit());
  delay.delay_ms(half_period);

  // Turn off the East LED
  gpioe.1.bsrr.write(|w| w.br11().set_bit());
    //  delay.delay_ms(half_period);
    }
}

// #![no_main]
// #![no_std]

// use core::ptr;

// #[allow(unused_imports)]
// use aux7::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     aux7::init();

//     unsafe {
//         ptr::read_volatile(0x4800_1800 as *const u32);
//     }

//     loop {}
// }

// #![no_main]
// #![no_std]

// use core::ptr;

// #[allow(unused_imports)]
// use aux7::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let mut itm = aux7::init().0;

//     unsafe {
//         const GPIOE_BSRR: u32 = 0x4800_1018;
//         const GPIOE_ODR: u32 = 0x4800_1014;

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn on the NORTH LED (red)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn on the EAST LED (green)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn off the NORTH LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );

//         // Turn off the EAST LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
//     }

//     loop {}
// }