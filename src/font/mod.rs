use std::marker::PhantomData;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    prelude::*,
    text::{TextMetrics, TextRenderer, VerticalAlignment},
};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

mod glyph_tracer;

pub use self::glyph_tracer::*;

#[derive(Debug, Clone)]
pub struct Font<C: PixelColor> {
    inner: rusttype::Font<'static>,
    pixel_size: u32,
    _c: PhantomData<C>,
}

impl<C: PixelColor> Font<C> {
    pub fn from_bytes(bytes: &'static [u8], pixel_size: u32) -> Result<Self, &'static str> {
        rusttype::Font::try_from_bytes(bytes)
            .map(|font| Font {
                inner: font,
                pixel_size,
                _c: PhantomData::default(),
            })
            .ok_or("Could not load font from bytes")
    }

    pub fn measure_text(&self, text: &str, size: f64) -> (f64, f64) {
        let scale = rusttype::Scale::uniform(size as f32);
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let pixel_height = size.ceil();

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout(text, scale, offset).collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as f64;

        (width, pixel_height)
    }

    pub fn render_text(
        &self,
        text: &str,
        pix_map: &mut Pixmap,
        font_size: f64,
        paint: &Paint,
        position: (f64, f64),
    ) {
        let scale = rusttype::Scale::uniform(font_size as f32);

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout(text, scale, offset).collect();

        let mut glyph_tracer = GlyphTracer {
            path_builder: PathBuilder::new(),
            position: rusttype::point(0.0, 0.0),
        };
        for g in glyphs.iter() {
            let mut gpos = match g.pixel_bounding_box() {
                Some(bbox) => rusttype::point(bbox.min.x as f32, bbox.min.y as f32),
                None => {
                    continue;
                }
            };
            gpos.x += position.0 as f32;
            gpos.y += position.1 as f32;
            glyph_tracer.position = gpos;
            g.build_outline(&mut glyph_tracer);
        }
        if let Some(path) = glyph_tracer.path_builder.finish() {
            pix_map.fill_path(&path, paint, FillRule::Winding, Transform::identity(), None);
        }
    }
}

impl<C: PixelColor> TextRenderer for Font<C> {
    type Color = C;

    fn draw_string<D>(&self, text: &str, position: Point, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        todo!()
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        todo!()
    }

    fn measure_string(&self, text: &str, position: Point) -> TextMetrics {
        // let scale = rusttype::Scale::uniform(size as f32);
        // let v_metrics = self.inner.v_metrics(scale);
        // let offset = rusttype::point(0.0, v_metrics.ascent);

        // let pixel_height = size.ceil();

        // // Glyphs to draw for "RustType". Feel free to try other strings.
        // let glyphs: Vec<rusttype::PositionedGlyph> =
        //     self.inner.layout(text, scale, offset).collect();

        // let width = glyphs
        //     .iter()
        //     .rev()
        //     .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        //     .next()
        //     .unwrap_or(0.0)
        //     .ceil() as f64;
        todo!()
    }

    fn vertical_offset(&self, position: Point, vertical_alignment: VerticalAlignment) -> Point {
        todo!()
    }

    fn line_height(&self) -> u32 {
        self.pixel_size
    }
}
