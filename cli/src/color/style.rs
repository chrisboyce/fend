use super::base::Base;
use std::fmt;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Color {
    foreground: Option<Base>,
    underline: bool,
    bold: bool,
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ColorVisitor;

        impl<'de> serde::de::Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a color, with properties `foreground`, `underline` and `bold`")
            }

            fn visit_map<V: serde::de::MapAccess<'de>>(
                self,
                mut map: V,
            ) -> Result<Color, V::Error> {
                let mut result = Color::default();
                let mut seen_foreground = false;
                let mut seen_underline = false;
                let mut seen_bold = false;
                while let Some(key) = map.next_key()? {
                    match key {
                        "foreground" => {
                            if seen_foreground {
                                return Err(serde::de::Error::duplicate_field("foreground"));
                            }
                            result.foreground = Some(map.next_value()?);
                            seen_foreground = true;
                        }
                        "underline" => {
                            if seen_underline {
                                return Err(serde::de::Error::duplicate_field("underline"));
                            }
                            result.underline = map.next_value()?;
                            seen_underline = true;
                        }
                        "bold" => {
                            if seen_bold {
                                return Err(serde::de::Error::duplicate_field("bold"));
                            }
                            result.bold = map.next_value()?;
                            seen_bold = true;
                        }
                        unknown_key => {
                            return Err(serde::de::Error::unknown_field(unknown_key, FIELDS));
                        }
                    }
                }
                Ok(result)
            }
        }

        const FIELDS: &[&str] = &["foreground", "underline", "bold"];
        deserializer.deserialize_struct("Color", FIELDS, ColorVisitor)
    }
}

impl Color {
    pub fn new(foreground: Base) -> Self {
        Self {
            foreground: Some(foreground),
            underline: false,
            bold: false,
        }
    }

    pub fn bold(foreground: Base) -> Self {
        Self {
            bold: true,
            ..Self::new(foreground)
        }
    }

    pub fn to_ansi(&self) -> ansi_term::Style {
        let mut style = ansi_term::Style::default();
        if let Some(foreground) = self.foreground {
            style = style.fg(foreground.as_ansi());
        }
        if self.underline {
            style = style.underline();
        }
        if self.bold {
            style = style.bold();
        }
        style
    }
}