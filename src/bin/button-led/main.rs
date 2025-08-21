#![no_std]
#![no_main]

use panic_halt as _;

use rp_pico as bsp;

mod button;

use bsp::entry;
use bsp::hal::{
    Watchdog,
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
};
use button::{ButtonExt, ButtonState};
use embedded_hal::digital::StatefulOutputPin;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    let sio = Sio::new(p.SIO);

    let pins = bsp::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut led = pins.gpio15.into_push_pull_output();
    let mut button = pins.gpio13.into_pull_down_input();

    let mut last_state = ButtonState::Released;

    loop {
        let state = button.state().unwrap();

        if last_state == ButtonState::Pressed && state == ButtonState::Released {
            delay.delay_ms(40); // debounce
            led.toggle().unwrap();
        }

        last_state = state;
    }
}
