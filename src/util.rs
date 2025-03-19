use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Write;

// Default background and foreground colors
pub const DEFAULT_BG: &str = "#000000";
pub const DEFAULT_FG: &str = "#ffffff";

// Use once_cell for the regex to avoid recompiling it on each call
static HEX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$").unwrap());

/// Converts a hex color string to RGB components
///
/// # Arguments
/// * `hex_str` - A string in the format "#RRGGBB"
///
/// # Returns
/// * `Result<[u8; 3], String>` - RGB values as [r, g, b] or an error message
pub fn hex_to_rgb(hex_str: &str) -> Result<[u8; 3], String> {
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
        None => Err(format!("Invalid hex color format: {}", hex_str)),
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
/// * `Result<String, String>` - Blended color as a hex string or an error message
pub fn blend(fg: &str, bg: &str, alpha: f32) -> Result<String, String> {
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

    Ok(result)
}

/// Darkens a color by blending it with black or a specified background color
///
/// # Arguments
/// * `hex` - Color to darken as a hex string
/// * `amount` - Amount to darken (0.0 to 1.0)
/// * `bg` - Optional background color (defaults to black)
///
/// # Returns
/// * `Result<String, String>` - Darkened color as a hex string or an error message
pub fn darken(hex: &str, amount: f32, bg: Option<&str>) -> Result<String, String> {
    let bg = bg.unwrap_or(DEFAULT_BG);
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
/// * `Result<String, String>` - Lightened color as a hex string or an error message
#[allow(dead_code)]
pub fn lighten(hex: &str, amount: f32, fg: Option<&str>) -> Result<String, String> {
    let fg = fg.unwrap_or(DEFAULT_FG);
    blend(hex, fg, amount.abs())
}
