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

// Serializer modules to handle string conversion for Neovim compatibility
mod style_string_serializer {
    use super::OneDarkStyle;
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;

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
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;
    use serde::ser::SerializeSeq;

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
    colors: Dictionary,
    highlights: Dictionary,
    diagnostics: DiagnosticsConfig,
}

#[derive(Clone, Serialize, Deserialize)]
struct CodeStyle {
    comments: String,
    keywords: String,
    functions: String,
    strings: String,
    variables: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct LualineConfig {
    transparent: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct DiagnosticsConfig {
    darker: bool,
    undercurl: bool,
    background: bool,
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
        style: OneDarkStyle::Dark,
        toggle_style_list: OneDarkStyle::all_styles(),
        toggle_style_index: 0,
        toggle_style_key: None,
        transparent: false,
        term_colors: true,
        ending_tildes: false,
        cmp_itemkind_reverse: false,
        loaded: true,
        code_style: CodeStyle {
            comments: "italic".to_string(),
            keywords: "none".to_string(),
            functions: "none".to_string(),
            strings: "none".to_string(),
            variables: "none".to_string(),
        },
        lualine: LualineConfig {
            transparent: false,
        },
        colors: Dictionary::new(),
        highlights: Dictionary::new(),
        diagnostics: DiagnosticsConfig {
            darker: true,
            undercurl: true,
            background: true,
        },
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

    // Create setup function
    let setup = Function::from_fn(|opts: Option<Dictionary>| -> nvim_oxi::Result<()> {
        let mut config = get_config()?;

        if let Some(opts) = opts {
            // Handle style if present
            if let Ok(style_str) = opts.get::<String>("style") {
                if let Ok(style) = OneDarkStyle::from_str(&style_str) {
                    config.style = style;
                }
            }

            // Handle toggle_style_list separately if present
            if let Ok(style_strings) = opts.get::<Vec<String>>("toggle_style_list") {
                let mut styles = Vec::new();
                for s in style_strings {
                    if let Ok(style) = OneDarkStyle::from_str(&s) {
                        styles.push(style);
                    }
                }
                if !styles.is_empty() {
                    config.toggle_style_list = styles;
                }
            }

            // Handle code_style if present
            if let Ok(code_style_dict) = opts.get::<Dictionary>("code_style") {
                if let Ok(comments) = code_style_dict.get::<String>("comments") {
                    config.code_style.comments = comments;
                }
                if let Ok(keywords) = code_style_dict.get::<String>("keywords") {
                    config.code_style.keywords = keywords;
                }
                if let Ok(functions) = code_style_dict.get::<String>("functions") {
                    config.code_style.functions = functions;
                }
                if let Ok(strings) = code_style_dict.get::<String>("strings") {
                    config.code_style.strings = strings;
                }
                if let Ok(variables) = code_style_dict.get::<String>("variables") {
                    config.code_style.variables = variables;
                }
            }

            // Handle lualine if present
            if let Ok(lualine_dict) = opts.get::<Dictionary>("lualine") {
                if let Ok(transparent) = lualine_dict.get::<bool>("transparent") {
                    config.lualine.transparent = transparent;
                }
            }

            // Handle diagnostics if present
            if let Ok(diagnostics_dict) = opts.get::<Dictionary>("diagnostics") {
                if let Ok(darker) = diagnostics_dict.get::<bool>("darker") {
                    config.diagnostics.darker = darker;
                }
                if let Ok(undercurl) = diagnostics_dict.get::<bool>("undercurl") {
                    config.diagnostics.undercurl = undercurl;
                }
                if let Ok(background) = diagnostics_dict.get::<bool>("background") {
                    config.diagnostics.background = background;
                }
            }

            // Handle other top-level options
            if let Ok(toggle_key) = opts.get::<String>("toggle_style_key") {
                config.toggle_style_key = Some(toggle_key);
            }
            if let Ok(transparent) = opts.get::<bool>("transparent") {
                config.transparent = transparent;
            }
            if let Ok(term_colors) = opts.get::<bool>("term_colors") {
                config.term_colors = term_colors;
            }
            if let Ok(ending_tildes) = opts.get::<bool>("ending_tildes") {
                config.ending_tildes = ending_tildes;
            }
            if let Ok(cmp_itemkind_reverse) = opts.get::<bool>("cmp_itemkind_reverse") {
                config.cmp_itemkind_reverse = cmp_itemkind_reverse;
            }

            // Update colors and highlights dictionaries if present
            if let Ok(colors) = opts.get::<Dictionary>("colors") {
                config.colors = colors;
            }
            if let Ok(highlights) = opts.get::<Dictionary>("highlights") {
                config.highlights = highlights;
            }

            // Save the updated config
            set_config(config.clone())?;
        }

        // Set up toggle key if configured
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
