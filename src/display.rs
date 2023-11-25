use std::{thread, time};
use embedded_graphics::pixelcolor::{Bgr565, RgbColor};
use embedded_graphics::draw_target::DrawTarget;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use embedded_hal::blocking::delay::DelayMs;
use st7735_lcd::{Orientation, ST7735};

const LCD_RST_PIN: u8 = 27;
const LCD_DC_PIN: u8 = 25;
// const LCD_CS_PIN: u8 = 8;
const LCD_BL_PIN: u8 = 24;

const LCD_WIDTH: u32 = 128;
const LCD_HEIGHT: u32 = 128;

pub struct Delay;

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        thread::sleep(time::Duration::from_millis(ms as u64));
    }
}

pub struct Display {
    bl_pin: OutputPin,
    delay: Delay,
    pub device: ST7735,
}

impl Display {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        let rst_pin = gpio.get(LCD_RST_PIN).unwrap().into_output();
        let dc_pin = gpio.get(LCD_DC_PIN).unwrap().into_output();
        let bl_pin = gpio.get(LCD_BL_PIN).unwrap().into_output();
        let spi = Spi::new(
            Bus::Spi0, SlaveSelect::Ss0, 9_000_000, Mode::Mode0).unwrap();

        let device = ST7735::new(
            spi, dc_pin, rst_pin, true, false, LCD_WIDTH, LCD_HEIGHT);

        let delay = Delay {};

        Self {
            bl_pin,
            delay,
            device,
        }
    }

    pub fn setup(&mut self) {
        // Turn on the backlight
        self.bl_pin.set_high();

        // Hardware reset
        self.reset();
    }

    fn reset(&mut self) {
        self.device.init(&mut self.delay).unwrap();
        self.device.set_orientation(&Orientation::Landscape).unwrap();
        self.device.set_offset(0, 0);
        self.device.clear(Bgr565::BLACK).unwrap();
    }
}
