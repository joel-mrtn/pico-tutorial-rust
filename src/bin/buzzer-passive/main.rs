#![no_std]
#![no_main]

use panic_halt as _;

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{
    Watchdog,
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
};
use cortex_m::delay::Delay;
use embedded_hal::digital::{InputPin, OutputPin};

const VOLUME: f32 = 0.05;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut buzzer = pins.gpio15.into_push_pull_output();
    let mut button = pins.gpio16.into_pull_up_input();

    loop {
        if button.is_low().unwrap() {
            // Oscillate between 1500 Hz and 2500 Hz
            for x in (0..360).step_by(10) {
                let sin_val = libm::sinf(x as f32 * core::f32::consts::PI / 180.0);
                let tone_val = (2000.0 + sin_val * 500.0) as i32;

                set_freq(&mut buzzer, &mut delay, tone_val, 10, VOLUME);
            }
        } else {
            set_freq(&mut buzzer, &mut delay, 0, 10, VOLUME);
        }
        delay.delay_ms(10);
    }
}

fn set_freq<P: OutputPin>(pin: &mut P, delay: &mut Delay, freq: i32, times: i32, volume: f32) {
    if freq == 0 {
        let _ = pin.set_low();
    } else {
        let on_time = (1_000_000.0 / freq as f32 * volume) as u32;
        let off_time = (1_000_000.0 / freq as f32 * (1.0 - volume)) as u32;

        // times * freq / 1000 = number of cycles to play
        let cycles = times * freq / 1000;
        for _ in 0..cycles {
            let _ = pin.set_high();
            delay.delay_us(on_time);
            let _ = pin.set_low();
            delay.delay_us(off_time);
        }
    }
}
