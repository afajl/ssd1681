//! Simple no-std "Hello World" example for the Raspberry Pi Pico microcontroller board
//! with Dalian Good Display Tri-Color e-ink display 1.54 inch e-ink small display screen, [GDEM0154Z90](https://www.good-display.com/product/436.html)
//! using DESPI-C02 adapter https://buyepaper.com/products/development-kit-connection-adapter-board-for-eaper-display-demo-kit
//!
//! Connections:
//!
//! | Pico | DESPI-C02   |
//! |--------|-------|
//! | GP18   | SCK   |
//! | GP19   | MOSI  |
//! | GP17   | CS    |
//! | GP13   | BUSY  |
//! | GP12   | DC    |
//! | GP11   | RESET |
//!
//! To run this example clone this repository and run:
//! `cargo run --example dalian_1in54_tricolor

//#![no_std]
//#![no_main]


#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_arch = "arm")]
use defmt_rtt as _;
#[cfg(target_arch = "arm")]
use panic_probe as _;
#[cfg(target_arch = "arm")]
use rp_pico as bsp;

#[cfg(target_arch = "arm")]
use defmt::{info, println};
#[cfg(target_arch = "arm")]
use embedded_graphics::{
    geometry::{Point, Size},
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    prelude::Primitive,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
    Drawable,
};
#[cfg(target_arch = "arm")]
use embedded_hal::{
    delay::DelayNs,
    digital::StatefulOutputPin,
};
#[cfg(target_arch = "arm")]
use embedded_hal_bus::spi::ExclusiveDevice;
#[cfg(target_arch = "arm")]
use rp_pico::{
    entry,
    pac,
    hal::{
        clocks::init_clocks_and_plls,
        fugit::RateExtU32,
        gpio::FunctionSpi,
        spi,
        Clock,
        Sio,
        Watchdog,
    },
};
#[cfg(target_arch = "arm")]
use ssd1681::{
    color::{Black, Red, White},
    driver::Ssd1681,
    graphics::{Display, Display1in54, DisplayRotation},
    WIDTH,
};

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = DelayCompat(cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    ));

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    // These are implicitly used by the spi driver if they are in the correct mode
    let sck = pins.gpio18.into_function::<FunctionSpi>(); // SCK
    let mosi = pins.gpio19.into_function::<FunctionSpi>(); // SCL TX
    let miso = pins.gpio16.into_function::<FunctionSpi>(); // SDA RX
    let cs_spi = pins.gpio21.into_push_pull_output();

    let cs = pins.gpio17.into_push_pull_output();
    let dc = pins.gpio12.into_push_pull_output();
    let rst = pins.gpio11.into_push_pull_output();
    let busy = pins.gpio13.into_pull_down_input();

    // Create an SPI driver instance for the SPI0 device
    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI0, (mosi, miso, sck)).init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16_000_000u32.Hz(),
        embedded_hal::spi::MODE_0,
    );

    let mut spi_device = ExclusiveDevice::new_no_delay(spi, cs_spi).unwrap();

    // Initialize display controller
    let mut ssd1681 = Ssd1681::new(&mut spi_device, cs, busy, dc, rst, &mut delay).unwrap();

    // Clear frames on the display driver
    ssd1681.clear_red_frame(&mut spi_device);
    ssd1681.clear_bw_frame(&mut spi_device);

    // Create buffer for black and white
    let mut display_bw = Display1in54::bw();

    draw_rotation_and_rulers(&mut display_bw);

    display_bw.set_rotation(DisplayRotation::Rotate0);
    Rectangle::new(Point::new(60, 60), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(Black))
        .draw(&mut display_bw)
        .unwrap();

    println!("Send bw frame to display");
    ssd1681.update_bw_frame(&mut spi_device, display_bw.buffer());

    // Draw red color
    let mut display_red = Display1in54::red();

    Circle::new(Point::new(100, 100), 20)
        .into_styled(PrimitiveStyle::with_fill(Red))
        .draw(&mut display_red)
        .unwrap();

    println!("Send red frame to display");
    ssd1681.update_red_frame(&mut spi_device, display_red.buffer());

    println!("Update display");
    ssd1681.display_frame(&mut spi_device);

    println!("Done");

    loop {
        let _ = led_pin.toggle();
        delay.delay_ms(500);
    }
}
#[cfg(target_arch = "arm")]
fn draw_rotation_and_rulers(display: &mut Display1in54) {
    display.set_rotation(DisplayRotation::Rotate0);
    draw_text(display, "rotation 0", 25, 25);
    draw_ruler(display);

    display.set_rotation(DisplayRotation::Rotate90);
    draw_text(display, "rotation 90", 25, 25);
    draw_ruler(display);

    display.set_rotation(DisplayRotation::Rotate180);
    draw_text(display, "rotation 180", 25, 25);
    draw_ruler(display);

    display.set_rotation(DisplayRotation::Rotate270);
    draw_text(display, "rotation 270", 25, 25);
    draw_ruler(display);
}
#[cfg(target_arch = "arm")]
fn draw_ruler(display: &mut Display1in54) {
    for col in 1..WIDTH {
        if col % 25 == 0 {
            Line::new(Point::new(col as i32, 0), Point::new(col as i32, 10))
                .into_styled(PrimitiveStyle::with_stroke(Black, 1))
                .draw(display)
                .unwrap();
        }

        if col % 50 == 0 {
            let mut buf = [0u8; 4];
            let label = format_no_std::show(&mut buf, format_args!("{}", col)).unwrap();
            draw_text(display, &label, col as i32, 12);
        }
    }
}
#[cfg(target_arch = "arm")]
fn draw_text(display: &mut Display1in54, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Black)
        .background_color(White)
        .build();
    let _ = Text::new(text, Point::new(x, y), style).draw(display);
}

/// Wrapper around `Delay` to implement the embedded-hal 1.0 delay.
///
/// This can be removed when a new version of the `cortex_m` crate is released.
#[cfg(target_arch = "arm")]
struct DelayCompat(cortex_m::delay::Delay);
#[cfg(target_arch = "arm")]
impl embedded_hal::delay::DelayNs for DelayCompat {
    fn delay_ns(&mut self, mut ns: u32) {
        while ns > 1000 {
            self.0.delay_us(1);
            ns = ns.saturating_sub(1000);
        }
    }

    fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us);
    }
}
