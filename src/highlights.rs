use nvim_oxi::{
    api::{self, opts::SetHighlightOpts},
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{get_global_config, util};

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

impl HighlightGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, fg: &str) -> Self {
        self.fg = fg.to_string();
        self
    }

    pub fn bg(mut self, bg: &str) -> Self {
        self.bg = bg.to_string();
        self
    }

    pub fn sp(mut self, sp: &str) -> Self {
        self.sp = sp.to_string();
        self
    }

    pub fn fmt(mut self, fmt: &str) -> Self {
        self.fmt = fmt.to_string();
        self
    }
}

// Highlight collection structure
#[derive(Clone, Debug)]
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
    pub common: Option<HashMap<String, HighlightGroup>>,
    pub syntax: Option<HashMap<String, HighlightGroup>>,
    pub treesitter: Option<HashMap<String, HighlightGroup>>,
    pub lsp: Option<HashMap<String, HighlightGroup>>,
    pub plugins: Option<HashMap<String, HashMap<String, HighlightGroup>>>,
    pub langs: Option<HashMap<String, HashMap<String, HighlightGroup>>>,
}

impl Default for Highlights {
    fn default() -> Self {
        let config = get_global_config().unwrap_or_default();
        let palette = crate::palette::merge_palletes();

        let mut hl = Highlights {
            common: HashMap::new(),
            syntax: HashMap::new(),
            treesitter: HashMap::new(),
            lsp: None,
            plugins: HashMap::new(),
            langs: HashMap::new(),
        };

        let common_float_border = HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1);
        let common_normal_float = HighlightGroup::new().fg(&palette.fg).bg(&palette.bg1);
        let common_added = HighlightGroup::new().fg(&palette.green);
        let common_removed = HighlightGroup::new().fg(&palette.red);
        let common_diff_delete = HighlightGroup::new().fg("none").bg(&palette.diff_delete);
        let common_diff_text = HighlightGroup::new().fg("none").bg(&palette.diff_text);
        let common_diff_add = HighlightGroup::new().fg("none").bg(&palette.diff_add);
        let common_diff_change = HighlightGroup::new().fg("none").bg(&palette.diff_change);
        let common_inc_search = HighlightGroup::new().fg(&palette.bg0).bg(&palette.orange);
        let common_directory = HighlightGroup::new().fg(&palette.blue);

