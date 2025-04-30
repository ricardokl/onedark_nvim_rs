use error::OneDarkError;
use mlua::prelude::*;
use nvim_oxi::api::{self, opts::SetKeymapOpts, types::Mode};

mod config;
mod error;
mod highlights;
mod palette;
mod terminal;
mod util;

use crate::config::{OneDarkConfig, OneDarkStyle, GLOBAL_CONFIG};

pub fn toggle_fn(_: &Lua, _: ()) -> LuaResult<()> {
    {
        let mut config = GLOBAL_CONFIG.write().map_err(Into::<OneDarkError>::into)?;
        config.toggle_style_index =
            (config.toggle_style_index + 1) % config.toggle_style_list.len() as u8;
        config.style = config.toggle_style_list[config.toggle_style_index as usize];
        api::notify(
            &format!("New coloscheme style: {:?}", config.style),
            api::types::LogLevel::Info,
            &Default::default(),
        )
        .map_err(Into::<OneDarkError>::into)?;
        if config.style == OneDarkStyle::Light {
            api::set_option_value("background", "light", &Default::default())
                .map_err(Into::<OneDarkError>::into)?;
        } else {
            api::set_option_value("background", "dark", &Default::default())
                .map_err(Into::<OneDarkError>::into)?;
        };
    }

    // crate::colorscheme_fn(&Lua::new())?;
    Ok(())
}

pub fn setup_fn(_: &Lua, opts: Option<OneDarkConfig<'static>>) -> LuaResult<()> {
    {
        if let Some(obj) = opts {
            *GLOBAL_CONFIG.write().map_err(Into::<OneDarkError>::into)? = obj.clone();
        }
    }

    {
        if let Some(key) = &GLOBAL_CONFIG
            .read()
            .map_err(Into::<OneDarkError>::into)?
            .toggle_style_key
        {
            api::set_keymap(
                Mode::Normal,
                key,
                "<cmd>lua require(\"onedark_nvim_rs\").toggle()<cr>",
                &SetKeymapOpts::builder().silent(true).noremap(true).build(),
            )
            .map_err(Into::<OneDarkError>::into)?;
        }
    }

    Ok(())
}

pub fn colorscheme_fn(_: &Lua, _: ()) -> LuaResult<()> {
    api::command("hi clear").map_err(Into::<OneDarkError>::into)?;

    if api::get_var::<u8>("syntax_on").map_err(Into::<OneDarkError>::into)? == 1 {
        api::command("syntax reset").map_err(Into::<OneDarkError>::into)?;
    }

    api::set_option_value("termguicolors", true, &Default::default())
        .map_err(Into::<OneDarkError>::into)?;
    api::set_var("colors_name", "onedark_nvim_rs").map_err(Into::<OneDarkError>::into)?;

    // Call setup functions from other modules
    crate::highlights::setup().map_err(Into::<OneDarkError>::into)?;
    crate::terminal::setup().map_err(Into::<OneDarkError>::into)?;
    Ok(())
}

#[mlua::lua_module]
fn onedark_nvim_rs(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("setup", lua.create_function(setup_fn)?)?;
    exports.set("colorscheme", lua.create_function(colorscheme_fn)?)?;
    exports.set("toggle", lua.create_function(toggle_fn)?)?;
    exports.set("load", lua.create_function(setup_fn)?)?;
    Ok(exports)
}
