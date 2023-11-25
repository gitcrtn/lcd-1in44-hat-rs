use std::collections::HashMap;
use int_enum::IntEnum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rppal::gpio::{Gpio, InputPin};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, EnumIter, Hash)]
pub enum Key {
    UP          = 6,
    DOWN        = 19,
    LEFT        = 5,
    RIGHT       = 26,
    CENTER      = 13,
    BUTTON1     = 21,
    BUTTON2     = 20,
    BUTTON3     = 16,
}

pub struct Gamepad {
    keys: HashMap<Key, InputPin>,
}

impl Gamepad {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        let mut keys = HashMap::new();
        for key in Key::iter() {
            let pin = gpio.get(key.int_value()).unwrap().into_input_pullup();
            keys.insert(key.clone(), pin);
        }
        Self {
            keys,
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys.get(&key).unwrap().is_low()
    }
}
