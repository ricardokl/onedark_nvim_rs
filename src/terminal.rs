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

    api::set_var("terminal_color_0", c.black)?;
    api::set_var("terminal_color_1", c.red.clone())?;
    api::set_var("terminal_color_2", c.green.clone())?;
    api::set_var("terminal_color_3", c.yellow.clone())?;
    api::set_var("terminal_color_4", c.blue.clone())?;
    api::set_var("terminal_color_5", c.purple.clone())?;
    api::set_var("terminal_color_6", c.cyan.clone())?;
    api::set_var("terminal_color_7", c.fg.clone())?;
    api::set_var("terminal_color_8", c.grey)?;
    api::set_var("terminal_color_9", c.red)?;
    api::set_var("terminal_color_10", c.green)?;
    api::set_var("terminal_color_11", c.yellow)?;
    api::set_var("terminal_color_12", c.blue)?;
    api::set_var("terminal_color_13", c.purple)?;
    api::set_var("terminal_color_14", c.cyan)?;
    api::set_var("terminal_color_15", c.fg)?;

    Ok(())
}
