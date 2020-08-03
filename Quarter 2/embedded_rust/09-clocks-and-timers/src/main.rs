#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]

// fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
//     // TODO implement this
//     // for _ in 0..1_000{}
//     const K: u16 = 5; // this value needs to be tweaked
//     for _ in 0..(K * ms) {
//         aux9::nop()
//     }
// }

fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    tim6.arr.write(|w|w.arr().bits(ms));
    tim6.cr1.modify(|_,w|w.cen().set_bit()); //counter start
    //uif => sr register to reset counter
    while !tim6.sr.read().uif().bit_is_set(){
        //keep it busy until timer
    }
    tim6.sr.modify(|_, w| w.uif().clear_bit());

}
#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // TODO initialize TIM6
    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());   
    tim6.cr1.write(|w|w.opm().set_bit().cen().clear_bit());
    tim6.psc.write(|w|w.psc().bits(7_999)); //freq is 1khz
    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
