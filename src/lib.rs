use nvim_oxi::{
    Dictionary, Function, Object,
    api::{self, opts::*, types::*},
    conversion::{Error as ConversionError, FromObject, ToObject},
    serde::{Deserializer, Serializer},
};
use serde::{Deserialize, Serialize};

mod highlights;
mod terminal;

#[derive(Clone, Serialize, Deserialize)]
struct OneDarkConfig {
    style: String,
    toggle_style_list: Vec<String>,
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
    // Define styles list
    let styles_list = vec![
        "dark".to_string(),
        "darker".to_string(),
        "cool".to_string(),
        "deep".to_string(),
        "warm".to_string(),
        "warmer".to_string(),
        "light".to_string(),
    ];

    // Set up default config
    let default_config = OneDarkConfig {
        style: "dark".to_string(),
        toggle_style_list: styles_list.clone(),
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

        if background == "light" || config.style == "light" {
            config.style = "light".to_string();
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
        let new_index = if index as usize > config.toggle_style_list.len() {
            1
        } else {
            index
        };
        let new_style = config.toggle_style_list[new_index as usize - 1].clone();

        config.style = new_style.clone();
        config.toggle_style_index = new_index;
        set_config(config)?;

        if new_style == "light" {
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
            // Handle toggle_style_list separately if present
            if let Ok(new_toggle_list) = opts.get::<Vec<String>>("toggle_style_list") {
                config.toggle_style_list = new_toggle_list;
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
            if let Ok(style) = opts.get::<String>("style") {
                config.style = style;
            }
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
