//use once_cell::sync::Lazy;
//use regex::Regex;

// Default background and foreground colors
const DEFAULT_BG: &str = "#000000";
//const DEFAULT_FG: &str = "#ffffff";

// Use once_cell for the regex to avoid recompiling it on each call
//static HEX_REGEX: Lazy<Regex> =
//    Lazy::new(|| Regex::new(r"^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$").unwrap());

fn hex_to_rgb(hex_str: crate::palette::Color) -> [u8; 3] {
    let hex_str = hex_str.as_str();
    // Safety: Color was already parsed to contain valid chars
    let r = u8::from_str_radix(&hex_str[1..2], 16).unwrap();
    let g = u8::from_str_radix(&hex_str[3..4], 16).unwrap();
    let b = u8::from_str_radix(&hex_str[5..6], 16).unwrap();

    [r, g, b]
}

fn blend<'a>(
    fg: crate::palette::Color,
    bg: crate::palette::Color,
    alpha: f32,
) -> crate::palette::Color {
    let fg_rgb = hex_to_rgb(fg);
    let bg_rgb = hex_to_rgb(bg);

    let blend_channel = |i: usize| -> u8 {
        let ret = (alpha * fg_rgb[i] as f32) + ((1.0 - alpha) * bg_rgb[i] as f32);
        ret.min(255.0).max(0.0).round() as u8
    };

    let result = format!(
        "#{:02X}{:02X}{:02X}",
        blend_channel(0),
        blend_channel(1),
        blend_channel(2)
    );

    // Transformation from rgb coming from a hex, should be safe
    crate::palette::Color::parse(&result).unwrap()
}

pub fn darken(
    hex: crate::palette::Color,
    amount: f32,
    bg: Option<crate::palette::Color>,
) -> crate::palette::Color {
    let bg = bg.unwrap_or(crate::palette::Color::parse(DEFAULT_BG).unwrap());
    blend(hex, bg, amount.abs())
}

//#[allow(dead_code)]
//pub fn lighten<'a>(hex: Box<str>, amount: f32, fg: Option<Box<str>>) -> Result<Box<str>, Box<str>> {
//    let fg = fg.unwrap_or(DEFAULT_FG.into());
//    blend(hex, fg, amount.abs())
//}
