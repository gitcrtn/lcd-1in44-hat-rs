# lcd-1in44-hat-rs

## Overview
A helper library for Pi Zero with waveshare 1.44inch LCD HAT.

## Preparation
Enable SPI on Pi Zero.
```
sudo raspi-config nonint do_i2c 1
sudo raspi-config nonint do_spi 0
sudo reboot
```

## How to build
```
rustup target add arm-unknown-linux-gnueabihf
git clone https://github.com/raspberrypi/tools rpi_tools
git clone https://github.com/gitcrtn/lcd-1in44-hat-rs
cd lcd-1in44-hat-rs
cargo build
```

## Example
```
use lcd_1in44_hat::gamepad::{Gamepad, Key};
use lcd_1in44_hat::display::Display;
use embedded_graphics::pixelcolor::Bgr565;
use embedded_graphics::image::Image;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::prelude::*;
use tinytga::Tga;
use std::{thread, time};

fn delay_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

fn main() {
    let gamepad = Gamepad::new();
    let mut display = Display::new();
    display.setup();

    let tga: Tga<Bgr565> = Tga::from_slice(include_bytes!("../assets/something.tga")).unwrap();
    let image = Image::new(&tga, Point::zero());
    image.draw(&mut display.device).unwrap();

    Line::new(Point::new(16, 24), Point::new(51, 34))
        .into_styled(PrimitiveStyle::with_stroke(Bgr565::RED, 8))
        .draw(&mut display.device).unwrap();

    loop {
        delay_ms(20);

        if gamepad.is_pressed(Key::UP) {
            println!("up");
        }
    }
}
```

## License
[MIT](https://github.com/gitcrtn/lcd-1in44-hat-rs/blob/main/LICENSE)

## Author
[Carotene](https://github.com/gitcrtn)