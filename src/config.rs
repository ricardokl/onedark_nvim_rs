use crate::highlights::ConfigHighlights;
use crate::highlights::FmtType;
use crate::palette::ConfigColorPalette;
use nvim_oxi::{
    conversion::{Error as ConversionError, FromObject},
    serde::Deserializer,
    Object,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OneDarkStyle {
    Dark,
    Darker,
    Cool,
    Deep,
    Warm,
    Warmer,
    Light,
}

impl OneDarkStyle {
    // Get all available styles as a vector
    pub fn all_styles() -> Vec<OneDarkStyle> {
        vec![
            OneDarkStyle::Dark,
            OneDarkStyle::Darker,
            OneDarkStyle::Cool,
            OneDarkStyle::Deep,
            OneDarkStyle::Warm,
            OneDarkStyle::Warmer,
            OneDarkStyle::Light,
        ]
    }
}

impl Default for OneDarkStyle {
    fn default() -> Self {
        OneDarkStyle::Dark
    }
}

// Global configuration storage
pub static GLOBAL_CONFIG: Lazy<RwLock<OneDarkConfig>> =
    Lazy::new(|| OneDarkConfig::default().into());

#[derive(Debug, Clone, Deserialize)]
pub struct CodeStyle {
    pub comments: Option<Vec<FmtType>>,
    pub keywords: Option<Vec<FmtType>>,
    pub functions: Option<Vec<FmtType>>,
    pub strings: Option<Vec<FmtType>>,
    pub variables: Option<Vec<FmtType>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LualineConfig {
    pub transparent: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticsConfig {
    pub darker: bool,
    pub undercurl: bool,
    pub background: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OneDarkConfig<'a> {
    pub style: OneDarkStyle,
    pub toggle_style_list: Vec<OneDarkStyle>,
    pub toggle_style_index: u8,
    pub toggle_style_key: Option<Box<str>>,
    pub transparent: bool,
    pub term_colors: bool,
    pub ending_tildes: bool,
    pub cmp_itemkind_reverse: bool,
    pub code_style: CodeStyle,
    pub lualine: LualineConfig,
    pub colors: Option<ConfigColorPalette>,
    pub highlights: Option<ConfigHighlights<'a>>,
    pub diagnostics: DiagnosticsConfig,
}

impl Default for OneDarkConfig<'_> {
    fn default() -> Self {
        OneDarkConfig {
            toggle_style_list: OneDarkStyle::all_styles(),
            toggle_style_index: 0,
            toggle_style_key: None,
            transparent: false,
            term_colors: true,
            ending_tildes: false,
            cmp_itemkind_reverse: false,
            highlights: None,
            colors: None,
            style: OneDarkStyle::Dark,
            code_style: CodeStyle {
                comments: Some(vec![FmtType::Italic]),
                keywords: None,
                functions: None,
                strings: None,
                variables: None,
            },
            lualine: LualineConfig { transparent: false },
            diagnostics: DiagnosticsConfig {
                darker: true,
                undercurl: true,
                background: true,
            },
        }
    }
}

impl FromObject for OneDarkConfig<'_> {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Ok(Self::deserialize(Deserializer::new(obj))?)
    }
}
