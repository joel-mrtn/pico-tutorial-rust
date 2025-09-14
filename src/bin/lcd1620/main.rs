#![no_std]
#![no_main]

use fugit::RateExtU32;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use rp_pico::{
    Pins, entry,
    hal::{self, Clock},
    pac,
};

use panic_halt as _;
use ufmt::uwrite;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = hal::Sio::new(p.SIO);
    let mut watchdog = hal::Watchdog::new(p.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(p.TIMER, &mut p.RESETS, &clocks);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);
    let sda = pins.gpio4.reconfigure();
    let scl = pins.gpio5.reconfigure();

    let i2c = hal::I2C::i2c0(
        p.I2C0,
        sda,
        scl,
        400.kHz(),
        &mut p.RESETS,
        &clocks.peripheral_clock,
    );

    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, timer);
    if let Err(e) = lcd.init() {
        panic!("Error initializing LCD: {}", e);
    }
    _ = lcd.backlight(true);
    _ = lcd.clear();
    _ = lcd.home();

    _ = uwrite!(&mut lcd, "Hello, World!");
    _ = lcd.set_cursor(0, 1);
    _ = uwrite!(&mut lcd, "Counter:");

    #[allow(clippy::empty_loop)]
    loop {
        _ = lcd.set_cursor(9, 1);
        let time = timer.get_counter().ticks() / 1_000_000;
        _ = uwrite!(&mut lcd, "{}", time);
        delay.delay_ms(1000);
    }
}
