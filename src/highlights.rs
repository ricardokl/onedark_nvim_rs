use nvim_oxi::{
    api::{self, opts::SetHighlightOpts},
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    get_global_config,
    palette::{ColorPalette, ConfigColorPalette},
    util,
};

// Highlight group structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighlightGroup {
    pub fg: String,
    pub bg: String,
    pub sp: String,
    pub fmt: String,
}

impl Default for HighlightGroup {
    fn default() -> Self {
        HighlightGroup {
            fg: "none".into(),
            bg: "none".into(),
            sp: "none".into(),
            fmt: "none".into(),
        }
    }
}

// Highlight collection structure
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Highlights {
    pub common: HashMap<String, HighlightGroup>,
    pub syntax: HashMap<String, HighlightGroup>,
    pub treesitter: HashMap<String, HighlightGroup>,
    pub lsp: Option<HashMap<String, HighlightGroup>>,
    pub plugins: HashMap<String, HashMap<String, HighlightGroup>>,
    pub langs: HashMap<String, HashMap<String, HighlightGroup>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ConfigHighlights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<HashMap<String, HighlightGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<HashMap<String, HighlightGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treesitter: Option<HashMap<String, HighlightGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsp: Option<HashMap<String, HighlightGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<HashMap<String, HashMap<String, HighlightGroup>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langs: Option<HashMap<String, HashMap<String, HighlightGroup>>>,
}

impl Default for Highlights {
    fn default() -> Self {
        let config = get_global_config::<ColorPalette>().unwrap_or_default();
        let palette = config.colors;

        let mut hl = Highlights {
            common: HashMap::new(),
            syntax: HashMap::new(),
            treesitter: HashMap::new(),
            lsp: None,
            plugins: HashMap::new(),
            langs: HashMap::new(),
        };

        // Common highlights
        hl.common = HashMap::from([
            (
                "Normal".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg0.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "Terminal".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg0.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "EndOfBuffer".to_string(),
                HighlightGroup {
                    fg: if config.ending_tildes {
                        palette.bg2.clone()
                    } else {
                        palette.bg0.clone()
                    },
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg0.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "FoldColumn".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg1.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "Folded".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg1.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "SignColumn".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    bg: if config.transparent {
                        "none".to_string()
                    } else {
                        palette.bg0.clone()
                    },
                    ..Default::default()
                },
            ),
            (
                "ToolbarLine".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    ..Default::default()
                },
            ),
            (
                "Cursor".to_string(),
                HighlightGroup {
                    fmt: "reverse".to_string(),
                    ..Default::default()
                },
            ),
            (
                "vCursor".to_string(),
                HighlightGroup {
                    fmt: "reverse".to_string(),
                    ..Default::default()
                },
            ),
            (
                "iCursor".to_string(),
                HighlightGroup {
                    fmt: "reverse".to_string(),
                    ..Default::default()
                },
            ),
            (
                "lCursor".to_string(),
                HighlightGroup {
                    fmt: "reverse".to_string(),
                    ..Default::default()
                },
            ),
            (
                "CursorIM".to_string(),
                HighlightGroup {
                    fmt: "reverse".to_string(),
                    ..Default::default()
                },
            ),
            (
                "CursorColumn".to_string(),
                HighlightGroup {
                    bg: palette.bg1.clone(),
                    ..Default::default()
                },
            ),
            (
                "CursorLine".to_string(),
                HighlightGroup {
                    bg: palette.bg1.clone(),
                    ..Default::default()
                },
            ),
            (
                "ColorColumn".to_string(),
                HighlightGroup {
                    bg: palette.bg1.clone(),
                    ..Default::default()
                },
            ),
            (
                "CursorLineNr".to_string(),
                HighlightGroup {
                    fg: palette.fg.clone(),
                    ..Default::default()
                },
            ),
            (
                "LineNr".to_string(),
                HighlightGroup {
                    fg: palette.grey.clone(),
                    ..Default::default()
                },
            ),
            // Add more common highlights as needed
        ]);

        // Syntax highlights
        hl.syntax = HashMap::from([
            (
                "String".to_string(),
                HighlightGroup {
                    fg: palette.green.clone(),
                    fmt: config.code_style.strings.clone().into(),
                    ..Default::default()
                },
            ),
            (
                "Character".to_string(),
                HighlightGroup {
                    fg: palette.orange.clone(),
                    ..Default::default()
                },
            ),
            (
                "Number".to_string(),
                HighlightGroup {
                    fg: palette.orange.clone(),
                    ..Default::default()
                },
            ),
            (
                "Float".to_string(),
                HighlightGroup {
                    fg: palette.orange.clone(),
                    ..Default::default()
                },
            ),
            (
                "Boolean".to_string(),
                HighlightGroup {
                    fg: palette.orange.clone(),
                    ..Default::default()
                },
            ),
            (
                "Type".to_string(),
                HighlightGroup {
                    fg: palette.yellow.clone(),
                    ..Default::default()
                },
            ),
            // Add more syntax highlights as needed
        ]);

        // TreeSitter highlights
        hl.treesitter = HashMap::new();

        // Plugin highlights
        let mut lsp_plugin = HashMap::new();

