use crate::OneDarkStyle;
use crate::GLOBAL_CONFIG;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub black: Color,
    pub bg0: Color,
    pub bg1: Color,
    pub bg2: Color,
    pub bg3: Color,
    pub bg_d: Color,
    pub bg_blue: Color,
    pub bg_yellow: Color,
    pub fg: Color,
    pub purple: Color,
    pub green: Color,
    pub orange: Color,
    pub blue: Color,
    pub yellow: Color,
    pub cyan: Color,
    pub red: Color,
    pub grey: Color,
    pub light_grey: Color,
    pub dark_cyan: Color,
    pub dark_red: Color,
    pub dark_yellow: Color,
    pub dark_purple: Color,
    pub diff_add: Color,
    pub diff_delete: Color,
    pub diff_change: Color,
    pub diff_text: Color,
}

#[derive(Debug, Clone)]
// Zero cost abstraction for ascii chars
pub struct Color(pub [u8; 7]);

impl Color {
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err("Must be # followed by 6 hex digits");
        }
        let hex = s.as_bytes();
        if !hex.iter().all(u8::is_ascii_hexdigit) {
            return Err("Invalid hex characters");
        }
        let mut arr = [0; 7];
        arr.copy_from_slice(hex);
        Ok(Color(arr))
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: We validate during parsing that these are ASCII hex digits
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Color::parse(s).map_err(|e| e.to_string())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ConfigColorPalette {
    pub black: Option<Color>,
    pub bg0: Option<Color>,
    pub bg1: Option<Color>,
    pub bg2: Option<Color>,
    pub bg3: Option<Color>,
    pub bg_d: Option<Color>,
    pub bg_blue: Option<Color>,
    pub bg_yellow: Option<Color>,
    pub fg: Option<Color>,
    pub purple: Option<Color>,
    pub green: Option<Color>,
    pub orange: Option<Color>,
    pub blue: Option<Color>,
    pub yellow: Option<Color>,
    pub cyan: Option<Color>,
    pub red: Option<Color>,
    pub grey: Option<Color>,
    pub light_grey: Option<Color>,
    pub dark_cyan: Option<Color>,
    pub dark_red: Option<Color>,
    pub dark_yellow: Option<Color>,
    pub dark_purple: Option<Color>,
    pub diff_add: Option<Color>,
    pub diff_delete: Option<Color>,
    pub diff_change: Option<Color>,
    pub diff_text: Option<Color>,
}

