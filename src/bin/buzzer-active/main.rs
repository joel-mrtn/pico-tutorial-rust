#![no_std]
#![no_main]

use panic_halt as _;

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use embedded_hal::digital::{InputPin, OutputPin};

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let sio = Sio::new(p.SIO);

    let pins = bsp::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let mut buzzer = pins.gpio15.into_push_pull_output();
    let mut button = pins.gpio16.into_pull_up_input();

    loop {
        if button.is_high().unwrap() {
            buzzer.set_low().unwrap();
        } else {
            buzzer.set_low().unwrap();
        }
    }
}
