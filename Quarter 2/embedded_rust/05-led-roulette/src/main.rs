#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry,Delay,Leds,prelude::*};

#[entry]
fn main() -> ! {
    let (mut delay,mut leds) :(Delay,Leds) = aux5::init();

    let period = 500_u16;
    // infinite loop; just so we don't leave this stack frame
    loop {

        leds[0].on();
        leds[2].on();
        leds[4].on();
        leds[6].on();
        delay.delay_ms(period);
        leds[1].on();
        leds[3].on();
        leds[5].on();
        leds[7].on(); 
        delay.delay_ms(period);
        leds[0].off();
        leds[2].off();
        leds[4].off();
        leds[6].off();
        delay.delay_ms(period);
        leds[1].off();
        leds[3].off();
        leds[5].off();
        leds[7].off();
        delay.delay_ms(period);

    }
}
