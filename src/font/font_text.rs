use embedded_graphics::{
    drawable::Drawable, geometry::Point, pixelcolor::PixelColor, style::Styled,
    transform::Transform,
};

use crate::{font::FontStyle, TinySkiaDisplay};

/// A font text object.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FontText<'a> {
    /// The string.
    pub text: &'a str,

    /// The position.
    ///
    /// Defines the top-left starting pixel of the text object.
    pub position: Point,
}

impl<'a> FontText<'a> {
    /// Creates a text.
    pub const fn new(text: &'a str, position: Point) -> Self {
        Self { text, position }
    }

    /// Attaches a text style to the text object.
    pub fn into_styled<C>(self, style: FontStyle<C>) -> Styled<Self, FontStyle<C>>
    where
        C: PixelColor,
    {
        Styled::new(self, style)
    }
}

impl Transform for FontText<'_> {
    fn translate(&self, by: Point) -> Self {
        Self {
            position: self.position + by,
            ..*self
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.position += by;

        self
    }
}

impl<C> Drawable<C> for &Styled<FontText<'_>, FontStyle<C>>
where
    C: PixelColor,
{
    fn draw(self, display: &mut TinySkiaDisplay<C>) -> Result<(), String> {
        display.draw_iter(self.into_iter())
    }
}

// impl<C> Drawable<C> for &Styled<FontText<'_>, FontStyle<C>>
// where
//     C: PixelColor,
// {
//     fn draw(self, display: &mut TinySkiaDisplay<C>) -> Result<(), String> {
//         // display.draw_iter(self.into_iter())
//     }
// }
