#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians
//direction is set to point to qibala direction at home 
        let dir = if theta < -7. * PI / 8. {
            Direction::West
        } else if theta < -5. * PI / 8. {
            Direction::Southwest
        } else if theta < -3. * PI / 8. {
            Direction::South
        } else if theta < -PI / 8. {
            Direction::Southeast
        } else if theta < PI / 8. {
            Direction::East
        } else if theta < 3. * PI / 8. {
            Direction::Northeast
        } else if theta < 5. * PI / 8. {
            Direction::North
        } else if theta < 7. * PI / 8. {
            Direction::Northwest
        } else {
            Direction::North
        };

        iprintln!(&mut itm.stim[0],"{:?}",lsm303dlhc.mag().unwrap());

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(1000_u16);
    }
}