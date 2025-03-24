use crate::highlights::ConfigHighlights;
use crate::highlights::FmtType;
use crate::palette::ConfigColorPalette;
use nvim_oxi::{
    conversion::{Error as ConversionError, FromObject, ToObject},
    lua,
    serde::{Deserializer, Serializer},
    Object,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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
        //let style;
        //{
        //    match GLOBAL_CONFIG.read() {
        //        Ok(config) => style = config.style,
        //        Err(_) => style = OneDarkStyle::Dark,
        //    }
        //}
        //style
    }
}

// Global configuration storage
pub static GLOBAL_CONFIG: Lazy<RwLock<OneDarkConfig>> =
    Lazy::new(|| OneDarkConfig::default().into());
//pub static GLOBAL_CONFIG: Lazy<RwLock<OneDarkConfig>> = Lazy::new(|| {
//    RwLock::new(OneDarkConfig {
//        toggle_style_list: OneDarkStyle::all_styles(),
//        toggle_style_index: 0,
//        toggle_style_key: None,
//        transparent: false,
//        term_colors: true,
//        ending_tildes: false,
//        cmp_itemkind_reverse: false,
//        highlights: None,
//        colors: None,
//        style: OneDarkStyle::Dark,
//        code_style: CodeStyle {
//            comments: Some(vec![FmtType::Italic]),
//            keywords: None,
//            functions: None,
//            strings: None,
//            variables: None,
//        },
//        lualine: LualineConfig { transparent: false },
//        diagnostics: DiagnosticsConfig {
//            darker: true,
//            undercurl: true,
//            background: true,
//        },
//    })
//});
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStyle {
    pub comments: Option<Vec<FmtType>>,
    pub keywords: Option<Vec<FmtType>>,
    pub functions: Option<Vec<FmtType>>,
    pub strings: Option<Vec<FmtType>>,
    pub variables: Option<Vec<FmtType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LualineConfig {
    pub transparent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsConfig {
    pub darker: bool,
    pub undercurl: bool,
    pub background: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OneDarkConfig {
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
    pub highlights: Option<ConfigHighlights>,
    pub diagnostics: DiagnosticsConfig,
}

impl Default for OneDarkConfig {
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
        //let conf;
        //{
        //    match GLOBAL_CONFIG.read() {
        //        Ok(config) => {
        //            conf = config.clone();
        //        }
        //        Err(_) => {
        //            conf = OneDarkConfig {
        //                toggle_style_list: OneDarkStyle::all_styles(),
        //                toggle_style_index: 0,
        //                toggle_style_key: None,
        //                transparent: false,
        //                term_colors: true,
        //                ending_tildes: false,
        //                cmp_itemkind_reverse: false,
        //                highlights: None,
        //                colors: None,
        //                style: OneDarkStyle::Dark,
        //                code_style: CodeStyle {
        //                    comments: Some(vec![FmtType::Italic]),
        //                    keywords: None,
        //                    functions: None,
        //                    strings: None,
        //                    variables: None,
        //                },
        //                lualine: LualineConfig { transparent: false },
        //                diagnostics: DiagnosticsConfig {
        //                    darker: true,
        //                    undercurl: true,
        //                    background: true,
        //                },
        //            };
        //        }
        //    }
        //}
        //conf
    }
}

impl FromObject for OneDarkConfig {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for OneDarkConfig {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for OneDarkConfig {
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for OneDarkConfig {
    unsafe fn push(self, lstate: *mut lua::ffi::State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
