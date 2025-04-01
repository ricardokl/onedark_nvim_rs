use nvim_oxi::{api, api::Error::Other, Result};

use crate::{
    palette::{merge_palletes, ColorPalette},
    GLOBAL_CONFIG,
};

pub fn setup() -> Result<()> {
    let c: ColorPalette = merge_palletes();
    let set_term_colors;
    {
        set_term_colors = GLOBAL_CONFIG
            .read()
            .map_err(|e| Other(format!("{}", e)))?
            .term_colors;
    }

    if !set_term_colors {
        return Ok(());
    }

    api::set_var("terminal_color_0", c.black.to_string())?;
    api::set_var("terminal_color_1", c.red.to_string())?;
    api::set_var("terminal_color_2", c.green.to_string())?;
    api::set_var("terminal_color_3", c.yellow.to_string())?;
    api::set_var("terminal_color_4", c.blue.to_string())?;
    api::set_var("terminal_color_5", c.purple.to_string())?;
    api::set_var("terminal_color_6", c.cyan.to_string())?;
    api::set_var("terminal_color_7", c.fg.to_string())?;
    api::set_var("terminal_color_8", c.grey.to_string())?;
    api::set_var("terminal_color_9", c.red.to_string())?;
    api::set_var("terminal_color_10", c.green.to_string())?;
    api::set_var("terminal_color_11", c.yellow.to_string())?;
    api::set_var("terminal_color_12", c.blue.to_string())?;
    api::set_var("terminal_color_13", c.purple.to_string())?;
    api::set_var("terminal_color_14", c.cyan.to_string())?;
    api::set_var("terminal_color_15", c.fg.to_string())?;

    Ok(())
}
