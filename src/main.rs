#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use mk64fx512v::{sim, portc, gpioc};

/*
pub fn delay_ms(x: u32) {
    unsafe {
        let y = (*rtt()).vr.read() + x;
        // FIXME: deal with overflow!
        while (*rtt()).vr.read() < y {}
    }
}
*/

#[entry]
fn main() -> ! {
    unsafe { (*sim()).scgc5.write(1 << 11) }                            // port C clock gate control
    unsafe { (*portc()).pcr5.write((1 << 8) | (1 << 6) | (1 << 2)) }    // port C pin 5 GPIO | DSE | SRE
    unsafe { (*gpioc()).pddr.write(1 << 5) }                            // port C pin 5 Output direction
//    unsafe { (*rtt()).mr.write(32768 / 1000) }
    let mut on = true;

    loop {
//        delay_ms(250);
        if on {
            unsafe { (*gpioc()).pcor.write(1 << 5) }
        } else {
            unsafe { (*gpioc()).psor.write(1 << 5) }
        }
        on = !on;
    }
}

