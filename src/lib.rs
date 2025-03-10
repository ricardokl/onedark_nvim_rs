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

// Function to get global config or return None if it doesn't exist
fn get_global_config() -> Option<Dictionary> {
    match api::get_var::<Option<Dictionary>>("onedark_config").unwrap_or(None) {
        Some(dict) => Some(dict),
        None => None,
    }
}

// Functions to extract default values from the global config
fn get_style_from_global() -> OneDarkStyle {
    if let Some(config) = get_global_config() {
        if let Ok(style_str) = config.get::<String>("style") {
            if let Ok(style) = OneDarkStyle::from_str(&style_str) {
                return style;
            }
        }
    }
    default_style()
}

fn get_toggle_style_list_from_global() -> Vec<OneDarkStyle> {
    if let Some(config) = get_global_config() {
        if let Ok(styles) = config.get::<Vec<String>>("toggle_style_list") {
            let mut result = Vec::new();
            for style_str in styles {
                if let Ok(style) = OneDarkStyle::from_str(&style_str) {
                    result.push(style);
                }
            }
            if !result.is_empty() {
                return result;
            }
        }
    }
    default_toggle_style_list()
}

fn get_toggle_style_index_from_global() -> i64 {
    if let Some(config) = get_global_config() {
        if let Ok(index) = config.get::<i64>("toggle_style_index") {
            return index;
        }
    }
    0
}

fn get_toggle_style_key_from_global() -> Option<String> {
    if let Some(config) = get_global_config() {
        if let Ok(key) = config.get::<String>("toggle_style_key") {
            return Some(key);
        }
    }
    None
}

fn get_transparent_from_global() -> bool {
    if let Some(config) = get_global_config() {
        if let Ok(transparent) = config.get::<bool>("transparent") {
            return transparent;
        }
    }
    false
}

fn get_term_colors_from_global() -> bool {
    if let Some(config) = get_global_config() {
        if let Ok(term_colors) = config.get::<bool>("term_colors") {
            return term_colors;
        }
    }
    default_true()
}

fn get_ending_tildes_from_global() -> bool {
    if let Some(config) = get_global_config() {
        if let Ok(ending_tildes) = config.get::<bool>("ending_tildes") {
            return ending_tildes;
        }
    }
    false
}

fn get_cmp_itemkind_reverse_from_global() -> bool {
    if let Some(config) = get_global_config() {
        if let Ok(cmp_itemkind_reverse) = config.get::<bool>("cmp_itemkind_reverse") {
            return cmp_itemkind_reverse;
        }
    }
    false
}

fn get_loaded_from_global() -> bool {
    if let Some(config) = get_global_config() {
        if let Ok(loaded) = config.get::<bool>("loaded") {
            return loaded;
        }
    }
    default_true()
}

fn get_code_style_from_global() -> CodeStyle {
    if let Some(config) = get_global_config() {
        if let Ok(code_style_dict) = config.get::<Dictionary>("code_style") {
            let mut code_style = CodeStyle::default();

            if let Ok(comments) = code_style_dict.get::<String>("comments") {
                code_style.comments = comments;
            }
            if let Ok(keywords) = code_style_dict.get::<String>("keywords") {
                code_style.keywords = keywords;
            }
            if let Ok(functions) = code_style_dict.get::<String>("functions") {
                code_style.functions = functions;
            }
            if let Ok(strings) = code_style_dict.get::<String>("strings") {
                code_style.strings = strings;
            }
            if let Ok(variables) = code_style_dict.get::<String>("variables") {
                code_style.variables = variables;
            }

            return code_style;
        }
    }
    CodeStyle::default()
}

fn get_lualine_from_global() -> LualineConfig {
    if let Some(config) = get_global_config() {
        if let Ok(lualine_dict) = config.get::<Dictionary>("lualine") {
            let mut lualine = LualineConfig::default();

            if let Ok(transparent) = lualine_dict.get::<bool>("transparent") {
                lualine.transparent = transparent;
            }

            return lualine;
        }
    }
    LualineConfig::default()
}

fn get_diagnostics_from_global() -> DiagnosticsConfig {
    if let Some(config) = get_global_config() {
        if let Ok(diagnostics_dict) = config.get::<Dictionary>("diagnostics") {
            let mut diagnostics = DiagnosticsConfig::default();

            if let Ok(darker) = diagnostics_dict.get::<bool>("darker") {
                diagnostics.darker = darker;
            }
            if let Ok(undercurl) = diagnostics_dict.get::<bool>("undercurl") {
                diagnostics.undercurl = undercurl;
            }
            if let Ok(background) = diagnostics_dict.get::<bool>("background") {
                diagnostics.background = background;
            }

            return diagnostics;
        }
    }
    DiagnosticsConfig::default()
}

fn get_colors_from_global() -> ColorsConfig {
    if let Some(config) = get_global_config() {
        if let Ok(_colors_dict) = config.get::<Dictionary>("colors") {
            // When ColorsConfig has actual fields, extract them here
            // For now, just return default
        }
    }
    ColorsConfig::default()
}

fn get_highlights_from_global() -> HighlightsConfig {
    if let Some(config) = get_global_config() {
        if let Ok(_highlights_dict) = config.get::<Dictionary>("highlights") {
            // When HighlightsConfig has actual fields, extract them here
            // For now, just return default
        }
    }
    HighlightsConfig::default()
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
    #[serde(default = "get_comments_style_from_global")]
    comments: String,

    #[serde(default = "get_keywords_style_from_global")]
    keywords: String,

    #[serde(default = "get_functions_style_from_global")]
    functions: String,

    #[serde(default = "get_strings_style_from_global")]
    strings: String,

    #[serde(default = "get_variables_style_from_global")]
    variables: String,
}