pub fn get_palette(style: OneDarkStyle) -> ColorPalette {
    match style {
        OneDarkStyle::Dark => ColorPalette {
            black: "#181a1f".try_into().unwrap(),
            bg0: "#282c34".try_into().unwrap(),
            bg1: "#31353f".try_into().unwrap(),
            bg2: "#393f4a".try_into().unwrap(),
            bg3: "#3b3f4c".try_into().unwrap(),
            bg_d: "#21252b".try_into().unwrap(),
            bg_blue: "#73b8f1".try_into().unwrap(),
            bg_yellow: "#ebd09c".try_into().unwrap(),
            fg: "#abb2bf".try_into().unwrap(),
            purple: "#c678dd".try_into().unwrap(),
            green: "#98c379".try_into().unwrap(),
            orange: "#d19a66".try_into().unwrap(),
            blue: "#61afef".try_into().unwrap(),
            yellow: "#e5c07b".try_into().unwrap(),
            cyan: "#56b6c2".try_into().unwrap(),
            red: "#e86671".try_into().unwrap(),
            grey: "#5c6370".try_into().unwrap(),
            light_grey: "#848b98".try_into().unwrap(),
            dark_cyan: "#2b6f77".try_into().unwrap(),
            dark_red: "#993939".try_into().unwrap(),
            dark_yellow: "#93691d".try_into().unwrap(),
            dark_purple: "#8a3fa0".try_into().unwrap(),
            diff_add: "#31392b".try_into().unwrap(),
            diff_delete: "#382b2c".try_into().unwrap(),
            diff_change: "#1c3448".try_into().unwrap(),
            diff_text: "#2c5372".try_into().unwrap(),
        },
        OneDarkStyle::Darker => ColorPalette {
            black: "#0e1013".try_into().unwrap(),
            bg0: "#1f2329".try_into().unwrap(),
            bg1: "#282c34".try_into().unwrap(),
            bg2: "#30363f".try_into().unwrap(),
            bg3: "#323641".try_into().unwrap(),
            bg_d: "#181b20".try_into().unwrap(),
            bg_blue: "#61afef".try_into().unwrap(),
            bg_yellow: "#e8c88c".try_into().unwrap(),
            fg: "#a0a8b7".try_into().unwrap(),
            purple: "#bf68d9".try_into().unwrap(),
            green: "#8ebd6b".try_into().unwrap(),
            orange: "#cc9057".try_into().unwrap(),
            blue: "#4fa6ed".try_into().unwrap(),
            yellow: "#e2b86b".try_into().unwrap(),
            cyan: "#48b0bd".try_into().unwrap(),
            red: "#e55561".try_into().unwrap(),
            grey: "#535965".try_into().unwrap(),
            light_grey: "#7a818e".try_into().unwrap(),
            dark_cyan: "#266269".try_into().unwrap(),
            dark_red: "#8b3434".try_into().unwrap(),
            dark_yellow: "#835d1a".try_into().unwrap(),
            dark_purple: "#7e3992".try_into().unwrap(),
            diff_add: "#272e23".try_into().unwrap(),
            diff_delete: "#2d2223".try_into().unwrap(),
            diff_change: "#172a3a".try_into().unwrap(),
            diff_text: "#274964".try_into().unwrap(),
        },
        OneDarkStyle::Cool => ColorPalette {
            black: "#151820".try_into().unwrap(),
            bg0: "#242b38".try_into().unwrap(),
            bg1: "#2d3343".try_into().unwrap(),
            bg2: "#343e4f".try_into().unwrap(),
            bg3: "#363c51".try_into().unwrap(),
            bg_d: "#1e242e".try_into().unwrap(),
            bg_blue: "#6db9f7".try_into().unwrap(),
            bg_yellow: "#f0d197".try_into().unwrap(),
            fg: "#a5b0c5".try_into().unwrap(),
            purple: "#ca72e4".try_into().unwrap(),
            green: "#97ca72".try_into().unwrap(),
            orange: "#d99a5e".try_into().unwrap(),
            blue: "#5ab0f6".try_into().unwrap(),
            yellow: "#ebc275".try_into().unwrap(),
            cyan: "#4dbdcb".try_into().unwrap(),
            red: "#ef5f6b".try_into().unwrap(),
            grey: "#546178".try_into().unwrap(),
            light_grey: "#7d899f".try_into().unwrap(),
            dark_cyan: "#25747d".try_into().unwrap(),
            dark_red: "#a13131".try_into().unwrap(),
            dark_yellow: "#9a6b16".try_into().unwrap(),
            dark_purple: "#8f36a9".try_into().unwrap(),
            diff_add: "#303d27".try_into().unwrap(),
            diff_delete: "#3c2729".try_into().unwrap(),
            diff_change: "#18344c".try_into().unwrap(),
            diff_text: "#265478".try_into().unwrap(),
        },
        OneDarkStyle::Deep => ColorPalette {
            black: "#0c0e15".try_into().unwrap(),
            bg0: "#1a212e".try_into().unwrap(),
            bg1: "#21283b".try_into().unwrap(),
            bg2: "#283347".try_into().unwrap(),
            bg3: "#2a324a".try_into().unwrap(),
            bg_d: "#141b24".try_into().unwrap(),
            bg_blue: "#54b0fd".try_into().unwrap(),
            bg_yellow: "#f2cc81".try_into().unwrap(),
            fg: "#93a4c3".try_into().unwrap(),
            purple: "#c75ae8".try_into().unwrap(),
            green: "#8bcd5b".try_into().unwrap(),
            orange: "#dd9046".try_into().unwrap(),
            blue: "#41a7fc".try_into().unwrap(),
            yellow: "#efbd5d".try_into().unwrap(),
            cyan: "#34bfd0".try_into().unwrap(),
            red: "#f65866".try_into().unwrap(),
            grey: "#455574".try_into().unwrap(),
            light_grey: "#6c7d9c".try_into().unwrap(),
            dark_cyan: "#1b6a73".try_into().unwrap(),
            dark_red: "#992525".try_into().unwrap(),
            dark_yellow: "#8f610d".try_into().unwrap(),
            dark_purple: "#862aa1".try_into().unwrap(),
            diff_add: "#27341c".try_into().unwrap(),
            diff_delete: "#331c1e".try_into().unwrap(),
            diff_change: "#102b40".try_into().unwrap(),
            diff_text: "#1c4a6e".try_into().unwrap(),
        },
        OneDarkStyle::Warm => ColorPalette {
            black: "#191a1c".try_into().unwrap(),
            bg0: "#2c2d30".try_into().unwrap(),
            bg1: "#35373b".try_into().unwrap(),
            bg2: "#3e4045".try_into().unwrap(),
            bg3: "#404247".try_into().unwrap(),
            bg_d: "#242628".try_into().unwrap(),
            bg_blue: "#79b7eb".try_into().unwrap(),
            bg_yellow: "#e6cfa1".try_into().unwrap(),
            fg: "#b1b4b9".try_into().unwrap(),
            purple: "#c27fd7".try_into().unwrap(),
            green: "#99bc80".try_into().unwrap(),
            orange: "#c99a6e".try_into().unwrap(),
            blue: "#68aee8".try_into().unwrap(),
            yellow: "#dfbe81".try_into().unwrap(),
            cyan: "#5fafb9".try_into().unwrap(),
            red: "#e16d77".try_into().unwrap(),
            grey: "#646568".try_into().unwrap(),
            light_grey: "#8b8d91".try_into().unwrap(),
            dark_cyan: "#316a71".try_into().unwrap(),
            dark_red: "#914141".try_into().unwrap(),
            dark_yellow: "#8c6724".try_into().unwrap(),
            dark_purple: "#854897".try_into().unwrap(),
            diff_add: "#32352f".try_into().unwrap(),
            diff_delete: "#342f2f".try_into().unwrap(),
            diff_change: "#203444".try_into().unwrap(),
            diff_text: "#32526c".try_into().unwrap(),
        },
        OneDarkStyle::Warmer => ColorPalette {
            black: "#101012".try_into().unwrap(),
            bg0: "#232326".try_into().unwrap(),
            bg1: "#2c2d31".try_into().unwrap(),
            bg2: "#35363b".try_into().unwrap(),
            bg3: "#37383d".try_into().unwrap(),
            bg_d: "#1b1c1e".try_into().unwrap(),
            bg_blue: "#68aee8".try_into().unwrap(),
            bg_yellow: "#e2c792".try_into().unwrap(),
            fg: "#a7aab0".try_into().unwrap(),
            purple: "#bb70d2".try_into().unwrap(),
            green: "#8fb573".try_into().unwrap(),
            orange: "#c49060".try_into().unwrap(),
            blue: "#57a5e5".try_into().unwrap(),
            yellow: "#dbb671".try_into().unwrap(),
            cyan: "#51a8b3".try_into().unwrap(),
            red: "#de5d68".try_into().unwrap(),
            grey: "#5a5b5e".try_into().unwrap(),
            light_grey: "#818387".try_into().unwrap(),
            dark_cyan: "#2b5d63".try_into().unwrap(),
            dark_red: "#833b3b".try_into().unwrap(),
            dark_yellow: "#7c5c20".try_into().unwrap(),
            dark_purple: "#79428a".try_into().unwrap(),
            diff_add: "#282b26".try_into().unwrap(),
            diff_delete: "#2a2626".try_into().unwrap(),
            diff_change: "#1a2a37".try_into().unwrap(),
            diff_text: "#2c485f".try_into().unwrap(),
        },
        OneDarkStyle::Light => ColorPalette {
            black: "#101012".try_into().unwrap(),
            bg0: "#fafafa".try_into().unwrap(),
            bg1: "#f0f0f0".try_into().unwrap(),
            bg2: "#e6e6e6".try_into().unwrap(),
            bg3: "#dcdcdc".try_into().unwrap(),
            bg_d: "#c9c9c9".try_into().unwrap(),
            bg_blue: "#68aee8".try_into().unwrap(),
            bg_yellow: "#e2c792".try_into().unwrap(),
            fg: "#383a42".try_into().unwrap(),
            purple: "#a626a4".try_into().unwrap(),
            green: "#50a14f".try_into().unwrap(),
            orange: "#c18401".try_into().unwrap(),
            blue: "#4078f2".try_into().unwrap(),
            yellow: "#986801".try_into().unwrap(),
            cyan: "#0184bc".try_into().unwrap(),
            red: "#e45649".try_into().unwrap(),
            grey: "#a0a1a7".try_into().unwrap(),
            light_grey: "#818387".try_into().unwrap(),
            dark_cyan: "#2b5d63".try_into().unwrap(),
            dark_red: "#833b3b".try_into().unwrap(),
            dark_yellow: "#7c5c20".try_into().unwrap(),
            dark_purple: "#79428a".try_into().unwrap(),
            diff_add: "#e2fbe4".try_into().unwrap(),
            diff_delete: "#fce2e5".try_into().unwrap(),
            diff_change: "#e2ecfb".try_into().unwrap(),
            diff_text: "#cad3e0".try_into().unwrap(),
        },
    }
}

