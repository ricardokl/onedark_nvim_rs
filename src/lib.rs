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
    // Convert to string for API compatibility
    fn as_str(&self) -> &'static str {
        match self {
            OneDarkStyle::Dark => "dark",
            OneDarkStyle::Darker => "darker",
            OneDarkStyle::Cool => "cool",
            OneDarkStyle::Deep => "deep",
            OneDarkStyle::Warm => "warm",
            OneDarkStyle::Warmer => "warmer",
            OneDarkStyle::Light => "light",
        }
    }

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

    // Parse from string
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(OneDarkStyle::Dark),
            "darker" => Ok(OneDarkStyle::Darker),
            "cool" => Ok(OneDarkStyle::Cool),
            "deep" => Ok(OneDarkStyle::Deep),
            "warm" => Ok(OneDarkStyle::Warm),
            "warmer" => Ok(OneDarkStyle::Warmer),
            "light" => Ok(OneDarkStyle::Light),
            _ => Err(format!("Invalid style: {}", s)),
        }
    }
}

impl Default for OneDarkStyle {
    fn default() -> Self {
        if let Some(config) = get_global_config() {
            config.style
        } else {
            OneDarkStyle::Dark
        }
    }
}

// Function to get global config or return None if it doesn't exist
fn get_global_config() -> Option<OneDarkConfig> {
    let result: Option<OneDarkConfig> = api::get_var("onedark_config").ok();
    result
}

// Serializer modules to handle string conversion for Neovim compatibility
mod style_string_serializer {
    use super::OneDarkStyle;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(style: &OneDarkStyle, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(style.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OneDarkStyle, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OneDarkStyle::from_str(&s).map_err(D::Error::custom)
    }
}

mod style_vec_serializer {
    use super::OneDarkStyle;
    use serde::de::Error;
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(styles: &[OneDarkStyle], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(styles.len()))?;
        for style in styles {
            seq.serialize_element(style.as_str())?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<OneDarkStyle>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings = Vec::<String>::deserialize(deserializer)?;
        let mut styles = Vec::with_capacity(strings.len());

        for s in strings {
            let style = OneDarkStyle::from_str(&s).map_err(D::Error::custom)?;
            styles.push(style);
        }

        Ok(styles)
    }
}

#[derive(Clone, Serialize, Deserialize)]
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
        if let Some(config) = get_global_config() {
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

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
struct LualineConfig {
    transparent: bool,
}

impl Default for LualineConfig {
    fn default() -> Self {
        if let Some(config) = get_global_config() {
            config.lualine
        } else {
            LualineConfig { transparent: false }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct DiagnosticsConfig {
    darker: bool,
    undercurl: bool,
    background: bool,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        if let Some(config) = get_global_config() {
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

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct ColorsConfig {
    // This will be populated with actual color fields later
    // For now, it's just a placeholder
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct HighlightsConfig {
    // This will be populated with actual highlight fields later
    // For now, it's just a placeholder
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
struct OneDarkConfig {
    #[serde(with = "style_string_serializer")]
    style: OneDarkStyle,
    #[serde(with = "style_vec_serializer")]
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
    colors: ColorsConfig,
    highlights: HighlightsConfig,
    diagnostics: DiagnosticsConfig,
}

impl Default for OneDarkConfig {
    fn default() -> Self {
        if let Some(config) = get_global_config() {
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
                ..Default::default()
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
    // Helper function to set a single option
    //fn set_option(opt: &str, value: Object) -> nvim_oxi::Result<()> {
    //    let mut config_dict = api::get_var::<Dictionary>("onedark_config")?;
    //    config_dict.insert(opt.to_string(), value);
    //    api::set_var("onedark_config", config_dict)?;
    //    Ok(())
    //}
    let set_option: Function<(String, Object), nvim_oxi::Result<()>> =
        Function::from_fn(|arg: (String, Object)| -> nvim_oxi::Result<()> {
            let mut config_dict = api::get_var::<Dictionary>("onedark_config")?;
            config_dict.insert(arg.0.to_string(), arg.1);
            api::set_var("onedark_config", config_dict)?;
            Ok(())
        });

    // Create colorscheme function
    let colorscheme: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(|()| -> nvim_oxi::Result<()> {
            api::command("hi clear")?;
            if api::eval::<bool>("exists('syntax_on')")? {
                api::command("syntax reset")?;
            }

            api::set_option_value("termguicolors", true, &Default::default())?;
            api::set_var("colors_name", "onedark")?;

            let _background = api::get_option_value::<String>("background", &Default::default())?;

            //let mut config = get_global_config().unwrap_or_default();
            //if background == "light" || config.style == OneDarkStyle::Light {
            //    config.style = OneDarkStyle::Light;
            //}

            // Call setup functions from other modules
            //highlights::setup()?;
            //terminal::setup()?;

            Ok(())
        });

    // Create toggle function
    let toggle: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(move |()| -> nvim_oxi::Result<()> {
            if let Some(mut config) = get_global_config() {
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

                api::command("colorscheme onedark")?;
            }

            Ok(())
        });

    // Create setup function that accepts a OneDarkConfig directly
    let setup: Function<Option<OneDarkConfig>, nvim_oxi::Result<()>> =
        Function::from_fn(|opts: Option<OneDarkConfig>| -> nvim_oxi::Result<()> {
            if let Some(config) = opts {
                // If opts are passed:
                // If "onedark_config" was not set, opts are merged with default, and "onedark_config" is set
                // Else, opts are merged with "onedark_config", and "onedark_config" is reset
                api::set_var("onedark_config", config)?;
            } else {
                // If opts are not passed:
                // Default already checks if "onedark_config" is set, so no need to check again
                // But this also takes care setting "onedark_config" to default if it wasn't set
                // Optional: set "onedark_config" only if needed (if get_global_config().is_none())
                api::set_var("onedark_config", OneDarkConfig::default())?;
            }

            if let Some(mut config) = get_global_config() {
                // false || true = true
                // true || true = true
                config.loaded = config.loaded || true;
                if let Some(key) = config.toggle_style_key {
                    api::set_keymap(
                        Mode::Normal,
                        &key,
                        "<cmd>lua require(\"onedark\").toggle()<cr>",
                        &SetKeymapOpts::builder().silent(true).noremap(true).build(),
                    )?;
                }
            };
            Ok(())
        });

    // Create load function
    let load: Function<(), nvim_oxi::Result<()>> =
        Function::from_fn(|()| -> nvim_oxi::Result<()> {
            api::command("colorscheme onedark")?;
            Ok(())
        });

    // Return the plugin API
    Ok(Dictionary::from_iter::<[(&str, Object); 5]>([
        ("colorscheme", colorscheme.into()),
        ("toggle", toggle.into()),
        ("setup", setup.into()),
        ("load", load.into()),
        ("set_options", set_option.into()), //("styles_list", styles_list.into()),
    ]))
}
