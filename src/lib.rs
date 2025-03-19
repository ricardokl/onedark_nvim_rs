use nvim_oxi::{
    api::{self, err_writeln, opts::SetKeymapOpts, types::Mode},
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
        if let Ok(config) = get_global_config() {
            config.style
        } else {
            OneDarkStyle::Dark
        }
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
            comments: "italic".into(),
            keywords: "none".into(),
            functions: "none".into(),
            strings: "none".into(),
            variables: "none".into(),
        },
        lualine: LualineConfig { transparent: false },
        diagnostics: DiagnosticsConfig {
            darker: true,
            undercurl: true,
            background: true,
        },
    })
});

// Function to get global config
fn get_global_config() -> nvim_oxi::Result<OneDarkConfig> {
    match GLOBAL_CONFIG.read() {
        Ok(config) => {
            // Create a config with the same settings but possibly different 'colors' type
            let result: OneDarkConfig = OneDarkConfig {
                style: config.style,
                toggle_style_list: config.toggle_style_list.clone(),
                toggle_style_index: config.toggle_style_index,
                toggle_style_key: config.toggle_style_key.clone(),
                transparent: config.transparent,
                term_colors: config.term_colors,
                ending_tildes: config.ending_tildes,
                cmp_itemkind_reverse: config.cmp_itemkind_reverse,
                code_style: config.code_style.clone(),
                lualine: config.lualine.clone(),
                diagnostics: config.diagnostics.clone(),
                colors: config.colors.clone(),
                highlights: config.highlights.clone(),
            };
            Ok(result)
        }
        Err(_) => {
            api::err_writeln("Failed to read global config");
            Err(nvim_oxi::Error::from(nvim_oxi::api::Error::Other(
                "Failed to read global config".into(),
            )))
        }
    }
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
        if let Ok(config) = get_global_config() {
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
        if let Ok(config) = get_global_config() {
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
        if let Ok(config) = get_global_config() {
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
struct OneDarkConfig {
    style: OneDarkStyle,
    toggle_style_list: Vec<OneDarkStyle>,
    toggle_style_index: i64,
    toggle_style_key: Option<String>,
    transparent: bool,
    term_colors: bool,
    ending_tildes: bool,
    cmp_itemkind_reverse: bool,
    code_style: CodeStyle,
    lualine: LualineConfig,
    colors: Option<ConfigColorPalette>,
    highlights: Option<ConfigHighlights>,
    diagnostics: DiagnosticsConfig,
}

impl Default for OneDarkConfig {
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

#[nvim_oxi::plugin]
fn onedark_nvim_rs() -> nvim_oxi::Result<Dictionary> {
    // Create colorscheme function
    let colorscheme = Function::from_fn(|()| {
        match api::command("hi clear") {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };
        let syntax_on: u8 = match api::get_var("syntax_on") {
            Ok(x) => x,
            Err(e) => {
                err_writeln(&format!("{}", e));
                return ();
            }
        };
        if syntax_on == 1 {
            match api::command("syntax reset") {
                Ok(_) => {}
                Err(e) => err_writeln(&format!("{}", e)),
            };
        }

        match api::set_option_value("termguicolors", true, &Default::default()) {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };
        match api::set_var("colors_name", "onedark_nvim_rs") {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };

        let _background = match api::get_option_value::<String>("background", &Default::default()) {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };

        //let mut config = get_global_config().unwrap_or_default();
        //if background == "light" || config.style == OneDarkStyle::Light {
        //    config.style = OneDarkStyle::Light;
        //}

        // Call setup functions from other modules
        match crate::highlights::setup() {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };
        match crate::terminal::setup() {
            Ok(_) => {}
            Err(e) => err_writeln(&format!("{}", e)),
        };
    });

    // Create toggle function
    let toggle = Function::from_fn(|()| match GLOBAL_CONFIG.write() {
        Ok(mut config) => {
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
                match api::set_option_value("background", "light", &Default::default()) {
                    Ok(_) => {}
                    Err(e) => err_writeln(&format!("{}", e)),
                };
            } else {
                match api::set_option_value("background", "dark", &Default::default()) {
                    Ok(_) => {}
                    Err(e) => err_writeln(&format!("{}", e)),
                };
            }

            match api::command("colorscheme onedark_nvim_rs") {
                Ok(_) => {}
                Err(e) => err_writeln(&format!("{}", e)),
            };
        }
        Err(_) => {
            api::err_writeln("Failed to write to global config in toggle");
        }
    });

    //Create setup function that accepts a OneDarkConfig directly
    let setup = Function::from_fn(|opts: Option<Object>| {
        if let Some(obj) = opts {
            match OneDarkConfig::from_object(obj) {
                Ok(new_config) => {
                    match GLOBAL_CONFIG.write() {
                        Ok(mut config) => {
                            // Update the config with the new values
                            *config = new_config;
                        }
                        Err(_) => {
                            api::err_writeln("Failed to write to global config");
                        }
                    }
                }
                Err(e) => {
                    api::err_writeln(&format!("Failed to parse config: {}", e));
                }
            }
        }

        // Handle toggle key mapping
        match GLOBAL_CONFIG.read() {
            Ok(config) => {
                if let Some(key) = &config.toggle_style_key {
                    match api::set_keymap(
                        Mode::Normal,
                        key,
                        "<cmd>lua require(\"onedark_nvim_rs\").toggle()<cr>",
                        &SetKeymapOpts::builder().silent(true).noremap(true).build(),
                    ) {
                        Ok(_) => {}
                        Err(e) => api::err_writeln(&format!("Failed to set keymap: {}", e)),
                    }
                }
            }
            Err(_) => {
                api::err_writeln("Failed to read global config for keymap setup");
            }
        }
    });

    // Create load function
    let load: Function<(), ()> = Function::from_fn(|_| {
        match api::command("colorscheme onedark_nvim_rs") {
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
