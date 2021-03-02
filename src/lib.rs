use std::{collections::HashMap, marker::PhantomData};

use tiny_skia::*;

use embedded_graphics::{
    drawable::Pixel,
    geometry::{Dimensions, Size},
    image::{Image, ImageDimensions},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    prelude::IntoPixelIter,
    primitives,
    style::{PrimitiveStyle, Styled},
    DrawTarget,
};

// #[cfg(feature = "rtype")]
// mod font;

// #[cfg(feature = "rtype")]
// use font::*;

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
    // fonts: HashMap<String, Font>,
    _pixel_color: PhantomData<C>,
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

    fn fill(&mut self, path: &tiny_skia::Path, color: C) {
        self.pix_map.fill_path(
            path,
            &convert_color_to_paint(color),
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    fn stroke(&mut self, path: &tiny_skia::Path, color: C, stroke_width: u32) {
        let mut stroke = Stroke::default();
        stroke.width = stroke_width as f32;

        self.pix_map.stroke_path(
            path,
            &convert_color_to_paint(color),
            &stroke,
            Transform::identity(),
            None,
        );
    }

    /// Returns a reference to the underlying pixel data.
    pub fn data(&self) -> &[u8] {
        self.pix_map.data()
    }

    /// Pushes the frame buffer to the given surface and clears the frame buffer.
    pub fn flip(&mut self, surface: &mut [u8]) {
        surface.copy_from_slice(self.pix_map.data_mut());
    }

    // pub fn register_font(&mut self, font_family: &str, font: Font) {
    //     self.fonts.insert(font_family.into(), font);
    // }

    // pub fn draw_text(&mut self, text: &str, font_family: &str) {
    //     if let Some(font) = self.fonts.get(font_family) {
    //         // font.draw_text(text, pix_map, font_size, paint, position)
    //     }
    // }
}

impl<C> DrawTarget<C> for TinySkiaDisplay<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw> + Into<Rgb888>,
{
    type Error = String;

    fn draw_pixel(&mut self, pixel: Pixel<C>) -> Result<(), Self::Error> {
        self.pix_map.fill_rect(
            Rect::from_xywh(pixel.0.x as f32, pixel.0.y as f32, 1., 1.)
                .ok_or("Cannot crate tiny skia Rect.")?,
            &convert_color_to_paint(pixel.1),
            Transform::identity(),
            None,
        );

        Ok(())
    }

    fn draw_line(
        &mut self,
        item: &Styled<primitives::Line, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        let path = convert_line_path(item.primitive)?;

        if let Some(fill_color) = item.style.fill_color {
            self.fill(&path, fill_color);
        }

        if let Some(stroke_color) = item.style.stroke_color {
            self.stroke(&path, stroke_color, item.style.stroke_width);
        }

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        item: &Styled<primitives::Rectangle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        let style = item.style;
        let rect = convert_rect(item.primitive).ok_or("Cannot create tiny-skia rect.")?;
        let rect_path = convert_rect_path(item.primitive)?;

        if let Some(fill_color) = style.fill_color {
            self.pix_map.fill_rect(
                rect,
                &convert_color_to_paint(fill_color),
                Transform::identity(),
                None,
            );
        }
        if let Some(stroke_color) = style.stroke_color {
            self.stroke(&rect_path, stroke_color, style.stroke_width)
        }

        Ok(())
    }

    fn draw_circle(
        &mut self,
        item: &Styled<primitives::Circle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        let style = item.style;
        let circle = convert_circle_path(item.primitive)?;

        if let Some(fill_color) = style.fill_color {
            self.fill(&circle, fill_color);
        }
        if let Some(stroke_color) = style.stroke_color {
            self.stroke(&circle, stroke_color, item.style.stroke_width);
        }

        Ok(())
    }

    fn draw_image<'a, 'b, I>(&mut self, item: &'a Image<'b, I, C>) -> Result<(), Self::Error>
    where
        &'b I: IntoPixelIter<C>,
        I: ImageDimensions,
    {
        let mut pixels = vec![];

        for pixel in item {
            let (r, g, b, a) = rgba(pixel.1);
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }

        self.pix_map.draw_pixmap(
            item.top_left().x as i32,
            item.top_left().y as i32,
            PixmapRef::from_bytes(pixels.as_slice(), item.size().width, item.size().height)
                .ok_or("Cannot create tiny-skia pixmap.")?,
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
        Ok(())
    }

    fn size(&self) -> Size {
        self.size
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

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(r, g, b, a);
    paint
}

// converts a embedded-graphics rect to a tiny-skia rect.
fn convert_rect(rect: embedded_graphics::primitives::Rectangle) -> Option<Rect> {
    let width = (rect.bottom_right.x - rect.top_left.x) as f32;
    let height = (rect.bottom_right.y - rect.top_left.y) as f32;

    Rect::from_xywh(
        rect.top_left.x as f32,
        rect.top_left.y as f32,
        width,
        height,
    )
}

// converts a embedded-graphics circle to a tuple of circle values.
fn convert_circle(circle: embedded_graphics::primitives::Circle) -> (f32, f32, f32) {
    (
        circle.center.x as f32,
        circle.center.y as f32,
        circle.radius as f32,
    )
}

// converts a embedded-graphics rect to a tiny-skia rect path.
fn convert_rect_path(rect: embedded_graphics::primitives::Rectangle) -> Result<Path, String> {
    Ok(PathBuilder::from_rect(
        convert_rect(rect).ok_or("Cannot create tiny-skia rect")?,
    ))
}

// converts a embedded-graphics circle to a tiny-skia circle path.
fn convert_circle_path(circle: embedded_graphics::primitives::Circle) -> Result<Path, String> {
    let circle = convert_circle(circle);
    Ok(PathBuilder::from_circle(circle.0, circle.1, circle.2)
        .ok_or("Cannot create tiny-skia circle")?)
}

// converts a embedded-graphics line to a tiny-skia line path.
fn convert_line_path(line: embedded_graphics::primitives::Line) -> Result<Path, String> {
    let mut builder = PathBuilder::new();
    builder.move_to(line.start.x as f32, line.start.y as f32);
    builder.line_to(line.end.x as f32, line.end.y as f32);
    Ok(builder
        .finish()
        .ok_or("Cannot create tiny-skia path from line.")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_rect() {
        assert_eq!(
            Rect::from_xywh(1., 2., 3., 4.),
            convert_rect(embedded_graphics::primitives::Rectangle::new(
                embedded_graphics::geometry::Point::new(1, 2),
                embedded_graphics::geometry::Point::new(4, 6)
            ))
        );
    }

    #[test]
    fn test_convert_circle() {
        assert_eq!(
            (1., 2., 3.),
            convert_circle(embedded_graphics::primitives::Circle::new(
                embedded_graphics::geometry::Point::new(1, 2),
                3
            ))
        );
    }
}
