use embedded_graphics::pixelcolor::PixelColor;

/// Style properties for font text.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub struct FontStyle<C>
where
    C: PixelColor,
{
    /// Font family.
    pub font_family: String,

    /// Text color.
    pub text_color: Option<C>,

    /// Text pixel size.
    pub pixel_size: u32,
}

impl<C> FontStyle<C>
where
    C: PixelColor,
{
    /// Creates a font style.
    pub fn new(font_family: impl Into<String>, text_color: C, pixel_size: u32) -> Self {
        Self {
            font_family: font_family.into(),
            text_color: Some(text_color),
            pixel_size,
        }
    }
}

/// Font text style builder.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FontStyleBuilder<C>
where
    C: PixelColor,
{
    style: FontStyle<C>,
}

impl<C> FontStyleBuilder<C>
where
    C: PixelColor,
{
    /// Creates a new text style builder with a given font.
    pub fn new(font_family: impl Into<String>) -> Self {
        Self {
            style: FontStyle {
                font_family: font_family.into(),
                text_color: None,
                pixel_size: 0,
            },
        }
    }

    /// Sets the text color.
    pub fn text_color(mut self, text_color: C) -> Self {
        self.style.text_color = Some(text_color);
        self
    }

    /// Sets the pixel size.
    pub fn pixel_size(mut self, pixel_size: u32) -> Self {
        self.style.pixel_size = pixel_size;
        self
    }

    /// Builds the text style.
    pub fn build(self) -> FontStyle<C> {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::BinaryColor;

    #[test]
    fn builder_default() {
        assert_eq!(
            FontStyleBuilder::<BinaryColor>::new("my_font").build(),
            FontStyle {
                font_family: String::from("my_font"),
                text_color: None,
                pixel_size: 0
            }
        );
    }

    #[test]
    fn builder_text_color() {
        assert_eq!(
            FontStyleBuilder::new("my_font")
                .text_color(BinaryColor::On)
                .build(),
            FontStyle::new("my_font", BinaryColor::On, 0)
        );
    }

    #[test]
    fn builder_pixel_size() {
        assert_eq!(
            FontStyleBuilder::<BinaryColor>::new("my_font")
                .pixel_size(12)
                .build(),
            {
                let mut style = FontStyleBuilder::new("my_font").build();

                style.text_color = None;
                style.pixel_size = 12;

                style
            }
        );
    }
}
