#![no_std]
#![no_main]

mod led_pattern;
mod led_string;

use arduino_hal::spi;
use panic_halt as _;

use crate::led_pattern::LedPattern;
use crate::led_string::LedString;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Starting the LEDs...\r").unwrap();

    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    let mut ws = ws2812_spi::Ws2812::new(spi);

    let peppers = &["#fc1c03", "#fc1803", "#fc3d03", "#fc8B5e", "#fc6203"];
    let _cyberpunk = &[
        "#F000DB", "#621882", "#0A172D", "#0BC4CF", "#472183", "#4B56D2",
    ];

    let mut led_string = LedString::new(4, 50);
    led_string.clear(&mut ws);
    led_string.add_pattern(LedPattern::from_list(peppers), &mut serial);

    loop {
        led_string.update(&mut ws);
        arduino_hal::delay_ms(5);
    }
}
