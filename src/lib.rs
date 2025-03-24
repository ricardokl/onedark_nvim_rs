use nvim_oxi::{
    api::{self, err_writeln, opts::SetKeymapOpts, types::Mode, Error::Other},
    conversion::FromObject,
    Dictionary, Function, Object,
};

mod config;
mod highlights;
mod palette;
mod terminal;
mod util;

use crate::config::{OneDarkConfig, OneDarkStyle, GLOBAL_CONFIG};

pub fn toggle_fn() -> nvim_oxi::Result<()> {
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

    crate::colorscheme_fn()?;
    Ok(())
}

pub fn setup_fn(opts: Option<Object>) -> nvim_oxi::Result<()> {
    {
        if let Some(obj) = opts {
            //api::set_var("onedark_nvim_rs_config", OneDarkConfig::from_object(obj)?)?;
            *GLOBAL_CONFIG.write().map_err(|e| Other(format!("{}", e)))? =
                OneDarkConfig::from_object(obj)?;
        }
    }

    {
        if let Some(key) = &GLOBAL_CONFIG
            .read()
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
    }

    Ok(())
}

pub fn colorscheme_fn() -> nvim_oxi::Result<()> {
    api::command("hi clear")?;

    if api::get_var::<u8>("syntax_on")? == 1 {
        api::command("syntax reset")?;
    }

    api::set_option_value("termguicolors", true, &Default::default())?;
    api::set_var("colors_name", "onedark_nvim_rs")?;

    // Call setup functions from other modules
    crate::highlights::setup()?;
    crate::terminal::setup()?;
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
