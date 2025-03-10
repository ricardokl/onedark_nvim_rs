use nvim_oxi::{
    Dictionary, Function, Object,
    api::{self, opts::*, types::*},
    conversion::{Error as ConversionError, FromObject, ToObject},
    serde::{Deserializer, Serializer},
};
use serde::{Deserialize, Serialize};

mod highlights;
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

    // Get all styles as strings
    fn all_styles_as_strings() -> Vec<String> {
        Self::all_styles()
            .iter()
            .map(|s| s.as_str().to_string())
            .collect()
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

// Default functions for config fields
fn default_style() -> OneDarkStyle {
    OneDarkStyle::Dark
}

fn default_toggle_style_list() -> Vec<OneDarkStyle> {
    OneDarkStyle::all_styles()
}

fn default_true() -> bool {
    true
}

fn default_comments_style() -> String {
    "italic".to_string()
}

fn default_none_style() -> String {
    "none".to_string()
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
struct CodeStyle {
    #[serde(default = "default_comments_style")]
    comments: String,

    #[serde(default = "default_none_style")]
    keywords: String,

    #[serde(default = "default_none_style")]
    functions: String,

    #[serde(default = "default_none_style")]
    strings: String,

    #[serde(default = "default_none_style")]
    variables: String,
}

impl Default for CodeStyle {
    fn default() -> Self {
        Self {
            comments: default_comments_style(),
            keywords: default_none_style(),
            functions: default_none_style(),
            strings: default_none_style(),
            variables: default_none_style(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct LualineConfig {
    #[serde(default)]
    transparent: bool,
}

impl Default for LualineConfig {
    fn default() -> Self {
        Self { transparent: false }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct DiagnosticsConfig {
    #[serde(default = "default_true")]
    darker: bool,

    #[serde(default = "default_true")]
    undercurl: bool,

    #[serde(default = "default_true")]
    background: bool,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            darker: true,
            undercurl: true,
            background: true,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct OneDarkConfig {
    #[serde(with = "style_string_serializer", default = "default_style")]
    style: OneDarkStyle,

    #[serde(with = "style_vec_serializer", default = "default_toggle_style_list")]
    toggle_style_list: Vec<OneDarkStyle>,

    #[serde(default)]
    toggle_style_index: i64,

    #[serde(default)]
    toggle_style_key: Option<String>,

    #[serde(default)]
    transparent: bool,

    #[serde(default = "default_true")]
    term_colors: bool,

    #[serde(default)]
    ending_tildes: bool,

    #[serde(default)]
    cmp_itemkind_reverse: bool,

    #[serde(default = "default_true")]
    loaded: bool,

    #[serde(default)]
    code_style: CodeStyle,

    #[serde(default)]
    lualine: LualineConfig,

    #[serde(default)]
    colors: Dictionary,

    #[serde(default)]
    highlights: Dictionary,

    #[serde(default)]
    diagnostics: DiagnosticsConfig,
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

#[nvim_oxi::plugin]
fn onedark_nvim_rs() -> nvim_oxi::Result<Dictionary> {
    // Get all styles as strings for API compatibility
    let styles_list = OneDarkStyle::all_styles_as_strings();

    // Set up default config
    let default_config = OneDarkConfig {
        style: default_style(),
        toggle_style_list: default_toggle_style_list(),
        toggle_style_index: 0,
        toggle_style_key: None,
        transparent: false,
        term_colors: default_true(),
        ending_tildes: false,
        cmp_itemkind_reverse: false,
        loaded: default_true(),
        code_style: Default::default(),
        lualine: Default::default(),
        colors: Dictionary::new(),
        highlights: Dictionary::new(),
        diagnostics: Default::default(),
    };

    // Convert to Dictionary for Neovim API
    let default_config_dict = default_config.to_object()?.try_into::<Dictionary>()?;

    // Initialize global config if not already set
    let g_onedark_config = api::get_var::<Option<Dictionary>>("onedark_config").unwrap_or(None);

    if g_onedark_config.is_none() {
        api::set_var("onedark_config", default_config_dict.clone())?;
    } else {
        // Merge with existing config
        let mut config = g_onedark_config.unwrap();
        for (key, value) in default_config_dict.iter() {
            if !config.contains_key(key) {
                config.insert(key.clone(), value.clone());
            }
        }
        api::set_var("onedark_config", config)?;
    }

    // Helper function to get config
    fn get_config() -> nvim_oxi::Result<OneDarkConfig> {
        let config_dict = api::get_var::<Dictionary>("onedark_config")?;
        let config = OneDarkConfig::from_object(config_dict.into())?;
        Ok(config)
    }

    // Helper function to set config
    fn set_config(config: OneDarkConfig) -> nvim_oxi::Result<()> {
        let config_dict = config.to_object()?.try_into::<Dictionary>()?;
        api::set_var("onedark_config", config_dict)?;
        Ok(())
    }

    // Helper function to set a single option
    fn set_option(opt: &str, value: Object) -> nvim_oxi::Result<()> {
        let mut config_dict = api::get_var::<Dictionary>("onedark_config")?;
        config_dict.insert(opt.to_string(), value);
        api::set_var("onedark_config", config_dict)?;
        Ok(())
    }

    // Create colorscheme function
    let colorscheme = Function::from_fn(|()| -> nvim_oxi::Result<()> {
        api::command("hi clear")?;
        if api::eval::<bool>("exists('syntax_on')")? {
            api::command("syntax reset")?;
        }

        api::set_option_value("termguicolors", true, &Default::default())?;
        api::set_var("colors_name", "onedark")?;

        let background = api::get_option_value::<String>("background", &Default::default())?;
        let mut config = get_config()?;

        if background == "light" || config.style == OneDarkStyle::Light {
            config.style = OneDarkStyle::Light;
            set_config(config)?;
        }

        // Call setup functions from other modules
        highlights::setup()?;
        terminal::setup()?;

        Ok(())
    });

    // Create toggle function
    let toggle = Function::from_fn(|()| -> nvim_oxi::Result<()> {
        let mut config = get_config()?;
        let index = config.toggle_style_index + 1;
        let new_index = if index as usize >= config.toggle_style_list.len() {
            0
        } else {
            index
        };
        let new_style = config.toggle_style_list[new_index as usize];

        config.style = new_style;
        config.toggle_style_index = new_index;
        set_config(config)?;

        if new_style == OneDarkStyle::Light {
            api::set_option_value("background", "light", &Default::default())?;
        } else {
            api::set_option_value("background", "dark", &Default::default())?;
        }

        api::command("colorscheme onedark")?;

        Ok(())
    });

    // Create setup function that accepts a OneDarkConfig directly
    let setup = Function::from_fn(|opts: Option<OneDarkConfig>| -> nvim_oxi::Result<()> {
        // Get current config
        let current_config = get_config()?;
        
        // If options were provided, merge them with current config
        if let Some(partial_config) = opts {
            // Create a merged config - start with current and override with provided values
            let merged_config = OneDarkConfig {
                // Only override style if it's not the default
                style: if partial_config.style != default_style() {
                    partial_config.style
                } else {
                    current_config.style
                },
                
                // Only override toggle_style_list if it's not the default
                toggle_style_list: if partial_config.toggle_style_list != default_toggle_style_list() {
                    partial_config.toggle_style_list
                } else {
                    current_config.toggle_style_list
                },
                
                // For other fields, prefer the partial config value if provided
                toggle_style_index: current_config.toggle_style_index,
                toggle_style_key: partial_config.toggle_style_key.or(current_config.toggle_style_key),
                transparent: partial_config.transparent,
                term_colors: partial_config.term_colors,
                ending_tildes: partial_config.ending_tildes,
                cmp_itemkind_reverse: partial_config.cmp_itemkind_reverse,
                loaded: current_config.loaded, // Keep the loaded status
                
                // For nested structures, use the provided ones
                code_style: partial_config.code_style,
                lualine: partial_config.lualine,
                diagnostics: partial_config.diagnostics,
                
                // For dictionaries, only replace if not empty
                colors: if !partial_config.colors.is_empty() {
                    partial_config.colors
                } else {
                    current_config.colors
                },
                highlights: if !partial_config.highlights.is_empty() {
                    partial_config.highlights
                } else {
                    current_config.highlights
                },
            };
            
            // Save the merged config
            set_config(merged_config)?;
        }
        
        // Set up toggle key if configured
        let config = get_config()?;
        if let Some(toggle_key) = &config.toggle_style_key {
            if !toggle_key.is_empty() {
                let opts = SetKeymapOpts::builder().noremap(true).silent(true).build();
                
                api::set_keymap(
                    Mode::Normal,
                    toggle_key,
                    "<cmd>lua require('onedark').toggle()<cr>",
                    &opts,
                )?;
            }
        }
        
        Ok(())
    });

    // Create load function
    let load = Function::from_fn(|()| -> nvim_oxi::Result<()> {
        api::command("colorscheme onedark")?;
        Ok(())
    });

    // Return the plugin API
    Ok(Dictionary::from_iter([
        ("colorscheme", colorscheme.into()),
        ("toggle", toggle.into()),
        ("setup", setup.into()),
        ("load", load.into()),
        ("styles_list", styles_list.into()),
    ]))
}
