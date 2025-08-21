use rp_pico as bsp;

use bsp::hal::gpio::{self, FunctionSio, PullDown, SioInput, ValidFunction};
use embedded_hal::digital::InputPin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

pub trait ButtonExt {
    type Error;

    fn state(&mut self) -> Result<ButtonState, Self::Error>;
}

impl<I> ButtonExt for gpio::Pin<I, FunctionSio<SioInput>, PullDown>
where
    I: ValidFunction<FunctionSio<SioInput>>,
{
    type Error = core::convert::Infallible;

    fn state(&mut self) -> Result<ButtonState, Self::Error> {
        if self.is_low()? {
            Ok(ButtonState::Pressed)
        } else {
            Ok(ButtonState::Released)
        }
    }
}
