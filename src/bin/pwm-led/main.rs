#![no_std]
#![no_main]

use panic_halt as _;

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{
    Watchdog,
    clocks::{Clock, init_clocks_and_plls},
    pac, pwm,
    sio::Sio,
};
use embedded_hal::pwm::SetDutyCycle;

const PWM_LOW: u16 = 0;
const PWM_HIGH: u16 = 255;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.set_top(255);
    pwm.enable();

    let channel = &mut pwm.channel_b;
    channel.output_to(pins.led);

    loop {
        for i in PWM_LOW..=PWM_HIGH {
            delay.delay_ms(5);
            let _ = channel.set_duty_cycle(i);
        }

        for i in (PWM_LOW..PWM_HIGH).rev() {
            delay.delay_ms(5);
            let _ = channel.set_duty_cycle(i);
        }

        delay.delay_ms(500);
    }
}
