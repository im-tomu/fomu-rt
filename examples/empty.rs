#![no_std]
#![no_main]

extern crate panic_halt;
extern crate fomu_rt;

use fomu_rt::entry;

#[entry]
fn main() -> ! {
    // do something here
    loop { }
}
