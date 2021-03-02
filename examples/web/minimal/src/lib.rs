use wasm_bindgen::prelude::*;

use embedded_graphics::{
    fonts::{Font8x16, Text},
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::rectangle::Rectangle,
    style::{PrimitiveStyleBuilder, TextStyle},
};

use tiny_skia_display::*;

use orbclient::*;

use std::panic;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let width = 160;
    let height = 128;

    let (screen_width, screen_height) = orbclient::get_display_size().unwrap();

    let mut display = TinySkiaDisplay::new(width, height).unwrap();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
    black_backdrop.draw(&mut display).unwrap();

    // draw ferris
    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../../../../assets/ferris.raw"), 86, 64);
    let image: Image<_, Rgb565> = Image::new(&image_raw, Point::new(34, 8));
    image.draw(&mut display).unwrap();

    Text::new("tiny-skia-display!", Point::new(8, 95))
        .into_styled(TextStyle::new(Font8x16, Rgb565::WHITE))
        .draw(&mut display)
        .unwrap();

    let mut window = Window::new(
        ((screen_width - width) / 2) as i32,
        ((screen_height - height) / 2) as i32,
        width,
        height,
        "minimal",
    )
    .unwrap();

    let len = window.data().len() * std::mem::size_of::<Color>();
    let color_data =
        unsafe { std::slice::from_raw_parts_mut(window.data_mut().as_mut_ptr() as *mut u8, len) };

    display.flip(color_data);

    window.sync();

    animation_loop(move || {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => return false,
                EventOption::Mouse(evt) => println!(
                    "At position {:?} pixel color is : {:?}",
                    (evt.x, evt.y),
                    window.getpixel(evt.x, evt.y)
                ),
                event_option => println!("{:?}", event_option),
            }
        }
        true
    });
}
