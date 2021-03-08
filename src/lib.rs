use std::marker::PhantomData;

use tiny_skia::*;

use embedded_graphics_core::{
    draw_target::*,
    geometry::{Point, Size},
    pixelcolor::*,
    prelude::Dimensions,
    prelude::*,
    primitives::Rectangle,
    Pixel,
};

pub mod font;

/// This display is based on raqote's `DrawTarget` and is used as draw target for the embedded graphics crate.
///
/// # Example
///
/// ```rust
/// use tiny_skia_display::*;
/// use embedded_graphics::{
///     pixelcolor::Rgb888,
///     prelude::*,
///     primitives::rectangle::Rectangle,
///     style::PrimitiveStyleBuilder,
///     };
///
/// let mut display = TinySkiaDisplay::new(160, 128).unwrap();
///
/// let style = PrimitiveStyleBuilder::new().fill_color(Rgb888::BLACK).build();
/// let black_backdrop = Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
/// black_backdrop.draw(&mut display).unwrap();
/// ```
pub struct TinySkiaDisplay<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    pix_map: Pixmap,
    size: Size,
    _pixel_color: PhantomData<C>,
}

impl<C> DrawTarget for TinySkiaDisplay<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw> + Into<Rgb888>,
{
    type Color = C;

    type Error = String;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        // let now = SystemTime::now();

        for Pixel(p, color) in pixels.into_iter() {
            self.draw_pixel(p, color);
        }

        // println!("draw_iter: {:?}", now.elapsed());

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // let now = SystemTime::now();
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&Rectangle::new(Point::zero(), self.size));

        // Do not draw rectangle if the intersection size if zero.
        // The size is checked by using `Rectangle::bottom_right`, which returns `None`
        // if the size is zero.
        let bottom_right = if let Some(bottom_right) = area.bottom_right() {
            bottom_right
        } else {
            return Ok(());
        };

        if let Some(rect) = Rect::from_xywh(
            area.top_left.x as f32,
            area.top_left.y as f32,
            bottom_right.x as f32 - area.top_left.x as f32,
            bottom_right.y as f32 - area.top_left.y as f32,
        ) {
            self.pix_map.fill_rect(
                rect,
                &convert_color_to_paint(color),
                Transform::identity(),
                None,
            );
        } else {
            return Err(String::from("Cannot create tiny-skia rect"));
        }

        // println!("fill_solid: {:?}", now.elapsed());

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&Rectangle::new(Point::zero(), self.size));

        if area.size != Size::zero() {
            for p in area
                .points()
                .zip(colors)
                .filter(|(pos, _color)| area.contains(*pos))
                .map(|(pos, color)| Pixel(pos, color))
            {
                self.draw_pixel(p.0, p.1);
            }
        }

        Ok(())
    }
}

impl<C> Dimensions for TinySkiaDisplay<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::default(), self.size)
    }
}

impl<C> TinySkiaDisplay<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw> + Into<Rgb888>,
{
    /// Creates a new display display with the given size.
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(TinySkiaDisplay {
            pix_map: Pixmap::new(width, height).ok_or("Cannot create tiny-skia Pixmap")?,
            size: Size::new(width, height),
            // fonts: HashMap::new(),
            _pixel_color: PhantomData::default(),
        })
    }

    /// Returns a reference to the underlying pixel data.
    pub fn data(&self) -> &[u8] {
        self.pix_map.data()
    }

    pub fn draw_pixel(&mut self, position: Point, color: C) {
        let (r, g, b, a) = rgba(color);

        if position.x >= 0
            && position.y >= 0
            && position.x < self.size.width as i32
            && position.y < self.size.height as i32
        {
            let index = (position.y as usize * self.size.width as usize + position.x as usize) * 4;

            self.pix_map.data_mut()[index] = r;
            self.pix_map.data_mut()[index + 1] = g;
            self.pix_map.data_mut()[index + 2] = b;
            self.pix_map.data_mut()[index + 3] = a;
        }
    }

    /// Pushes the frame buffer to the given surface and clears the frame buffer.
    pub fn flip(&mut self, surface: &mut [u8]) {
        surface.copy_from_slice(self.pix_map.data_mut());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn rgba<C: PixelColor + Into<Rgb888>>(color: C) -> (u8, u8, u8, u8) {
    let color: Rgb888 = color.into();

    (color.b(), color.g(), color.r(), 255)
}

#[cfg(target_arch = "wasm32")]
fn rgba<C: PixelColor + Into<Rgb888>>(color: C) -> (u8, u8, u8, u8) {
    let color: Rgb888 = color.into();

    (color.r(), color.g(), color.b(), 255)
}

// converts the given embedded-graphics pixel color to a tiny-skia paint.
fn convert_color_to_paint<'a, C: PixelColor + Into<Rgb888>>(color: C) -> Paint<'a> {
    let (r, g, b, a) = rgba(color);

    let mut paint = Paint {
        anti_alias: true,
        ..Default::default()
    };
    paint.set_color_rgba8(r, g, b, a);
    paint
}
