use embedded_hal::prelude::*;
use linux_embedded_hal::{
    spidev::{SpiModeFlags, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::PrimitiveStyle,
    text_style,
};

use ssd1681::{buffer_len, color::*, graphics::VarDisplay, prelude::*, WIDTH};

// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues
fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/
    // Configure Digital I/O Pins
    let cs = Pin::new(2); // GPIO/BCM 8, pin 24
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let reset = Pin::new(7); // GPIO/BCM 7, pin 26
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset
        .set_direction(Direction::Out)
        .expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");

    let busy = Pin::new(1); // GPIO/BCM 1, pin 28
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(12); // GPIO/BCM 12, pin 32
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    println!("Pins configured");

    let mut delay = Delay {};

    // Initialise display controller
    let mut ssd1681 = Ssd1681::new(&mut spi, cs, busy, dc, reset, &mut delay).unwrap();

    // Clear frames on the display driver
    ssd1681.clear_red_frame(&mut spi)?;
    ssd1681.clear_bw_frame(&mut spi)?;

    // Create buffer for black and white
    let mut display_bw = Display1in54::bw();

    draw_rotation_and_rulers(&mut display_bw);

    Rectangle::new(Point::new(80, 80), Point::new(120, 120))
        .into_styled(PrimitiveStyle::with_fill(Black))
        .draw(&mut display_bw)
        .unwrap();

    println!("Send bw frame to display");
    ssd1681.update_bw_frame(&mut spi, display_bw.buffer())?;

    // Draw red color
    let mut display_red = Display1in54::red();

    Circle::new(Point::new(100, 100), 30)
        .into_styled(PrimitiveStyle::with_fill(Red))
        .draw(&mut display_red)
        .unwrap();

    println!("Send red frame to display");
    ssd1681.update_red_frame(&mut spi, display_red.buffer())?;

    println!("Update display");
    ssd1681.display_frame(&mut spi)?;

    println!("Sleep 5s");
    delay.delay_ms(5000u16);

    // Draw partial update
    println!("Create a 40x40 display");
    let mut buffer = [Color::White.get_byte_value(); buffer_len(40, 40)];
    let mut display_partial = VarDisplay::bw(40, 40, &mut buffer);

    Rectangle::new(Point::new(0, 0), Point::new(40, 40))
        .into_styled(PrimitiveStyle::with_fill(Black))
        .draw(&mut display_partial)
        .unwrap();

    println!("Send partial frame to display");
    ssd1681
        .update_partial_bw_frame(&mut spi, display_partial.buffer(), 90, 90, 40, 40)
        .unwrap();

    println!("Update display");
    ssd1681.display_partial_frame(&mut spi).unwrap();

    println!("Done");
    Ok(())
}

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

fn draw_ruler(display: &mut Display1in54) {
    for col in 0..WIDTH {
        if col % 25 == 0 {
            Line::new(Point::new(col as i32, 0), Point::new(col as i32, 10))
                .into_styled(PrimitiveStyle::with_stroke(Black, 1))
                .draw(display)
                .unwrap();
            let label = col.to_string();
            draw_text(display, &label, col as i32, 12);
        }
    }
}

fn draw_text(display: &mut Display1in54, text: &str, x: i32, y: i32) {
    let _ = Text::new(text, Point::new(x, y))
        .into_styled(text_style!(
            font = Font6x8,
            text_color = Black,
            background_color = White
        ))
        .draw(display);
}
