#![no_std]
#![no_main]

use panic_halt as _;

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{
    Watchdog,
    clocks::{Clock, init_clocks_and_plls},
    gpio::{DynPinId, FunctionSio, Pin, PullDown, SioOutput},
    pac,
    sio::Sio,
};
use embedded_hal::digital::OutputPin;

const DELAY: u32 = 100;

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

    let mut led_pins: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; _] = [
        pins.gpio16.into_push_pull_output().into_dyn_pin(),
        pins.gpio17.into_push_pull_output().into_dyn_pin(),
        pins.gpio18.into_push_pull_output().into_dyn_pin(),
        pins.gpio19.into_push_pull_output().into_dyn_pin(),
        pins.gpio20.into_push_pull_output().into_dyn_pin(),
        pins.gpio21.into_push_pull_output().into_dyn_pin(),
        pins.gpio22.into_push_pull_output().into_dyn_pin(),
        pins.gpio26.into_push_pull_output().into_dyn_pin(),
        pins.gpio27.into_push_pull_output().into_dyn_pin(),
        pins.gpio28.into_push_pull_output().into_dyn_pin(),
    ];

    let mut current_led: isize = 0;
    let mut direction: isize = 1;
    let led_count = led_pins.len() as isize;

    loop {
        led_pins[current_led as usize].set_high().unwrap();
        delay.delay_ms(DELAY);
        led_pins[current_led as usize].set_low().unwrap();

        current_led += direction;

        if current_led == led_count || current_led < 0 {
            direction = -direction;
            current_led += 2 * direction;
        }
    }
}
