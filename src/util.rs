use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Write;

// Default background and foreground colors
const DEFAULT_BG: &str = "#000000";
const DEFAULT_FG: &str = "#ffffff";

// Use once_cell for the regex to avoid recompiling it on each call
static HEX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$").unwrap());

/// Converts a hex color string to RGB components
///
/// # Arguments
/// * `hex_str` - A string in the format "#RRGGBB"
///
/// # Returns
/// * `Result<[u8; 3], Box<str>>` - RGB values as [r, g, b] or an error message
fn hex_to_rgb(hex_str: Box<str>) -> Result<[u8; 3], Box<str>> {
    let hex_str = hex_str.to_lowercase();

    match HEX_REGEX.captures(&hex_str) {
        Some(caps) => {
            let r = u8::from_str_radix(&caps[1], 16)
                .map_err(|_| format!("Invalid red component in hex: {}", hex_str))?;
            let g = u8::from_str_radix(&caps[2], 16)
                .map_err(|_| format!("Invalid green component in hex: {}", hex_str))?;
            let b = u8::from_str_radix(&caps[3], 16)
                .map_err(|_| format!("Invalid blue component in hex: {}", hex_str))?;

            Ok([r, g, b])
        }
        None => Err(format!("Invalid hex color format: {}", hex_str).into()),
    }
}

/// Blends two colors together with the given alpha value
///
/// # Arguments
/// * `fg` - Foreground color as a hex string
/// * `bg` - Background color as a hex string
/// * `alpha` - Blend amount between 0.0 and 1.0 (0.0 = bg, 1.0 = fg)
///
/// # Returns
/// * `Result<Box<str>, Box<str>>` - Blended color as a hex string or an error message
fn blend<'a>(fg: Box<str>, bg: Box<str>, alpha: f32) -> Result<Box<str>, Box<str>> {
    let fg_rgb = hex_to_rgb(fg)?;
    let bg_rgb = hex_to_rgb(bg)?;

    let blend_channel = |i: usize| -> u8 {
        let ret = (alpha * fg_rgb[i] as f32) + ((1.0 - alpha) * bg_rgb[i] as f32);
        ret.min(255.0).max(0.0).round() as u8
    };

    let mut result = String::with_capacity(7);
    write!(
        &mut result,
        "#{:02X}{:02X}{:02X}",
        blend_channel(0),
        blend_channel(1),
        blend_channel(2)
    )
    .map_err(|e| format!("Failed to format hex string: {}", e))?;

    Ok(result.into())
}

/// Darkens a color by blending it with black or a specified background color
///
/// # Arguments
/// * `hex` - Color to darken as a hex string
/// * `amount` - Amount to darken (0.0 to 1.0)
/// * `bg` - Optional background color (defaults to black)
///
/// # Returns
/// * `Result<Box<str>, Box<str>>` - Darkened color as a hex string or an error message
pub fn darken(hex: Box<str>, amount: f32, bg: Option<Box<str>>) -> Result<Box<str>, Box<str>> {
    let bg = bg.unwrap_or(DEFAULT_BG.into());
    blend(hex, bg, amount.abs())
}

/// Lightens a color by blending it with white or a specified foreground color
///
/// # Arguments
/// * `hex` - Color to lighten as a hex string
/// * `amount` - Amount to lighten (0.0 to 1.0)
/// * `fg` - Optional foreground color (defaults to white)
///
/// # Returns
/// * `Result<Box<str>, Box<str>>` - Lightened color as a hex string or an error message
#[allow(dead_code)]
pub fn lighten<'a>(hex: Box<str>, amount: f32, fg: Option<Box<str>>) -> Result<Box<str>, Box<str>> {
    let fg = fg.unwrap_or(DEFAULT_FG.into());
    blend(hex, fg, amount.abs())
}
