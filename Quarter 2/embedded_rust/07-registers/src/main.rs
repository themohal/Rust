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