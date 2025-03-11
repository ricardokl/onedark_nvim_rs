use crate::{palette, OneDarkConfig, OneDarkStyle};
use nvim_oxi::api;
use std::collections::HashMap;

// Define a struct to hold all the color values
#[derive(Clone, Debug)]
pub struct ColorPalette {
    pub black: String,
    pub bg0: String,
    pub bg1: String,
    pub bg2: String,
    pub bg3: String,
    pub bg_d: String,
    pub bg_blue: String,
    pub bg_yellow: String,
    pub fg: String,
    pub purple: String,
    pub green: String,
    pub orange: String,
    pub blue: String,
    pub yellow: String,
    pub cyan: String,
    pub red: String,
    pub grey: String,
    pub light_grey: String,
    pub dark_cyan: String,
    pub dark_red: String,
    pub dark_yellow: String,
    pub dark_purple: String,
    pub diff_add: String,
    pub diff_delete: String,
    pub diff_change: String,
    pub diff_text: String,
}

// Function to get global config or return default if it doesn't exist
fn get_global_config() -> OneDarkConfig {
    match api::get_var::<OneDarkConfig>("onedark_config") {
        Ok(config) => config,
        Err(_) => OneDarkConfig::default(),
    }
}

// Convert Lua palette table to ColorPalette struct
fn convert_lua_palette_to_struct(style: OneDarkStyle) -> Result<ColorPalette, nvim_oxi::Error> {
    let lua_code = format!("return require('palette').{}", style.as_str());
    let palette_table: HashMap<String, String> = api::exec_lua(&lua_code, vec![])?;
    
    Ok(ColorPalette {
        black: palette_table.get("black").cloned().unwrap_or_default(),
        bg0: palette_table.get("bg0").cloned().unwrap_or_default(),
        bg1: palette_table.get("bg1").cloned().unwrap_or_default(),
        bg2: palette_table.get("bg2").cloned().unwrap_or_default(),
        bg3: palette_table.get("bg3").cloned().unwrap_or_default(),
        bg_d: palette_table.get("bg_d").cloned().unwrap_or_default(),
        bg_blue: palette_table.get("bg_blue").cloned().unwrap_or_default(),
        bg_yellow: palette_table.get("bg_yellow").cloned().unwrap_or_default(),
        fg: palette_table.get("fg").cloned().unwrap_or_default(),
        purple: palette_table.get("purple").cloned().unwrap_or_default(),
        green: palette_table.get("green").cloned().unwrap_or_default(),
        orange: palette_table.get("orange").cloned().unwrap_or_default(),
        blue: palette_table.get("blue").cloned().unwrap_or_default(),
        yellow: palette_table.get("yellow").cloned().unwrap_or_default(),
        cyan: palette_table.get("cyan").cloned().unwrap_or_default(),
        red: palette_table.get("red").cloned().unwrap_or_default(),
        grey: palette_table.get("grey").cloned().unwrap_or_default(),
        light_grey: palette_table.get("light_grey").cloned().unwrap_or_default(),
        dark_cyan: palette_table.get("dark_cyan").cloned().unwrap_or_default(),
        dark_red: palette_table.get("dark_red").cloned().unwrap_or_default(),
        dark_yellow: palette_table.get("dark_yellow").cloned().unwrap_or_default(),
        dark_purple: palette_table.get("dark_purple").cloned().unwrap_or_default(),
        diff_add: palette_table.get("diff_add").cloned().unwrap_or_default(),
        diff_delete: palette_table.get("diff_delete").cloned().unwrap_or_default(),
        diff_change: palette_table.get("diff_change").cloned().unwrap_or_default(),
        diff_text: palette_table.get("diff_text").cloned().unwrap_or_default(),
    })
}

// Get the base palette based on the style
fn get_base_palette(style: OneDarkStyle) -> Result<ColorPalette, nvim_oxi::Error> {
    convert_lua_palette_to_struct(style)
}

// Extend the palette with custom colors from config
pub fn select_colors() -> Result<ColorPalette, nvim_oxi::Error> {
    // Get the base palette based on the style from config
    let config = get_global_config();
    let mut palette = get_base_palette(config.style)?;

    // If there are custom colors in the config, apply them
    if let Ok(custom_colors) = api::get_var::<HashMap<String, String>>("onedark_config.colors") {
        for (key, value) in custom_colors {
            match key.as_str() {
                "black" => palette.black = value,
                "bg0" => palette.bg0 = value,
                "bg1" => palette.bg1 = value,
                "bg2" => palette.bg2 = value,
                "bg3" => palette.bg3 = value,
                "bg_d" => palette.bg_d = value,
                "bg_blue" => palette.bg_blue = value,
                "bg_yellow" => palette.bg_yellow = value,
                "fg" => palette.fg = value,
                "purple" => palette.purple = value,
                "green" => palette.green = value,
                "orange" => palette.orange = value,
                "blue" => palette.blue = value,
                "yellow" => palette.yellow = value,
                "cyan" => palette.cyan = value,
                "red" => palette.red = value,
                "grey" => palette.grey = value,
                "light_grey" => palette.light_grey = value,
                "dark_cyan" => palette.dark_cyan = value,
                "dark_red" => palette.dark_red = value,
                "dark_yellow" => palette.dark_yellow = value,
                "dark_purple" => palette.dark_purple = value,
                "diff_add" => palette.diff_add = value,
                "diff_delete" => palette.diff_delete = value,
                "diff_change" => palette.diff_change = value,
                "diff_text" => palette.diff_text = value,
                _ => {} // Ignore unknown keys
            }
        }
    }

    Ok(palette)
}

// Function to get the colors for use in other modules
pub fn get_colors() -> Result<ColorPalette, nvim_oxi::Error> {
    select_colors()
}

// Function to generate terminal colors
pub fn get_terminal_colors() -> HashMap<u8, String> {
    let mut terminal_colors = HashMap::new();
    
    if let Ok(palette) = get_colors() {
        // Standard terminal colors
        terminal_colors.insert(0, palette.black.clone());
        terminal_colors.insert(1, palette.red.clone());
        terminal_colors.insert(2, palette.green.clone());
        terminal_colors.insert(3, palette.yellow.clone());
        terminal_colors.insert(4, palette.blue.clone());
        terminal_colors.insert(5, palette.purple.clone());
        terminal_colors.insert(6, palette.cyan.clone());
        terminal_colors.insert(7, palette.fg.clone());
        
        // Bright terminal colors
        terminal_colors.insert(8, palette.grey.clone());
        terminal_colors.insert(9, palette.red.clone());
        terminal_colors.insert(10, palette.green.clone());
        terminal_colors.insert(11, palette.yellow.clone());
        terminal_colors.insert(12, palette.blue.clone());
        terminal_colors.insert(13, palette.purple.clone());
        terminal_colors.insert(14, palette.cyan.clone());
        terminal_colors.insert(15, palette.fg.clone());
    }
    
    terminal_colors
}
