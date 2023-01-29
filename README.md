# desk-disco 🎉

Bring the party to your desk with desk-disco! This project allows you to play colorful patterns around your desk using smart LEDs and an Arduino UNO.

![video](https://media.discordapp.net/attachments/833285965019217980/1069231532570587176/tmp.gif)

## Getting Started

These instructions will help you get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust 1.64
- Arduino UNO
- WS2812B LEDs

### Installing

- Clone the repository onto your local machine using `git clone https://github.com/nikolatesla13/desk-disco.git`

- Open the terminal and navigate to the project directory using `cd desk-disco`
- Run `cargo build` to compile the code
- Upload the code onto your Arduino using your preferred method
- Connect the smart LEDs to your Arduino
- Turn on the power and watch the lights dance!

## Libraries used

- avr_hal
- arduino_hal
- smart_leds
- ws2812_spi

## License

This project is licensed under the GPLv3 License - see the LICENSE file for details. Copyright (C) 2023 Asandei Stefan-Alexandru.
