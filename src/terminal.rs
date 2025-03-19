use nvim_oxi::{api, Result};

use crate::{get_global_config, palette::ColorPalette};

pub fn setup() -> Result<()> {
    let cfg = get_global_config::<ColorPalette>().unwrap_or_default();
    let c: ColorPalette = cfg.colors.unwrap_or_default();
    let set_term_colors: bool = cfg.term_colors;

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
