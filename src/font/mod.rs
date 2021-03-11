use std::marker::PhantomData;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::Rectangle,
    text::{CharacterStyle, DecorationColor, TextMetrics, TextRenderer, VerticalAlignment},
};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

mod glyph_tracer;

pub use self::glyph_tracer::*;

pub struct FontTextStyle<C: PixelColor> {
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Underline color.
    pub underline_color: DecorationColor<C>,

    /// Strikethrough color.
    pub strikethrough_color: DecorationColor<C>,

    /// Font size.
    pub font_size: u32,

    /// Font.
    font: Font<C>,
}

impl<C: PixelColor> FontTextStyle<C> {
    pub fn new(font: Font<C>, text_color: C, font_size: u32) -> Self {
        FontTextStyleBuilder::new(font)
            .text_color(text_color)
            .font_size(font_size)
            .build()
    }

    /// Resolves a decoration color.
    fn resolve_decoration_color(&self, color: DecorationColor<C>) -> Option<C> {
        match color {
            DecorationColor::None => None,
            DecorationColor::TextColor => self.text_color,
            DecorationColor::Custom(c) => Some(c),
        }
    }

    fn draw_background<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if width == 0 {
            return Ok(());
        }

        if let Some(background_color) = self.background_color {
            target.fill_solid(
                &Rectangle::new(position, Size::new(width, self.font_size)),
                background_color,
            )?;
        }

        Ok(())
    }

    fn draw_strikethrough<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(strikethrough_color) = self.resolve_decoration_color(self.strikethrough_color) {
            let top_left = position + Point::new(0, 0);
            let size = Size::new(width, self.font_size);

            target.fill_solid(&Rectangle::new(top_left, size), strikethrough_color)?;
        }

        Ok(())
    }

    fn draw_underline<D>(&self, width: u32, position: Point, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(underline_color) = self.resolve_decoration_color(self.underline_color) {
            let top_left = position + Point::new(0, 0);
            let size = Size::new(width, self.font_size);

            target.fill_solid(&Rectangle::new(top_left, size), underline_color)?;
        }

        Ok(())
    }
}

impl<C: PixelColor> CharacterStyle for FontTextStyle<C> {
    type Color = C;

    fn set_text_color(&mut self, text_color: Option<Self::Color>) {
        self.text_color = text_color;
    }

    fn set_background_color(&mut self, background_color: Option<Self::Color>) {
        self.background_color = background_color;
    }

    fn set_underline_color(&mut self, underline_color: DecorationColor<Self::Color>) {
        self.underline_color = underline_color;
    }

    fn set_strikethrough_color(&mut self, strikethrough_color: DecorationColor<Self::Color>) {
        self.strikethrough_color = strikethrough_color;
    }
}

pub struct FontTextStyleBuilder<C: PixelColor> {
    style: FontTextStyle<C>,
}

impl<C: PixelColor> FontTextStyleBuilder<C> {
    /// Creates a new text style builder.
    pub fn new(font: Font<C>) -> Self {
        Self {
            style: FontTextStyle {
                font,
                background_color: None,
                font_size: 12,
                text_color: None,
                underline_color: DecorationColor::None,
                strikethrough_color: DecorationColor::None,
            },
        }
    }

    pub fn font_size(mut self, font_size: u32) -> Self {
        self.style.font_size = font_size;
        self
    }

    /// Enables underline using the text color.
    pub fn underline(mut self) -> Self {
        self.style.underline_color = DecorationColor::TextColor;

        self
    }

    /// Enables strikethrough using the text color.
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough_color = DecorationColor::TextColor;

        self
    }

    /// Sets the text color.
    pub fn text_color(mut self, text_color: C) -> Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the background color.
    pub fn background_color(mut self, background_color: C) -> Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Enables underline with a custom color.
    pub fn underline_with_color(mut self, underline_color: C) -> Self {
        self.style.underline_color = DecorationColor::Custom(underline_color);

        self
    }

    /// Enables strikethrough with a custom color.
    pub fn strikethrough_with_color(mut self, strikethrough_color: C) -> Self {
        self.style.strikethrough_color = DecorationColor::Custom(strikethrough_color);

        self
    }

    /// Builds the text style.
    ///
    /// This method can only be called after a font was set by using the [`font`] method. All other
    /// settings are optional and they will be set to their default value if they are missing.
    ///
    /// [`font`]: #method.font
    pub fn build(self) -> FontTextStyle<C> {
        self.style
    }
}

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

impl<C: PixelColor> TextRenderer for FontTextStyle<C> {
    type Color = C;

    fn draw_string<D>(&self, text: &str, position: Point, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut p = position;

        let scale = rusttype::Scale::uniform(self.font_size as f32);

        let v_metrics = self.font.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.font.inner.layout(text, scale, offset).collect();

        let mut glyph_tracer = GlyphTracer {
            path_builder: PathBuilder::new(),
            position: rusttype::point(0.0, 0.0),
        };

        let mut width = 0;

        for g in glyphs.iter() {
            let mut gpos = match g.pixel_bounding_box() {
                Some(bbox) => rusttype::point(bbox.min.x as f32, bbox.min.y as f32),
                None => {
                    continue;
                }
            };
            gpos.x += position.x as f32;
            gpos.y += position.y as f32;
            glyph_tracer.position = gpos;
            g.build_outline(&mut glyph_tracer);

            p = Point::new(gpos.x as i32, p.y);
            width +=
                (g.position().x as f32 + g.unpositioned().h_metrics().advance_width).ceil() as u32;
        }

        self.draw_background(width, position, target)?;
        self.draw_strikethrough(width, position, target)?;
        self.draw_underline(width, position, target)?;

        Ok(p)
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
        let scale = rusttype::Scale::uniform(self.font_size as f32);
        let v_metrics = self.font.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.font.inner.layout(text, scale, offset).collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as f64;

        let size = Size::new(width as u32, self.font_size);

        TextMetrics {
            bounding_box: Rectangle::new(position, size),
            next_position: position + size.x_axis(),
        }
    }

    fn vertical_offset(&self, position: Point, _vertical_alignment: VerticalAlignment) -> Point {
        position
    }

    fn line_height(&self) -> u32 {
        self.font_size
    }
}
