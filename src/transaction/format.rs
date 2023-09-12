use colored::{ Color, Colorize, ColoredString };
use serde::Serializer;

pub const BITCOIN_COLOR: Color = Color::TrueColor { r: 255, g: 153, b: 0 };
pub const DARK_ORANGE: Color = Color::TrueColor { r: 255, g: 110, b: 0 };

pub struct TxFormat {}

impl TxFormat {
    pub fn paint(text: &str, color: Color, bold: bool, italic: bool) {
        let mut format: ColoredString;
        match color {
            Color::BrightBlue => format = text.bright_blue(),
            Color::Cyan => format = text.cyan(),
            Color::Green => format = text.green(),
            Color::Magenta => format = text.magenta(),
            Color::TrueColor { r, g, b } => format = text.truecolor(r, g, b),
            _ => format = format!("Not implemented that color {:?}", color).bold()
        }
        if bold { format = format.bold(); }
        if italic { format = format.italic(); }
        print!("{}", format);
    }

    pub fn hex_encoding<S, T>(t: T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        s.serialize_str(&hex::encode(t))
    }

    pub fn hex_formatting<S, T>(d: T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: std::fmt::LowerHex,
    {
        s.serialize_str(&format!("{:#x}", d))
    }
}