fn get_comments_style_from_global() -> String {
    if let Some(config) = get_global_config() {
        if let Ok(code_style) = config.get::<Dictionary>("code_style") {
            if let Ok(comments) = code_style.get::<String>("comments") {
                return comments;
            }
        }
    }
    "italic".into()
}

fn get_keywords_style_from_global() -> String {
    if let Some(config) = get_global_config() {
        if let Ok(code_style) = config.get::<Dictionary>("code_style") {
            if let Ok(keywords) = code_style.get::<String>("keywords") {
                return keywords;
            }
        }
    }
    "none".into()
}

fn get_functions_style_from_global() -> String {
    if let Some(config) = get_global_config() {
        if let Ok(code_style) = config.get::<Dictionary>("code_style") {
            if let Ok(functions) = code_style.get::<String>("functions") {
                return functions;
            }
        }
    }
    "none".into()
}

fn get_strings_style_from_global() -> String {
    if let Some(config) = get_global_config() {
        if let Ok(code_style) = config.get::<Dictionary>("code_style") {
            if let Ok(strings) = code_style.get::<String>("strings") {
                return strings;
            }
        }
    }
    "none".into()
}

fn get_variables_style_from_global() -> String {
    if let Some(config) = get_global_config() {
        if let Ok(code_style) = config.get::<Dictionary>("code_style") {
            if let Ok(variables) = code_style.get::<String>("variables") {
                return variables;
            }
        }
    }
    "none".into()
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

#[derive(Clone, Default, Serialize, Deserialize)]
struct ColorsConfig {
    // This will be populated with actual color fields later
    // For now, it's just a placeholder
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct HighlightsConfig {
    // This will be populated with actual highlight fields later
    // For now, it's just a placeholder
}

#[derive(Clone, Serialize, Deserialize)]
struct OneDarkConfig {
    #[serde(with = "style_string_serializer", default = "get_style_from_global")]
    style: OneDarkStyle,

    #[serde(
        with = "style_vec_serializer",
        default = "get_toggle_style_list_from_global"
    )]
    toggle_style_list: Vec<OneDarkStyle>,

    #[serde(default = "get_toggle_style_index_from_global")]
    toggle_style_index: i64,

    #[serde(default = "get_toggle_style_key_from_global")]
    toggle_style_key: Option<String>,

    #[serde(default = "get_transparent_from_global")]
    transparent: bool,

    #[serde(default = "get_term_colors_from_global")]
    term_colors: bool,

    #[serde(default = "get_ending_tildes_from_global")]
    ending_tildes: bool,

    #[serde(default = "get_cmp_itemkind_reverse_from_global")]
    cmp_itemkind_reverse: bool,

    #[serde(default = "get_loaded_from_global")]
    loaded: bool,

    #[serde(default = "get_code_style_from_global")]
    code_style: CodeStyle,

    #[serde(default = "get_lualine_from_global")]
    lualine: LualineConfig,

    #[serde(default = "get_colors_from_global")]
    colors: ColorsConfig,

    #[serde(default = "get_highlights_from_global")]
    highlights: HighlightsConfig,

    #[serde(default = "get_diagnostics_from_global")]
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
        style: get_style_from_global(),
        toggle_style_list: get_toggle_style_list_from_global(),
        toggle_style_index: get_toggle_style_index_from_global(),
        toggle_style_key: get_toggle_style_key_from_global(),
        transparent: get_transparent_from_global(),
        term_colors: get_term_colors_from_global(),
        ending_tildes: get_ending_tildes_from_global(),
        cmp_itemkind_reverse: get_cmp_itemkind_reverse_from_global(),
        loaded: get_loaded_from_global(),
        code_style: get_code_style_from_global(),
        lualine: get_lualine_from_global(),
        colors: get_colors_from_global(),
        highlights: get_highlights_from_global(),
        diagnostics: get_diagnostics_from_global(),
    };

    // Convert to Dictionary for Neovim API
    let default_config_dict = default_config.to_object()?.try_into::<Dictionary>()?;

    // Initialize global config if not already set
    if api::get_var::<Option<Dictionary>>("onedark_config")
        .unwrap_or(None)
        .is_none()
    {
        api::set_var("onedark_config", default_config_dict)?;
    }

    // Helper function to get config
    fn get_config() -> nvim_oxi::Result<OneDarkConfig> {
        let config_dict = match api::get_var::<Option<Dictionary>>("onedark_config").unwrap_or(None)
        {
            Some(dict) => dict,
            None => Dictionary::new(),
        };
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
    let setup = Function::from_fn(|config: OneDarkConfig| -> nvim_oxi::Result<()> {
        // Get current config
        let current_config = get_config()?;

        // Create a merged config - start with current and override with provided values
        let merged_config = OneDarkConfig {
            // Only override fields that are explicitly set in config
            style: config.style,
            toggle_style_list: config.toggle_style_list,
            toggle_style_index: current_config.toggle_style_index,
            toggle_style_key: config.toggle_style_key.or(current_config.toggle_style_key),
            transparent: config.transparent,
            term_colors: config.term_colors,
            ending_tildes: config.ending_tildes,
            cmp_itemkind_reverse: config.cmp_itemkind_reverse,
            loaded: true, // Always set loaded to true

            // For nested structures, use the provided ones
            code_style: config.code_style,
            lualine: config.lualine,
            diagnostics: config.diagnostics,

            // For the new struct types, use the provided ones
            colors: config.colors,
            highlights: config.highlights,
        };

        // Save the merged config
        set_config(merged_config)?;

        // Set up toggle key if configured
        if let Some(toggle_key) = &merged_config.toggle_style_key {
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
