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
        iprintln!(&mut itm.stim[0],"Value of Theta: {:?}",theta);

//direction is set to point to qibala direction at home 
        let dir = if theta < -7. * PI / 8. {
            iprintln!(&mut itm.stim[0],"West Led");
            Direction::West
        } else if theta < -5. * PI / 8. {
            iprintln!(&mut itm.stim[0],"SouthWest Led");
            Direction::Southwest
        } else if theta < -3. * PI / 8. {
            iprintln!(&mut itm.stim[0],"South Led");
            Direction::South
        } else if theta < -PI / 8. {
            iprintln!(&mut itm.stim[0],"SouthEast Led");
            Direction::Southeast
        } else if theta < PI / 8. {
            iprintln!(&mut itm.stim[0],"East Led");
            Direction::East
        } else if theta < 3. * PI / 8. {
            iprintln!(&mut itm.stim[0],"NorthEast Led");
            Direction::Northeast
        } else if theta < 5. * PI / 8. {
            iprintln!(&mut itm.stim[0],"North Led");
            Direction::North
        } else if theta < 7. * PI / 8. {
            iprintln!(&mut itm.stim[0],"NorthWest Led");
            Direction::Northwest
        } else {
            iprintln!(&mut itm.stim[0],"North Led");
            Direction::North
        };

        iprintln!(&mut itm.stim[0],"{:?}",lsm303dlhc.mag().unwrap());

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(1000_u16);
    }
}