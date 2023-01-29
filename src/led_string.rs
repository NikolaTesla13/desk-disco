use crate::led_pattern::LedPattern;
use arduino_hal::clock::MHz16;
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::hal::Usart;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::Spi;
use arduino_hal::{pac::USART0, port::mode::Input};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

const MAX_LEDS: usize = 200;

type Ws2812Driver = Ws2812<Spi>;
type Serial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>;

pub struct LedString {
    pub physical_length: u32,
    pub leds_count: u32,

    pattern: LedPattern,
    data: [RGB8; MAX_LEDS],
}

impl LedString {
    pub fn new(physical_length: u32, leds_per_meter: u32) -> LedString {
        let leds: u32 = leds_per_meter * physical_length;
        let mut data = [RGB8::default(); MAX_LEDS];

        for i in 0..leds {
            data[i as usize] = RGB8::default();
        }

        return LedString {
            physical_length,
            leds_count: leds,

            pattern: LedPattern::default(),
            data: data,
        };
    }

    pub fn add_pattern(&mut self, new_pattern: LedPattern, serial: &mut Serial) {
        self.pattern = new_pattern;
        self.update_data(serial);
    }
    pub fn clear(&self, driver: &mut Ws2812Driver) {
        let empty = [RGB8::default(); MAX_LEDS];
        driver.write(empty.iter().cloned()).unwrap();
    }

    pub fn update(&mut self, driver: &mut Ws2812Driver) {
        driver.write(self.data.iter().cloned()).unwrap();
        self.data.rotate_right(1);
    }

    fn update_data(&mut self, serial: &mut Serial) {
        let block_size: usize = self.leds_count as usize / self.pattern.colors.len();
        let brightness = Some(100);

        let mut pos = 0;

        if block_size * self.pattern.colors.len() > self.leds_count as usize {
            ufmt::uwriteln!(
                serial,
                "Not enough LEDs! Currently {:?} but {:?} are needed\r",
                self.leds_count,
                block_size * self.pattern.colors.len()
            )
            .unwrap();
            return;
        }

        ufmt::uwriteln!(serial, "Changing the pattern...\r").unwrap();

        for color in self.pattern.colors {
            for i in pos..(pos + block_size) {
                self.data[i] = color.to_rgb8(brightness)
            }
            pos += block_size;
        }
    }
}