        // Common highlights
        hl.common = HashMap::from([
            (
                "Normal".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "Terminal".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "EndOfBuffer".to_string(),
                HighlightGroup::new()
                    .fg(if config.ending_tildes {
                        &palette.bg2
                    } else {
                        &palette.bg0
                    })
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "FoldColumn".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg1
                    }),
            ),
            (
                "Folded".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg1
                    }),
            ),
            (
                "SignColumn".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "ToolbarLine".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            ("Cursor".to_string(), HighlightGroup::new().fmt("reverse")),
            ("vCursor".to_string(), HighlightGroup::new().fmt("reverse")),
            ("iCursor".to_string(), HighlightGroup::new().fmt("reverse")),
            ("lCursor".to_string(), HighlightGroup::new().fmt("reverse")),
            ("CursorIM".to_string(), HighlightGroup::new().fmt("reverse")),
            (
                "CursorColumn".to_string(),
                HighlightGroup::new().bg(&palette.bg1),
            ),
            (
                "CursorLine".to_string(),
                HighlightGroup::new().bg(&palette.bg1),
            ),
            (
                "ColorColumn".to_string(),
                HighlightGroup::new().bg(&palette.bg1),
            ),
            (
                "CursorLineNr".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "LineNr".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "Conceal".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            ("Added".to_string(), common_added.clone()),
            ("Removed".to_string(), common_removed.clone()),
            (
                "Changed".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            ("DiffAdd".to_string(), common_diff_add.clone()),
            ("DiffChange".to_string(), common_diff_change.clone()),
            ("DiffDelete".to_string(), common_diff_delete.clone()),
            ("DiffText".to_string(), common_diff_text.clone()),
            (
                "DiffAdded".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "DiffChanged".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffRemoved".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "DiffDeleted".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "DiffFile".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "DiffIndexLine".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            ("Directory".to_string(), common_directory.clone()),
            (
                "ErrorMsg".to_string(),
                HighlightGroup::new().fg(&palette.red).fmt("bold"),
            ),
            (
                "WarningMsg".to_string(),
                HighlightGroup::new().fg(&palette.yellow).fmt("bold"),
            ),
            (
                "MoreMsg".to_string(),
                HighlightGroup::new().fg(&palette.blue).fmt("bold"),
            ),
            (
                "CurSearch".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.orange),
            ),
            ("IncSearch".to_string(), common_inc_search.clone()),
            (
                "Search".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.bg_yellow),
            ),
            (
                "Substitute".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.green),
            ),
            (
                "MatchParen".to_string(),
                HighlightGroup::new().fg("none").bg(&palette.grey),
            ),
            (
                "NonText".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "Whitespace".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "SpecialKey".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "Pmenu".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg1),
            ),
            (
                "PmenuSbar".to_string(),
                HighlightGroup::new().fg("none").bg(&palette.bg1),
            ),
            (
                "PmenuSel".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.bg_blue),
            ),
            (
                "WildMenu".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.blue),
            ),
            (
                "PmenuThumb".to_string(),
                HighlightGroup::new().fg("none").bg(&palette.grey),
            ),
            (
                "Question".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "SpellBad".to_string(),
                HighlightGroup::new()
                    .fg("none")
                    .fmt("undercurl")
                    .sp(&palette.red),
            ),
            (
                "SpellCap".to_string(),
                HighlightGroup::new()
                    .fg("none")
                    .fmt("undercurl")
                    .sp(&palette.yellow),
            ),
            (
                "SpellLocal".to_string(),
                HighlightGroup::new()
                    .fg("none")
                    .fmt("undercurl")
                    .sp(&palette.blue),
            ),
            (
                "SpellRare".to_string(),
                HighlightGroup::new()
                    .fg("none")
                    .fmt("undercurl")
                    .sp(&palette.purple),
            ),
            (
                "StatusLine".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg2),
            ),
            (
                "StatusLineTerm".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg2),
            ),
            (
                "StatusLineNC".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "StatusLineTermNC".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "TabLine".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg1),
            ),
            (
                "TabLineFill".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "TabLineSel".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.fg),
            ),
            (
                "WinSeparator".to_string(),
                HighlightGroup::new().fg(&palette.bg3),
            ),
            ("Visual".to_string(), HighlightGroup::new().bg(&palette.bg3)),
            (
                "VisualNOS".to_string(),
                HighlightGroup::new()
                    .fg("none")
                    .bg(&palette.bg2)
                    .fmt("underline"),
            ),
            (
                "QuickFixLine".to_string(),
                HighlightGroup::new().fg(&palette.blue).fmt("underline"),
            ),
            (
                "Debug".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "debugPC".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.green),
            ),
            (
                "debugBreakpoint".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.red),
            ),
            (
                "ToolbarButton".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.bg_blue),
            ),
            ("FloatBorder".to_string(), common_float_border.clone()),
            ("NormalFloat".to_string(), common_normal_float.clone()),
        ]);

        let syntax_comment = HighlightGroup::new()
            .fg(&palette.grey)
            .fmt(&config.code_style.comments.clone());
        let syntax_title = HighlightGroup::new().fg(&palette.cyan);
        let syntax_special = HighlightGroup::new().fg(&palette.red);
        let syntax_delimiter = HighlightGroup::new().fg(&palette.light_grey);

        // Syntax highlights
        hl.syntax = HashMap::from([
            (
                "String".to_string(),
                HighlightGroup::new()
                    .fg(&palette.green)
                    .fmt(&config.code_style.strings.clone()),
            ),
            (
                "Character".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "Number".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "Float".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "Boolean".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "Type".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "Structure".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "StorageClass".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "Identifier".to_string(),
                HighlightGroup::new()
                    .fg(&palette.red)
                    .fmt(&config.code_style.variables.clone()),
            ),
            (
                "Constant".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "PreProc".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "PreCondit".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "Include".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "Keyword".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "Define".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "Typedef".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "Exception".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "Conditional".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "Repeat".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "Statement".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            ("Macro".to_string(), HighlightGroup::new().fg(&palette.red)),
            (
                "Error".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "Label".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            ("Special".to_string(), syntax_special.clone()),
            (
                "SpecialChar".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "Function".to_string(),
                HighlightGroup::new()
                    .fg(&palette.blue)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "Operator".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            ("Title".to_string(), syntax_title.clone()),
            ("Tag".to_string(), HighlightGroup::new().fg(&palette.green)),
            ("Delimiter".to_string(), syntax_delimiter.clone()),
            ("Comment".to_string(), syntax_comment.clone()),
            (
                "SpecialComment".to_string(),
                HighlightGroup::new()
                    .fg(&palette.grey)
                    .fmt(&config.code_style.comments.clone()),
            ),
            (
                "Todo".to_string(),
                HighlightGroup::new()
                    .fg(&palette.red)
                    .fmt(&config.code_style.comments.clone()),
            ),
        ]);

        // TreeSitter highlights
        hl.treesitter = HashMap::from([
            (
                "TSAnnotation".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSAttribute".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TSBoolean".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSCharacter".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSComment".to_string(),
                HighlightGroup::new()
                    .fg(&palette.grey)
                    .fmt(&config.code_style.comments.clone()),
            ),
            (
                "TSConditional".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "TSConstant".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSConstBuiltin".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSConstMacro".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSConstructor".to_string(),
                HighlightGroup::new().fg(&palette.yellow).fmt("bold"),
            ),
            ("TSError".to_string(), HighlightGroup::new().fg(&palette.fg)),
            (
                "TSException".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "TSField".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TSFloat".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSFunction".to_string(),
                HighlightGroup::new()
                    .fg(&palette.blue)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "TSFuncBuiltin".to_string(),
                HighlightGroup::new()
                    .fg(&palette.cyan)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "TSFuncMacro".to_string(),
                HighlightGroup::new()
                    .fg(&palette.cyan)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "TSInclude".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "TSKeyword".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "TSKeywordFunction".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "TSKeywordOperator".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "TSLabel".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "TSMethod".to_string(),
                HighlightGroup::new()
                    .fg(&palette.blue)
                    .fmt(&config.code_style.functions.clone()),
            ),
            (
                "TSNamespace".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            ("TSNone".to_string(), HighlightGroup::new().fg(&palette.fg)),
            (
                "TSNumber".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSOperator".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSParameter".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "TSParameterReference".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSProperty".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TSPunctDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.light_grey),
            ),
            (
                "TSPunctBracket".to_string(),
                HighlightGroup::new().fg(&palette.light_grey),
            ),
            (
                "TSPunctSpecial".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "TSRepeat".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords.clone()),
            ),
            (
                "TSString".to_string(),
                HighlightGroup::new()
                    .fg(&palette.green)
                    .fmt(&config.code_style.strings.clone()),
            ),
            (
                "TSStringRegex".to_string(),
                HighlightGroup::new()
                    .fg(&palette.orange)
                    .fmt(&config.code_style.strings.clone()),
            ),
            (
                "TSStringEscape".to_string(),
                HighlightGroup::new()
                    .fg(&palette.red)
                    .fmt(&config.code_style.strings.clone()),
            ),
            (
                "TSSymbol".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TSTag".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "TSTagDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            ("TSText".to_string(), HighlightGroup::new().fg(&palette.fg)),
            (
                "TSStrong".to_string(),
                HighlightGroup::new().fg(&palette.fg).fmt("bold"),
            ),
            (
                "TSEmphasis".to_string(),
                HighlightGroup::new().fg(&palette.fg).fmt("italic"),
            ),
            (
                "TSUnderline".to_string(),
                HighlightGroup::new().fg(&palette.fg).fmt("underline"),
            ),
            (
                "TSStrike".to_string(),
                HighlightGroup::new().fg(&palette.fg).fmt("strikethrough"),
            ),
            (
                "TSTitle".to_string(),
                HighlightGroup::new().fg(&palette.orange).fmt("bold"),
            ),
            (
                "TSLiteral".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "TSURI".to_string(),
                HighlightGroup::new().fg(&palette.cyan).fmt("underline"),
            ),
            ("TSMath".to_string(), HighlightGroup::new().fg(&palette.fg)),
            (
                "TSTextReference".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "TSEnvironment".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSEnvironmentName".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            ("TSNote".to_string(), HighlightGroup::new().fg(&palette.fg)),
            (
                "TSWarning".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSDanger".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "TSType".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "TSTypeBuiltin".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "TSVariable".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .fmt(&config.code_style.variables.clone()),
            ),
            (
                "TSVariableBuiltin".to_string(),
                HighlightGroup::new()
                    .fg(&palette.red)
                    .fmt(&config.code_style.variables.clone()),
            ),
        ]);

        // LSP plugin highlights
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

        let error_bg = if config.diagnostics.background {
            match util::darken(&diagnostics_error_color, 0.1, Some(&palette.bg0)) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let warn_bg = if config.diagnostics.background {
            match util::darken(&diagnostics_warn_color, 0.1, Some(&palette.bg0)) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let info_bg = if config.diagnostics.background {
            match util::darken(&diagnostics_info_color, 0.1, Some(&palette.bg0)) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let hint_bg = if config.diagnostics.background {
            match util::darken(&diagnostics_hint_color, 0.1, Some(&palette.bg0)) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };

        // Underline diagnostics
        let underline_fmt = if config.diagnostics.undercurl {
            "undercurl"
        } else {
            "underline"
        };

        let diagnostic_error = HighlightGroup::new().fg(&palette.red);
        let diagnostic_hint = HighlightGroup::new().fg(&palette.purple);
        let diagnostic_info = HighlightGroup::new().fg(&palette.cyan);
        let diagnostic_warn = HighlightGroup::new().fg(&palette.yellow);
        let diagnostic_virtual_text_error = HighlightGroup::new()
            .fg(&diagnostics_error_color)
            .bg(&error_bg);
        let diagnostic_virtual_text_warn = HighlightGroup::new()
            .fg(&diagnostics_warn_color)
            .bg(&warn_bg);
        let diagnostic_virtual_text_info = HighlightGroup::new()
            .fg(&diagnostics_info_color)
            .bg(&info_bg);
        let diagnostic_virtual_text_hint = HighlightGroup::new()
            .fg(&diagnostics_hint_color)
            .bg(&hint_bg);
        let diagnostic_underline_error = HighlightGroup::new().fmt(underline_fmt).sp(&palette.red);
        let diagnostic_underline_warn =
            HighlightGroup::new().fmt(underline_fmt).sp(&palette.yellow);
        let diagnostic_underline_info = HighlightGroup::new().fmt(underline_fmt).sp(&palette.blue);
        let diagnostic_underline_hint =
            HighlightGroup::new().fmt(underline_fmt).sp(&palette.purple);

        let lsp_plugin = HashMap::from([
            (
                "LspCxxHlGroupEnumConstant".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "LspCxxHlGroupMemberVariable".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "LspCxxHlGroupNamespace".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "LspCxxHlSkippedRegion".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "LspCxxHlSkippedRegionBeginEnd".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            ("DiagnosticError".to_string(), diagnostic_error.clone()),
            ("DiagnosticHint".to_string(), diagnostic_hint.clone()),
            ("DiagnosticInfo".to_string(), diagnostic_info.clone()),
            ("DiagnosticWarn".to_string(), diagnostic_warn.clone()),
            (
                "DiagnosticVirtualTextError".to_string(),
                diagnostic_virtual_text_error.clone(),
            ),
            (
                "DiagnosticVirtualTextWarn".to_string(),
                diagnostic_virtual_text_warn.clone(),
            ),
            (
                "DiagnosticVirtualTextInfo".to_string(),
                diagnostic_virtual_text_info.clone(),
            ),
            (
                "DiagnosticVirtualTextHint".to_string(),
                diagnostic_virtual_text_hint.clone(),
            ),
            (
                "DiagnosticUnderlineInfo".to_string(),
                diagnostic_virtual_text_info.clone(),
            ),
            (
                "DiagnosticUnderlineWarn".to_string(),
                diagnostic_virtual_text_warn.clone(),
            ),
            (
                "LspReferenceText".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "LspReferenceWrite".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "LspReferenceRead".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "LspCodeLens".to_string(),
                HighlightGroup::new()
                    .fg(&palette.grey)
                    .fmt(&config.code_style.comments),
            ),
            (
                "LspCodeLensSeparator".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "LspDiagnosticsDefaultError".to_string(),
                diagnostic_error.clone(),
            ),
            (
                "LspDiagnosticsDefaultHint".to_string(),
                diagnostic_hint.clone(),
            ),
            (
                "LspDiagnosticsDefaultInformation".to_string(),
                diagnostic_info.clone(),
            ),
            (
                "LspDiagnosticsDefaultWarning".to_string(),
                diagnostic_warn.clone(),
            ),
            (
                "LspDiagnosticsUnderlineError".to_string(),
                diagnostic_underline_error,
            ),
            (
                "LspDiagnosticsUnderlineHint".to_string(),
                diagnostic_underline_hint,
            ),
            (
                "LspDiagnosticsUnderlineInformation".to_string(),
                diagnostic_underline_info,
            ),
            (
                "LspDiagnosticsUnderlineWarning".to_string(),
                diagnostic_underline_warn,
            ),
            (
                "LspDiagnosticsVirtualTextError".to_string(),
                diagnostic_virtual_text_error,
            ),
            (
                "LspDiagnosticsVirtualTextWarning".to_string(),
                diagnostic_virtual_text_warn,
            ),
            (
                "LspDiagnosticsVirtualTextInformation".to_string(),
                diagnostic_virtual_text_info,
            ),
            (
                "LspDiagnosticsVirtualTextHint".to_string(),
                diagnostic_virtual_text_hint,
            ),
        ]);

        // Implement plugin highlight tables
        let mut plugins = HashMap::new();
        plugins.insert("lsp".to_string(), lsp_plugin);

        let lsp_kind = HashMap::from([
            ("Default", &palette.purple),
            ("Array", &palette.yellow),
            ("Boolean", &palette.orange),
            ("Class", &palette.yellow),
            ("Color", &palette.green),
            ("Constant", &palette.orange),
            ("Constructor", &palette.blue),
            ("Enum", &palette.purple),
            ("EnumMember", &palette.yellow),
            ("Event", &palette.yellow),
            ("Field", &palette.purple),
            ("File", &palette.blue),
            ("Folder", &palette.orange),
            ("Function", &palette.blue),
            ("Interface", &palette.green),
            ("Key", &palette.cyan),
            ("Keyword", &palette.cyan),
            ("Method", &palette.blue),
            ("Module", &palette.orange),
            ("Namespace", &palette.red),
            ("Null", &palette.grey),
            ("Number", &palette.orange),
            ("Object", &palette.red),
            ("Operator", &palette.red),
            ("Package", &palette.yellow),
            ("Property", &palette.cyan),
            ("Reference", &palette.orange),
            ("Snippet", &palette.red),
            ("String", &palette.green),
            ("Struct", &palette.purple),
            ("Text", &palette.light_grey),
            ("TypeParameter", &palette.red),
            ("Unit", &palette.green),
            ("Value", &palette.orange),
            ("Variable", &palette.purple),
        ]);

        // CMP plugin highlights
        let mut cmp_highlights = HashMap::from([
            (
                "CmpItemAbbr".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "CmpItemAbbrDeprecated".to_string(),
                HighlightGroup::new()
                    .fg(&palette.light_grey)
                    .fmt("strikethrough"),
            ),
            (
                "CmpItemAbbrMatch".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "CmpItemAbbrMatchFuzzy".to_string(),
                HighlightGroup::new().fg(&palette.cyan).fmt("underline"),
            ),
            (
                "CmpItemMenu".to_string(),
                HighlightGroup::new().fg(&palette.light_grey),
            ),
            (
                "CmpItemKind".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(if config.cmp_itemkind_reverse {
                        "reverse"
                    } else {
                        "none"
                    }),
            ),
        ]);

        for (kind, color) in lsp_kind.iter() {
            cmp_highlights.insert(
                format!("CmpItemKind{}", kind),
                HighlightGroup::new().fg(color).fmt(
                    if get_global_config().unwrap_or_default().cmp_itemkind_reverse {
                        "reverse"
                    } else {
                        "none"
                    },
                ),
            );
        }

        plugins.insert("cmp".to_string(), cmp_highlights);

        // WhichKey plugin highlights
        let whichkey_highlights = HashMap::from([
            (
                "WhichKey".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "WhichKeyDesc".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "WhichKeyGroup".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "WhichKeySeparator".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
        ]);

        plugins.insert("whichkey".to_string(), whichkey_highlights);

        // GitGutter plugin highlights
        let gitgutter_highlights = HashMap::from([
            (
                "GitGutterAdd".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "GitGutterChange".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "GitGutterDelete".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
        ]);

        plugins.insert("gitgutter".to_string(), gitgutter_highlights);

        // DiffView plugin highlights
        let diffview_highlights = HashMap::from([
            (
                "DiffviewFilePanelTitle".to_string(),
                HighlightGroup::new().fg(&palette.blue).fmt("bold"),
            ),
            (
                "DiffviewFilePanelCounter".to_string(),
                HighlightGroup::new().fg(&palette.purple).fmt("bold"),
            ),
            (
                "DiffviewFilePanelFileName".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "DiffviewNormal".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "DiffviewCursorLine".to_string(),
                HighlightGroup::new().bg(&palette.bg1),
            ),
            (
                "DiffviewVertSplit".to_string(),
                HighlightGroup::new().fg(&palette.bg3),
            ),
            (
                "DiffviewSignColumn".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "DiffviewStatusLine".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg2),
            ),
            (
                "DiffviewStatusLineNC".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "DiffviewEndOfBuffer".to_string(),
                HighlightGroup::new()
                    .fg(if config.ending_tildes {
                        &palette.bg2
                    } else {
                        &palette.bg0
                    })
                    .bg(if config.transparent {
                        "none"
                    } else {
                        &palette.bg0
                    }),
            ),
            (
                "DiffviewFilePanelRootPath".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "DiffviewFilePanelPath".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "DiffviewFilePanelInsertions".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "DiffviewFilePanelDeletions".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "DiffviewStatusAdded".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "DiffviewStatusUntracked".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusModified".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusRenamed".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusCopied".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusTypeChange".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusUnmerged".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "DiffviewStatusUnknown".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "DiffviewStatusDeleted".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "DiffviewStatusBroken".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
        ]);

        plugins.insert("diffview".to_string(), diffview_highlights);

        // GitSigns plugin highlights
        let gitsigns_highlights = HashMap::from([
            (
                "GitSignsAdd".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "GitSignsAddLn".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "GitSignsAddNr".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "GitSignsChange".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "GitSignsChangeLn".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "GitSignsChangeNr".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "GitSignsDelete".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "GitSignsDeleteLn".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "GitSignsDeleteNr".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
        ]);

        plugins.insert("gitsigns".to_string(), gitsigns_highlights);

        // Telescope plugin highlights
        let telescope_highlights = HashMap::from([
            (
                "TelescopeBorder".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "TelescopePromptBorder".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TelescopeResultsBorder".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TelescopePreviewBorder".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "TelescopeMatching".to_string(),
                HighlightGroup::new().fg(&palette.orange).fmt("bold"),
            ),
            (
                "TelescopePromptPrefix".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "TelescopeSelection".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "TelescopeSelectionCaret".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
        ]);

        plugins.insert("telescope".to_string(), telescope_highlights);

        let mini = HashMap::from([
            (
                "MiniAnimateCursor".to_string(),
                HighlightGroup::new().fmt("reverse,nocombine"),
            ),
            (
                "MiniAnimateNormalFloat".to_string(),
                common_normal_float.clone(),
            ),
            ("MiniClueBorder".to_string(), common_float_border.clone()),
            ("MiniClueDescGroup".to_string(), diagnostic_warn.clone()),
            (
                "MiniClueDescSingle".to_string(),
                common_normal_float.clone(),
            ),
            ("MiniClueNextKey".to_string(), diagnostic_hint.clone()),
            (
                "MiniClueNextKeyWithPostkeys".to_string(),
                diagnostic_error.clone(),
            ),
            ("MiniClueSeparator".to_string(), diagnostic_info.clone()),
            (
                "MiniClueTitle".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "MiniCompletionActiveParameter".to_string(),
                HighlightGroup::new().fmt("underline"),
            ),
            (
                "MiniCursorword".to_string(),
                HighlightGroup::new().fmt("underline"),
            ),
            (
                "MiniCursorwordCurrent".to_string(),
                HighlightGroup::new().fmt("underline"),
            ),
            ("MiniDepsChangeAdded".to_string(), common_added),
            ("MiniDepsChangeRemoved".to_string(), common_removed),
            ("MiniDepsHint".to_string(), diagnostic_hint.clone()),
            ("MiniDepsInfo".to_string(), diagnostic_info.clone()),
            ("MiniDepsMsgBreaking".to_string(), diagnostic_warn.clone()),
            ("MiniDepsPlaceholder".to_string(), syntax_comment.clone()),
            ("MiniDepsTitle".to_string(), syntax_title.clone()),
            ("MiniDepsTitleError".to_string(), common_diff_delete.clone()),
            ("MiniDepsTitleSame".to_string(), common_diff_text.clone()),
            ("MiniDepsTitleUpdate".to_string(), common_diff_add.clone()),
            (
                "MiniDiffSignAdd".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "MiniDiffSignChange".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "MiniDiffSignDelete".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            ("MiniDiffOverAdd".to_string(), common_diff_add),
            ("MiniDiffOverChange".to_string(), common_diff_text),
            ("MiniDiffOverContext".to_string(), common_diff_change),
            ("MiniDiffOverDelete".to_string(), common_diff_delete),
            ("MiniFilesBorder".to_string(), common_float_border.clone()),
            (
                "MiniFilesBorderModified".to_string(),
                diagnostic_warn.clone(),
            ),
            (
                "MiniFilesCursorLine".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            ("MiniFilesDirectory".to_string(), common_directory.clone()),
            (
                "MiniFilesFile".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            ("MiniFilesNormal".to_string(), common_normal_float.clone()),
            (
                "MiniFilesTitle".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "MiniFilesTitleFocused".to_string(),
                HighlightGroup::new().fg(&palette.cyan).fmt("bold"),
            ),
            (
                "MiniHipatternsFixme".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.red)
                    .fmt("bold"),
            ),
            (
                "MiniHipatternsHack".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.yellow)
                    .fmt("bold"),
            ),
            (
                "MiniHipatternsNote".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.cyan)
                    .fmt("bold"),
            ),
            (
                "MiniHipatternsTodo".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.purple)
                    .fmt("bold"),
            ),
            (
                "MiniIconsAzure".to_string(),
                HighlightGroup::new().fg(&palette.bg_blue),
            ),
            (
                "MiniIconsBlue".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "MiniIconsCyan".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "MiniIconsGreen".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "MiniIconsGrey".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "MiniIconsOrange".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "MiniIconsPurple".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "MiniIconsRed".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "MiniIconsYellow".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "MiniIndentscopeSymbol".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "MiniIndentscopePrefix".to_string(),
                HighlightGroup::new().fmt("nocombine"),
            ),
            (
                "MiniJump".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt("underline")
                    .sp(&palette.purple),
            ),
            (
                "MiniJump2dDim".to_string(),
                HighlightGroup::new().fg(&palette.grey).fmt("nocombine"),
            ),
            (
                "MiniJump2dSpot".to_string(),
                HighlightGroup::new().fg(&palette.red).fmt("bold,nocombine"),
            ),
            (
                "MiniJump2dSpotAhead".to_string(),
                HighlightGroup::new()
                    .fg(&palette.cyan)
                    .bg(&palette.bg0)
                    .fmt("nocombine"),
            ),
            (
                "MiniJump2dSpotUnique".to_string(),
                HighlightGroup::new()
                    .fg(&palette.yellow)
                    .fmt("bold,nocombine"),
            ),
            ("MiniMapNormal".to_string(), common_normal_float.clone()),
            ("MiniMapSymbolCount".to_string(), syntax_special.clone()),
            ("MiniMapSymbolLine".to_string(), syntax_title.clone()),
            ("MiniMapSymbolView".to_string(), syntax_delimiter.clone()),
            ("MiniNotifyBorder".to_string(), common_float_border.clone()),
            ("MiniNotifyNormal".to_string(), common_normal_float.clone()),
            (
                "MiniNotifyTitle".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "MiniOperatorsExchangeFrom".to_string(),
                common_inc_search.clone(),
            ),
            ("MiniPickBorder".to_string(), common_float_border),
            ("MiniPickBorderBusy".to_string(), diagnostic_warn),
            (
                "MiniPickBorderText".to_string(),
                HighlightGroup::new().fg(&palette.cyan).fmt("bold"),
            ),
            ("MiniPickIconDirectory".to_string(), common_directory),
            ("MiniPickIconFile".to_string(), common_normal_float.clone()),
            ("MiniPickHeader".to_string(), diagnostic_hint.clone()),
            (
                "MiniPickMatchCurrent".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "MiniPickMatchMarked".to_string(),
                HighlightGroup::new().bg(&palette.diff_text),
            ),
            ("MiniPickMatchRanges".to_string(), diagnostic_hint.clone()),
            ("MiniPickNormal".to_string(), common_normal_float.clone()),
            (
                "MiniPickPreviewLine".to_string(),
                HighlightGroup::new().bg(&palette.bg2),
            ),
            (
                "MiniPickPreviewRegion".to_string(),
                common_inc_search.clone(),
            ),
            ("MiniPickPrompt".to_string(), diagnostic_info),
            (
                "MiniStarterCurrent".to_string(),
                HighlightGroup::new().fmt("nocombine"),
            ),
            (
                "MiniStarterFooter".to_string(),
                HighlightGroup::new().fg(&palette.dark_red).fmt("italic"),
            ),
            (
                "MiniStarterHeader".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "MiniStarterInactive".to_string(),
                HighlightGroup::new()
                    .fg(&palette.grey)
                    .fmt(&config.code_style.comments),
            ),
            (
                "MiniStarterItem".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg0),
            ),
            (
                "MiniStarterItemBullet".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "MiniStarterItemPrefix".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "MiniStarterSection".to_string(),
                HighlightGroup::new().fg(&palette.light_grey),
            ),
            (
                "MiniStarterQuery".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "MiniStatuslineDevinfo".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg2),
            ),
            (
                "MiniStatuslineFileinfo".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg2),
            ),
            (
                "MiniStatuslineFilename".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "MiniStatuslineInactive".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg0),
            ),
            (
                "MiniStatuslineModeCommand".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.yellow)
                    .fmt("bold"),
            ),
            (
                "MiniStatuslineModeInsert".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.blue)
                    .fmt("bold"),
            ),
            (
                "MiniStatuslineModeNormal".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.green)
                    .fmt("bold"),
            ),
            (
                "MiniStatuslineModeOther".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.cyan)
                    .fmt("bold"),
            ),
            (
                "MiniStatuslineModeReplace".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.red)
                    .fmt("bold"),
            ),
            (
                "MiniStatuslineModeVisual".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.purple)
                    .fmt("bold"),
            ),
            (
                "MiniSurround".to_string(),
                HighlightGroup::new().fg(&palette.bg0).bg(&palette.orange),
            ),
            (
                "MiniTablineCurrent".to_string(),
                HighlightGroup::new().fmt("bold"),
            ),
            (
                "MiniTablineFill".to_string(),
                HighlightGroup::new().fg(&palette.grey).bg(&palette.bg1),
            ),
            (
                "MiniTablineHidden".to_string(),
                HighlightGroup::new().fg(&palette.fg).bg(&palette.bg1),
            ),
            (
                "MiniTablineModifiedCurrent".to_string(),
                HighlightGroup::new().fg(&palette.orange).fmt("bold,italic"),
            ),
            (
                "MiniTablineModifiedHidden".to_string(),
                HighlightGroup::new()
                    .fg(&palette.light_grey)
                    .bg(&palette.bg1)
                    .fmt("italic"),
            ),
            (
                "MiniTablineModifiedVisible".to_string(),
                HighlightGroup::new()
                    .fg(&palette.yellow)
                    .bg(&palette.bg0)
                    .fmt("italic"),
            ),
            (
                "MiniTablineTabpagesection".to_string(),
                HighlightGroup::new()
                    .fg(&palette.bg0)
                    .bg(&palette.bg_yellow),
            ),
            (
                "MiniTablineVisible".to_string(),
                HighlightGroup::new()
                    .fg(&palette.light_grey)
                    .bg(&palette.bg0),
            ),
            (
                "MiniTestEmphasis".to_string(),
                HighlightGroup::new().fmt("bold"),
            ),
            (
                "MiniTestFail".to_string(),
                HighlightGroup::new().fg(&palette.red).fmt("bold"),
            ),
            (
                "MiniTestPass".to_string(),
                HighlightGroup::new().fg(&palette.green).fmt("bold"),
            ),
            (
                "MiniTrailspace".to_string(),
                HighlightGroup::new().bg(&palette.red),
            ),
        ]);

        plugins.insert("mini".to_string(), mini);

        // Language specific highlights
        let mut langs = HashMap::new();

        // C language highlights
        let c_highlights = HashMap::from([
            (
                "cInclude".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "cStorageClass".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "cTypedef".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "cDefine".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "cTSInclude".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "cTSConstant".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "cTSConstMacro".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "cTSOperator".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
        ]);
        langs.insert("c".to_string(), c_highlights);

        // C++ language highlights
        let cpp_highlights = HashMap::from([
            (
                "cppStatement".to_string(),
                HighlightGroup::new().fg(&palette.purple).fmt("bold"),
            ),
            (
                "cppTSInclude".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "cppTSConstant".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "cppTSConstMacro".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "cppTSOperator".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
        ]);
        langs.insert("cpp".to_string(), cpp_highlights);

        // Markdown language highlights
        let markdown_highlights = HashMap::from([
            (
                "markdownBlockquote".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownBold".to_string(),
                HighlightGroup::new().fmt("bold"),
            ),
            (
                "markdownBoldDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownCode".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "markdownCodeBlock".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "markdownCodeDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "markdownH1".to_string(),
                HighlightGroup::new().fg(&palette.red).fmt("bold"),
            ),
            (
                "markdownH2".to_string(),
                HighlightGroup::new().fg(&palette.purple).fmt("bold"),
            ),
            (
                "markdownH3".to_string(),
                HighlightGroup::new().fg(&palette.orange).fmt("bold"),
            ),
            (
                "markdownH4".to_string(),
                HighlightGroup::new().fg(&palette.red).fmt("bold"),
            ),
            (
                "markdownH5".to_string(),
                HighlightGroup::new().fg(&palette.purple).fmt("bold"),
            ),
            (
                "markdownH6".to_string(),
                HighlightGroup::new().fg(&palette.orange).fmt("bold"),
            ),
            (
                "markdownHeadingDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownHeadingRule".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownId".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "markdownIdDeclaration".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "markdownItalic".to_string(),
                HighlightGroup::new().fmt("italic"),
            ),
            (
                "markdownItalicDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey).fmt("italic"),
            ),
            (
                "markdownLinkDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownLinkText".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "markdownLinkTextDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownListMarker".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "markdownOrderedListMarker".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "markdownRule".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "markdownUrl".to_string(),
                HighlightGroup::new().fg(&palette.blue).fmt("underline"),
            ),
            (
                "markdownUrlDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.grey),
            ),
            (
                "markdownUrlTitleDelimiter".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
        ]);
        langs.insert("markdown".to_string(), markdown_highlights);

        // PHP language highlights
        let php_highlights = HashMap::from([
            (
                "phpFunctions".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .fmt(&config.code_style.functions),
            ),
            (
                "phpMethods".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "phpStructure".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "phpOperator".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "phpMemberSelector".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "phpVarSelector".to_string(),
                HighlightGroup::new()
                    .fg(&palette.orange)
                    .fmt(&config.code_style.variables),
            ),
            (
                "phpIdentifier".to_string(),
                HighlightGroup::new()
                    .fg(&palette.orange)
                    .fmt(&config.code_style.variables),
            ),
            (
                "phpBoolean".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "phpNumber".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "phpHereDoc".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "phpNowDoc".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "phpSCKeyword".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords),
            ),
            (
                "phpFCKeyword".to_string(),
                HighlightGroup::new()
                    .fg(&palette.purple)
                    .fmt(&config.code_style.keywords),
            ),
            (
                "phpRegion".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
        ]);
        langs.insert("php".to_string(), php_highlights);

        // Scala language highlights
        let scala_highlights = HashMap::from([
            (
                "scalaNameDefinition".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "scalaInterpolationBoundary".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "scalaInterpolation".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "scalaTypeOperator".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "scalaOperator".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "scalaKeywordModifier".to_string(),
                HighlightGroup::new()
                    .fg(&palette.red)
                    .fmt(&config.code_style.keywords),
            ),
        ]);
        langs.insert("scala".to_string(), scala_highlights);

        // TeX language highlights
        let tex_highlights = HashMap::from([
            (
                "latexTSInclude".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "latexTSFuncMacro".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .fmt(&config.code_style.functions),
            ),
            (
                "latexTSEnvironment".to_string(),
                HighlightGroup::new().fg(&palette.cyan).fmt("bold"),
            ),
            (
                "latexTSEnvironmentName".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "texCmdEnv".to_string(),
                HighlightGroup::new().fg(&palette.cyan),
            ),
            (
                "texEnvArgName".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "latexTSTitle".to_string(),
                HighlightGroup::new().fg(&palette.green),
            ),
            (
                "latexTSType".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "latexTSMath".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "texMathZoneX".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "texMathZoneXX".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "texMathDelimZone".to_string(),
                HighlightGroup::new().fg(&palette.light_grey),
            ),
            (
                "texMathDelim".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "texMathOper".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "texCmd".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "texCmdPart".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "texCmdPackage".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "texPgfType".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
        ]);
        langs.insert("tex".to_string(), tex_highlights);

        // Vim language highlights
        let vim_highlights = HashMap::from([
            (
                "vimOption".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "vimSetEqual".to_string(),
                HighlightGroup::new().fg(&palette.yellow),
            ),
            (
                "vimMap".to_string(),
                HighlightGroup::new().fg(&palette.purple),
            ),
            (
                "vimMapModKey".to_string(),
                HighlightGroup::new().fg(&palette.orange),
            ),
            (
                "vimNotation".to_string(),
                HighlightGroup::new().fg(&palette.red),
            ),
            (
                "vimMapLhs".to_string(),
                HighlightGroup::new().fg(&palette.fg),
            ),
            (
                "vimMapRhs".to_string(),
                HighlightGroup::new().fg(&palette.blue),
            ),
            (
                "vimVar".to_string(),
                HighlightGroup::new()
                    .fg(&palette.fg)
                    .fmt(&config.code_style.variables),
            ),
            (
                "vimCommentTitle".to_string(),
                HighlightGroup::new()
                    .fg(&palette.light_grey)
                    .fmt(&config.code_style.comments),
            ),
        ]);
        langs.insert("vim".to_string(), vim_highlights);

        hl.langs = langs;
        hl.plugins = plugins;

        hl
    }
}

//impl Highlights {
//    pub fn new() -> Self {
//        Highlights::default()
//    }
//}

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

// Merge user-defined highlights without overwriting with "none"
fn merge_highlights(highlights: &HashMap<String, HighlightGroup>) -> Result<()> {
    for (group_name, group_settings) in highlights {
        let mut opts = SetHighlightOpts::builder();

        if group_settings.fg != "none" {
            opts.foreground(&group_settings.fg);
        }
        if group_settings.bg != "none" {
            opts.background(&group_settings.bg);
        }
        if group_settings.sp != "none" {
            opts.special(&group_settings.sp);
        }
        if group_settings.fmt != "none" {
            opts.italic(group_settings.fmt == "italic")
                .bold(group_settings.fmt == "bold")
                .underline(group_settings.fmt == "underline")
                .undercurl(group_settings.fmt == "undercurl")
                .reverse(group_settings.fmt == "reverse");
        }

        api::set_hl(0, group_name, &opts.build())?;
    }
    Ok(())
}

// Setup function
pub fn setup() -> Result<()> {
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

    let config = get_global_config().unwrap_or_default();
    // Apply user-defined highlights
    if let Some(highlights) = config.highlights {
        if let Some(common) = &highlights.common {
            merge_highlights(common)?;
        }
        if let Some(syntax) = &highlights.syntax {
            merge_highlights(syntax)?;
        }
        if let Some(treesitter) = &highlights.treesitter {
            merge_highlights(treesitter)?;
        }
        if let Some(lsp) = &highlights.lsp {
            merge_highlights(lsp)?;
        }
        if let Some(plugins) = &highlights.plugins {
            for (_, group) in plugins {
                merge_highlights(group)?;
            }
        }
        if let Some(langs) = &highlights.langs {
            for (_, group) in langs {
                merge_highlights(group)?;
            }
        }
    }

    Ok(())
}
