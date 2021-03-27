# SSD1681 ePaper Display Driver

Rust driver for the [Solomon Systech SSD1681][SSD1681] e-Paper display (EPD)
controller, for use with [embedded-hal].

[![crates.io](https://img.shields.io/crates/v/ssd1681.svg)](https://crates.io/crates/ssd1681)
[![Documentation](https://docs.rs/ssd1681/badge.svg)](https://docs.rs/ssd1681/)


## Description

This driver is written for a [Adafruit 1.54" Tri-Color] display.
It will probably work for other displays with the same chip.

It is built using [embedded-hal] and optionally
[embedded-graphics]. 

## Examples
The examples must be built on a Raspberry PI. I use the
`run-example.sh` script to copy the sources, compile and run the
example on Raspberry Pi.

## Partial updates
Partial updates is not supported. There was support initially but
the driver refreshes the whole screen so there is no gain. 

According to Adafruit it seems to be a [hardware problem].

## Credits

* [Waveshare EPD driver](https://github.com/caemor/epd-waveshare)
* [SSD1675 EPD driver](https://github.com/wezm/ssd1675)
* [Adafruit_EPD](https://github.com/adafruit/Adafruit_EPD)
* [Adafruit CircuitPython EPD](https://github.com/adafruit/Adafruit_CircuitPython_EPD)

## License

`ssd1681` is dual licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) **or**
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[embedded-hal]: https://crates.io/crates/embedded-hal
[embedded-graphics]: https://github.com/embedded-graphics/embedded-graphics
[LICENSE-APACHE]: https://github.com/wezm/ssd1675/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/wezm/ssd1675/blob/master/LICENSE-MIT
[SSD1681]: https://www.solomon-systech.com/product/ssd1681
[hardware problem]: https://forums.adafruit.com/viewtopic.php?f=47&t=146252&p=722909&hilit=partial+update#p722957.