pub fn merge_palletes() -> ColorPalette {
    let def_palette = ColorPalette::default();
    let new_colors = match GLOBAL_CONFIG.read() {
        Ok(config) => config.colors.clone().unwrap_or_default(),
        Err(_) => ConfigColorPalette::default(),
    };

    ColorPalette {
        black: new_colors.black.unwrap_or(def_palette.black),
        bg0: new_colors.bg0.unwrap_or(def_palette.bg0),
        bg1: new_colors.bg1.unwrap_or(def_palette.bg1),
        bg2: new_colors.bg2.unwrap_or(def_palette.bg2),
        bg3: new_colors.bg3.unwrap_or(def_palette.bg3),
        bg_d: new_colors.bg_d.unwrap_or(def_palette.bg_d),
        bg_blue: new_colors.bg_blue.unwrap_or(def_palette.bg_blue),
        bg_yellow: new_colors.bg_yellow.unwrap_or(def_palette.bg_yellow),
        fg: new_colors.fg.unwrap_or(def_palette.fg),
        purple: new_colors.purple.unwrap_or(def_palette.purple),
        green: new_colors.green.unwrap_or(def_palette.green),
        orange: new_colors.orange.unwrap_or(def_palette.orange),
        blue: new_colors.blue.unwrap_or(def_palette.blue),
        yellow: new_colors.yellow.unwrap_or(def_palette.yellow),
        cyan: new_colors.cyan.unwrap_or(def_palette.cyan),
        red: new_colors.red.unwrap_or(def_palette.red),
        grey: new_colors.grey.unwrap_or(def_palette.grey),
        light_grey: new_colors.light_grey.unwrap_or(def_palette.light_grey),
        dark_cyan: new_colors.dark_cyan.unwrap_or(def_palette.dark_cyan),
        dark_red: new_colors.dark_red.unwrap_or(def_palette.dark_red),
        dark_yellow: new_colors.dark_yellow.unwrap_or(def_palette.dark_yellow),
        dark_purple: new_colors.dark_purple.unwrap_or(def_palette.dark_purple),
        diff_add: new_colors.diff_add.unwrap_or(def_palette.diff_add),
        diff_delete: new_colors.diff_delete.unwrap_or(def_palette.diff_delete),
        diff_change: new_colors.diff_change.unwrap_or(def_palette.diff_change),
        diff_text: new_colors.diff_text.unwrap_or(def_palette.diff_text),
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        let pal;
        {
            match GLOBAL_CONFIG.read() {
                Ok(conf) => pal = get_palette(conf.style),
                Err(_) => pal = get_palette(OneDarkStyle::default()),
            }
        }
        pal
    }
}
