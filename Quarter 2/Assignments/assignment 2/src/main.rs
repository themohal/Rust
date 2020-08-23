#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{String, Vec};
use heapless::consts::*;

#[entry]
fn main() -> ! {
    let (usart1, mut itm) = aux11::init();

    // // Send a string
    // for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
    //     // wait until it's safe to write to TDR
    //     while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

    //     usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    //     let mut v: Vec<u8, U8> = Vec::new();
    //     v.push(*byte).unwrap();
    //     let data_itm = String::from_utf8(v).unwrap();
    //     iprintln!(&mut itm.stim[0],"Data Received :{}",data_itm);
    // }

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;
        let mut v: Vec<u8, U8> = Vec::new();

        v.push(_byte as u8).unwrap();

        let data_itm = String::from_utf8(v).unwrap();
        iprintln!(&mut itm.stim[0],"Data Received :{}",data_itm);
        aux11::bkpt();
        
       
    }
}
