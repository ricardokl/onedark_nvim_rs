use highlights::FmtType;
use nvim_oxi::{
    api::{self, err_writeln, opts::SetKeymapOpts, types::Mode, Error::Other},
    conversion::{Error as ConversionError, FromObject, ToObject},
    lua,
    serde::{Deserializer, Serializer},
    Dictionary, Function, Object,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

mod highlights;
mod palette;
mod terminal;
mod util;

use crate::highlights::ConfigHighlights;
use crate::palette::ConfigColorPalette;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum OneDarkStyle {
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
    fn all_styles() -> Vec<OneDarkStyle> {
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
        let style;
        {
            match GLOBAL_CONFIG.read() {
                Ok(config) => style = config.style,
                Err(_) => style = OneDarkStyle::Dark,
            }
        }
        style
    }
}

// Global configuration storage
static GLOBAL_CONFIG: Lazy<RwLock<OneDarkConfig>> = Lazy::new(|| {
    RwLock::new(OneDarkConfig {
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
    })
});

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodeStyle {
    comments: Option<Vec<FmtType>>,
    keywords: Option<Vec<FmtType>>,
    functions: Option<Vec<FmtType>>,
    strings: Option<Vec<FmtType>>,
    variables: Option<Vec<FmtType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LualineConfig {
    transparent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticsConfig {
    darker: bool,
    undercurl: bool,
    background: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct OneDarkConfig<'a> {
    style: OneDarkStyle,
    toggle_style_list: Vec<OneDarkStyle>,
    toggle_style_index: u8,
    toggle_style_key: Option<Box<str>>,
    transparent: bool,
    term_colors: bool,
    ending_tildes: bool,
    cmp_itemkind_reverse: bool,
    code_style: CodeStyle,
    lualine: LualineConfig,
    colors: Option<ConfigColorPalette<'a>>,
    highlights: Option<ConfigHighlights<'a>>,
    diagnostics: DiagnosticsConfig,
}

impl Default for OneDarkConfig<'_> {
    fn default() -> Self {
        let conf;
        {
            match GLOBAL_CONFIG.read() {
                Ok(config) => {
                    conf = config.clone();
                }
                Err(_) => {
                    conf = OneDarkConfig {
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
                    };
                }
            }
        }
        conf
    }
}

impl FromObject for OneDarkConfig<'_> {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for OneDarkConfig<'_> {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for OneDarkConfig<'_> {
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for OneDarkConfig<'_> {
    unsafe fn push(self, lstate: *mut lua::ffi::State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

fn toggle_fn() -> nvim_oxi::Result<()> {
    {
        let mut config = GLOBAL_CONFIG.write().map_err(|e| Other(format!("{}", e)))?;
        config.toggle_style_index =
            (config.toggle_style_index + 1) % config.toggle_style_list.len() as u8;
        config.style = config.toggle_style_list[config.toggle_style_index as usize];
        api::notify(
            &format!("New coloscheme style: {:?}", config.style),
            api::types::LogLevel::Info,
            &Default::default(),
        )?;
        if config.style == OneDarkStyle::Light {
            api::set_option_value("background", "light", &Default::default())?;
        } else {
            api::set_option_value("background", "dark", &Default::default())?;
        };
    }

    colorscheme_fn()?;
    Ok(())
}

fn colorscheme_fn() -> nvim_oxi::Result<()> {
    api::command("hi clear")?;

    if api::get_var::<u8>("syntax_on")? == 1 {
        api::command("syntax reset")?;
    }

    api::set_option_value("termguicolors", true, &Default::default())?;
    api::set_var("colors_name", "onedark_nvim_rs")?;

    //let background = api::get_option_value::<String>("background", &Default::default())?;
    //let mut config = get_global_config().unwrap_or_default();
    //if background == "light" || config.style == OneDarkStyle::Light {
    //    config.style = OneDarkStyle::Light;
    //}

    // Call setup functions from other modules
    crate::highlights::setup()?;
    crate::terminal::setup()?;
    Ok(())
}

fn setup_fn(opts: Option<Object>) -> nvim_oxi::Result<()> {
    {
        if let Some(obj) = opts {
            *GLOBAL_CONFIG.write().map_err(|e| Other(format!("{}", e)))? =
                OneDarkConfig::from_object(obj)?;
        }
    }

    if let Some(key) = &GLOBAL_CONFIG
        .try_read()
        .map_err(|e| Other(format!("{}", e)))?
        .toggle_style_key
    {
        api::set_keymap(
            Mode::Normal,
            key,
            "<cmd>lua require(\"onedark_nvim_rs\").toggle()<cr>",
            &SetKeymapOpts::builder().silent(true).noremap(true).build(),
        )?;
    }

    Ok(())
}

#[nvim_oxi::plugin]
fn onedark_nvim_rs() -> nvim_oxi::Result<Dictionary> {
    let colorscheme: Function<(), ()> = Function::from_fn(|()| match colorscheme_fn() {
        Ok(_) => {}
        Err(e) => api::err_writeln(&format!("{}", e)),
    });

    let toggle: Function<(), ()> = Function::from_fn(|()| match toggle_fn() {
        Ok(_) => {}
        Err(e) => api::err_writeln(&format!("{}", e)),
    });

    let setup = Function::from(|opts: Option<Object>| match setup_fn(opts) {
        Ok(_) => {}
        Err(e) => api::err_writeln(&format!("{}", e)),
    });

    let load: Function<(), ()> = Function::from_fn(|_| {
        match colorscheme_fn() {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };
    });

    // Return the plugin API
    Ok(Dictionary::from_iter::<[(&str, Object); 4]>([
        ("colorscheme", colorscheme.into()),
        ("toggle", toggle.into()),
        ("setup", setup.into()),
        ("load", load.into()),
    ]))
}