        // Diagnostics colors
        let diagnostics_error_color = if config.diagnostics.darker {
            palette.dark_red.clone()
        } else {
            palette.red.clone()
        };

        let diagnostics_hint_color = if config.diagnostics.darker {
            palette.dark_purple.clone()
        } else {
            palette.purple.clone()
        };

        let diagnostics_warn_color = if config.diagnostics.darker {
            palette.dark_yellow.clone()
        } else {
            palette.yellow.clone()
        };

        let diagnostics_info_color = if config.diagnostics.darker {
            palette.dark_cyan.clone()
        } else {
            palette.cyan.clone()
        };

        // LSP plugin highlights
        lsp_plugin.insert(
            "DiagnosticError".to_string(),
            HighlightGroup {
                fg: palette.red.clone(),
                ..Default::default()
            },
        );

        lsp_plugin.insert(
            "DiagnosticHint".to_string(),
            HighlightGroup {
                fg: palette.purple.clone(),
                ..Default::default()
            },
        );

        lsp_plugin.insert(
            "DiagnosticInfo".to_string(),
            HighlightGroup {
                fg: palette.cyan.clone(),
                ..Default::default()
            },
        );

        lsp_plugin.insert(
            "DiagnosticWarn".to_string(),
            HighlightGroup {
                fg: palette.yellow.clone(),
                ..Default::default()
            },
        );

        // Virtual text diagnostics
        lsp_plugin.insert(
            "DiagnosticVirtualTextError".to_string(),
            HighlightGroup {
                fg: diagnostics_error_color.clone(),
                bg: if config.diagnostics.background {
                    match util::darken(&diagnostics_error_color, 0.1, Some(&palette.bg0)) {
                        Ok(color) => color,
                        Err(_) => "none".into(),
                    }
                } else {
                    "none".into()
                },
                ..Default::default()
            },
        );

        // Add more plugin highlights as needed
        hl.plugins.insert("lsp".to_string(), lsp_plugin);

        // Language-specific highlights
        let mut c_lang = HashMap::new();
        c_lang.insert(
            "cInclude".to_string(),
            HighlightGroup {
                fg: palette.blue.clone(),
                ..Default::default()
            },
        );

        c_lang.insert(
            "cStorageClass".to_string(),
            HighlightGroup {
                fg: palette.purple.clone(),
                ..Default::default()
            },
        );

        // Add more C language highlights as needed
        hl.langs.insert("c".to_string(), c_lang);

        hl
    }
}

impl Highlights {
    pub fn new(palette: &ColorPalette, config: &ColorPalette) -> Self {
        let mut hl = Highlights::default();
        hl
    }
}

// Apply vim highlights
fn vim_highlights(highlights: &HashMap<String, HighlightGroup>) -> Result<()> {
    for (group_name, group_settings) in highlights {
        let opts = SetHighlightOpts::builder()
            .foreground(&group_settings.fg.clone())
            .background(&group_settings.bg.clone())
            .special(&group_settings.sp.clone())
            .italic(group_settings.fmt == "italic")
            .bold(group_settings.fmt == "bold")
            .underline(group_settings.fmt == "underline")
            .undercurl(group_settings.fmt == "undercurl")
            .reverse(group_settings.fmt == "reverse")
            .build();

        api::set_hl(0, group_name, &opts)?;
    }
    Ok(())
}

// Setup function
pub fn setup() -> Result<()> {
    let config = get_global_config::<ColorPalette>().unwrap_or_default();
    let palette = config.colors;

    // Initialize highlight groups
    let hl = Highlights::default();

    // Apply all highlights
    vim_highlights(&hl.common)?;
    vim_highlights(&hl.syntax)?;
    vim_highlights(&hl.treesitter)?;

    if let Some(lsp) = &hl.lsp {
        vim_highlights(lsp)?;
    }

    for (_, group) in &hl.langs {
        vim_highlights(group)?;
    }

    for (_, group) in &hl.plugins {
        vim_highlights(group)?;
    }

    // Apply user-defined highlights
    if let Some(config) = get_global_config::<ConfigColorPalette>() {
        if let Some(highlights) = config.highlights {
            for (group_name, group_settings) in highlights {
                if let Ok(settings) = group_settings.as_dictionary() {
                    let mut opts = SetHighlightOpts::builder();

                    if let Some(fg) = settings.get("fg").and_then(|f| f.as_str().ok()) {
                        let color = if fg.starts_with('$') {
                            let name = &fg[1..];
                            match name {
                                "fg" => Some(palette.fg.clone()),
                                "bg0" => Some(palette.bg0.clone()),
                                "red" => Some(palette.red.clone()),
                                // Add more color mappings as needed
                                _ => {
                                    eprintln!("onedark.nvim: unknown color \"{}\"", name);
                                    None
                                }
                            }
                        } else {
                            Some(fg.to_string())
                        };

                        if let Some(color) = color {
                            opts = opts.foreground(Some(color));
                        }
                    }

                    // Similar handling for bg, sp, and fmt

                    api::set_hl(0, group_name, &opts.build())?;
                }
            }
        }
    }

    Ok(())
}
