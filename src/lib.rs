use nvim_oxi::{
    api::{self, opts::SetKeymapOpts, types::Mode},
    conversion::{Error as ConversionError, FromObject, ToObject},
    lua,
    serde::{Deserializer, Serializer},
    Dictionary, Function, Object,
};
use serde::{Deserialize, Serialize};

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
        if let Ok(config) = get_global_config::<ConfigColorPalette>() {
            config.style
        } else {
            OneDarkStyle::Dark
        }
    }
}

// Function to get global config or return None if it doesn't exist
fn get_global_config<T: Default + for<'a> Deserialize<'a>>() -> nvim_oxi::Result<OneDarkConfig<T>> {
    Ok(api::get_var("onedarkrs_config")?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CodeStyle {
    comments: String,
    keywords: String,
    functions: String,
    strings: String,
    variables: String,
}

impl Default for CodeStyle {
    fn default() -> Self {
        if let Ok(config) = get_global_config::<ConfigColorPalette>() {
            config.code_style
        } else {
            CodeStyle {
                comments: "italic".into(),
                keywords: "none".into(),
                functions: "none".into(),
                strings: "none".into(),
                variables: "none".into(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct LualineConfig {
    transparent: bool,
}

impl Default for LualineConfig {
    fn default() -> Self {
        if let Ok(config) = get_global_config::<ConfigColorPalette>() {
            config.lualine
        } else {
            LualineConfig { transparent: false }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticsConfig {
    darker: bool,
    undercurl: bool,
    background: bool,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        if let Ok(config) = get_global_config::<ConfigColorPalette>() {
            config.diagnostics
        } else {
            DiagnosticsConfig {
                darker: true,
                undercurl: true,
                background: true,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct OneDarkConfig<T> {
    style: OneDarkStyle,
    toggle_style_list: Vec<OneDarkStyle>,
    toggle_style_index: i64,
    toggle_style_key: Option<String>,
    transparent: bool,
    term_colors: bool,
    ending_tildes: bool,
    cmp_itemkind_reverse: bool,
    loaded: bool,
    code_style: CodeStyle,
    lualine: LualineConfig,
    colors: Option<T>,
    highlights: Option<ConfigHighlights>,
    diagnostics: DiagnosticsConfig,
}

impl<T: Default + for<'a> Deserialize<'a>> Default for OneDarkConfig<T> {
    fn default() -> Self {
        if let Ok(config) = get_global_config() {
            config
        } else {
            OneDarkConfig {
                toggle_style_list: OneDarkStyle::all_styles(),
                toggle_style_index: 0,
                toggle_style_key: None,
                transparent: false,
                term_colors: true,
                ending_tildes: false,
                cmp_itemkind_reverse: false,
                loaded: true, // TODO: Review why this is
                highlights: None,
                colors: None,
                style: OneDarkStyle::default(),
                code_style: CodeStyle::default(),
                lualine: LualineConfig::default(),
                diagnostics: DiagnosticsConfig::default(),
            }
        }
    }
}

impl<T: Default + for<'a> Deserialize<'a>> FromObject for OneDarkConfig<T> {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl<T: Default + for<'a> Serialize> ToObject for OneDarkConfig<T> {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl<T: Default + for<'a> Deserialize<'a>> lua::Poppable for OneDarkConfig<T> {
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl<T: Default + for<'a> Serialize> lua::Pushable for OneDarkConfig<T> {
    unsafe fn push(self, lstate: *mut lua::ffi::State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

#[nvim_oxi::plugin]
fn onedark_nvim_rs() -> nvim_oxi::Result<Dictionary> {
    // Create colorscheme function
    let colorscheme: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(|()| -> nvim_oxi::Result<()> {
            api::command("hi clear")?;
            if api::eval::<bool>("exists('syntax_on')")? {
                api::command("syntax reset")?;
            }

            api::set_option_value("termguicolors", true, &Default::default())?;
            api::set_var("colors_name", "onedark_nvim_rs")?;

            let _background = api::get_option_value::<String>("background", &Default::default())?;

            //let mut config = get_global_config().unwrap_or_default();
            //if background == "light" || config.style == OneDarkStyle::Light {
            //    config.style = OneDarkStyle::Light;
            //}

            // Call setup functions from other modules
            crate::highlights::setup()?;
            crate::terminal::setup()?;
            Ok(())
        });

    // Create toggle function
    let toggle: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(move |()| -> nvim_oxi::Result<()> {
            if let Ok(mut config) = get_global_config::<ConfigColorPalette>() {
                let index = config.toggle_style_index + 1;
                let new_index = if index as usize >= config.toggle_style_list.len() {
                    0
                } else {
                    index
                };
                let new_style = config.toggle_style_list[new_index as usize];

                config.style = new_style;
                config.toggle_style_index = new_index;

                if new_style == OneDarkStyle::Light {
                    api::set_option_value("background", "light", &Default::default())?;
                } else {
                    api::set_option_value("background", "dark", &Default::default())?;
                }

                api::command("colorscheme onedark_nvim_rs")?;
            }

            Ok(())
        });

    //Create setup function that accepts a OneDarkConfig directly
    let setup = Function::from_fn(|opts: Option<Object>| {
        if let Some(obj) = opts {
            // If opts are passed:
            // If "onedarkrs_config" was not set, opts are merged with default, and "onedarkrs_config" is set
            // Else, opts are merged with "onedarkrs_config", and "onedarkrs_config" is reset
            match OneDarkConfig::<ConfigColorPalette>::from_object(obj) {
                Ok(config) => match config.to_object() {
                    Ok(var) => match api::set_var("onedarkrs_config", var) {
                        Ok(_) => {}
                        Err(e) => {
                            api::err_writeln(&format!("Failed to set onedarkrs_config: {}", e))
                        }
                    },
                    Err(e) => {
                        api::err_writeln(&format!("Failed to convert to object: {}", e));
                    }
                },
                Err(e) => {
                    api::err_writeln(&format!("Failed to parse config: {}", e));
                }
            }
        } else {
            // If opts are not passed:
            // Default already checks if "onedarkrs_config" is set, so no need to check again
            // But this also takes care setting "onedarkrs_config" to default if it wasn't set
            // Optional: set "onedarkrs_config" only if needed (if get_global_config().is_none())
            match api::set_var(
                "onedarkrs_config",
                OneDarkConfig::<ConfigColorPalette>::default(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    api::err_writeln(&format!("Failed to set default onedarkrs_config: {}", e))
                }
            }
        }
        if let Ok(mut config) = get_global_config::<ConfigColorPalette>() {
            // false || true = true
            // true || true = true
            config.loaded = config.loaded || true;
            if let Some(key) = config.toggle_style_key {
                match api::set_keymap(
                    Mode::Normal,
                    &key,
                    "<cmd>lua require(\"onedark_nvim_rs\").toggle()<cr>",
                    &SetKeymapOpts::builder().silent(true).noremap(true).build(),
                ) {
                    Ok(_) => {}
                    Err(e) => api::err_writeln(&format!("Failed to set keymap: {}", e)),
                }
            }
        };
    });

    // Create load function
    let load: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(|()| -> nvim_oxi::Result<()> {
            api::command("colorscheme onedark_nvim_rs")?;
            Ok(())
        });

    // Return the plugin API
    Ok(Dictionary::from_iter::<[(&str, Object); 4]>([
        ("colorscheme", colorscheme.into()),
        ("toggle", toggle.into()),
        ("setup", setup.into()),
        ("load", load.into()),
    ]))
}
