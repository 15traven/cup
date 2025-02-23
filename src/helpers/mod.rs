mod private;
mod public;

use private::get_system_theme;

pub use public::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColorMode {
    Dark,
    Light,
    Unspecified
}

impl From<u8> for ColorMode {
    fn from(value: u8) -> Self {
        if value == 1 {
            Self::Light
        } else if value == 0 {
            Self::Dark
        } else {
            Self::Unspecified
        }
    }
}