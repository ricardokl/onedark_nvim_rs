use nvim_oxi::{
    api::{self, opts::*, types::*},
    Dictionary, Function, Object,
};

mod highlights;
mod terminal;

#[nvim_oxi::plugin]
fn onedark() -> nvim_oxi::Result<Dictionary> {
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
    let default_config = Dictionary::from_iter([
        ("style", "dark".into()),
        ("toggle_style_list", styles_list.clone().into()),
        ("toggle_style_index", 0.into()),
        ("transparent", false.into()),
        ("term_colors", true.into()),
        ("ending_tildes", false.into()),
        ("cmp_itemkind_reverse", false.into()),
        ("loaded", true.into()),
        (
            "code_style",
            Dictionary::from_iter([
                ("comments", "italic".into()),
                ("keywords", "none".into()),
                ("functions", "none".into()),
                ("strings", "none".into()),
                ("variables", "none".into()),
            ]).into(),
        ),
        (
            "lualine",
            Dictionary::from_iter([("transparent", false.into())]).into(),
        ),
        ("colors", Dictionary::new().into()),
        ("highlights", Dictionary::new().into()),
        (
            "diagnostics",
            Dictionary::from_iter([
                ("darker", true.into()),
                ("undercurl", true.into()),
                ("background", true.into()),
            ]).into(),
        ),
    ]);

    // Initialize global config if not already set
    let g_onedark_config = api::get_var::<Option<Dictionary>>("onedark_config").unwrap_or(None);
    
    if g_onedark_config.is_none() {
        api::set_var("onedark_config", default_config.clone())?;
    } else {
        // Merge with existing config
        let mut config = g_onedark_config.unwrap();
        for (key, value) in default_config.iter() {
            if !config.contains_key(key) {
                config.insert(key.clone(), value.clone());
            }
        }
        api::set_var("onedark_config", config)?;
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
        let config = api::get_var::<Dictionary>("onedark_config")?;
        
        if background == "light" {
            set_option("style", "light".into())?;
        } else if config.get::<String>("style")? == "light" {
            set_option("style", "light".into())?;
        }
        
        // Call setup functions from other modules
        highlights::setup()?;
        terminal::setup()?;
        
        Ok(())
    });

    // Create toggle function
    let toggle = Function::from_fn(|()| -> nvim_oxi::Result<()> {
        let mut config = api::get_var::<Dictionary>("onedark_config")?;
        let index = config.get::<i64>("toggle_style_index")? + 1;
        let toggle_style_list = config.get::<Vec<String>>("toggle_style_list")?;
        
        let new_index = if index as usize > toggle_style_list.len() { 1 } else { index };
        let new_style = &toggle_style_list[new_index as usize - 1];
        
        set_option("style", new_style.clone().into())?;
        set_option("toggle_style_index", new_index.into())?;
        
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
        let mut config = api::get_var::<Dictionary>("onedark_config")?;
        
        if let Some(opts) = opts {
            // Merge options
            for (key, value) in opts.iter() {
                if key == "toggle_style_list" {
                    // This table cannot be extended, it has to be replaced
                    config.insert(key.clone(), value.clone());
                } else if key == "code_style" || key == "lualine" || key == "diagnostics" {
                    // Deep merge for nested dictionaries
                    if let (Ok(mut existing), Ok(new)) = (
                        config.get::<Dictionary>(key),
                        value.clone().try_into::<Dictionary>()
                    ) {
                        for (sub_key, sub_value) in new.iter() {
                            existing.insert(sub_key.clone(), sub_value.clone());
                        }
                        config.insert(key.clone(), existing.into());
                    }
                } else {
                    config.insert(key.clone(), value.clone());
                }
            }
            
            api::set_var("onedark_config", config.clone())?;
        }
        
        // Set up toggle key if configured
        if let Ok(toggle_key) = config.get::<String>("toggle_style_key") {
            if !toggle_key.is_empty() {
                let opts = SetKeymapOpts::builder()
                    .noremap(true)
                    .silent(true)
                    .build();
                
                api::set_keymap(
                    Mode::Normal,
                    &toggle_key,
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

    // Helper function to set options in the config
    fn set_option(opt: &str, value: Object) -> nvim_oxi::Result<()> {
        let mut cfg = api::get_var::<Dictionary>("onedark_config")?;
        cfg.insert(opt.to_string(), value);
        api::set_var("onedark_config", cfg)?;
        Ok(())
    }

    // Return the plugin API
    Ok(Dictionary::from_iter([
        ("colorscheme", colorscheme.into()),
        ("toggle", toggle.into()),
        ("setup", setup.into()),
        ("load", load.into()),
        ("styles_list", styles_list.into()),
    ]))
}
