use nvim_oxi::{
    api::{self, opts::SetHighlightOpts, Error::Other},
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{config::DiagnosticsConfig, util, OneDarkConfig, GLOBAL_CONFIG};

// Highlight group structure
#[derive(Clone, Debug, Serialize, Deserialize)]
struct HighlightGroup {
    fg: Box<str>,
    bg: Box<str>,
    sp: Box<str>,
    fmt: Option<Vec<FmtType>>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum FmtType {
    Bold,
    Italic,
    Underline,
    Undercurl,
    Reverse,
    NoCombine,
    StrikeThrough,
}

impl Default for HighlightGroup {
    fn default() -> Self {
        HighlightGroup {
            fg: "none".into(),
            bg: "none".into(),
            sp: "none".into(),
            fmt: None,
        }
    }
}

impl HighlightGroup {
    fn new() -> Self {
        Self::default()
    }

    fn fg(mut self, fg: Box<str>) -> Self {
        self.fg = fg;
        self
    }

    fn bg(mut self, bg: Box<str>) -> Self {
        self.bg = bg;
        self
    }

    fn sp(mut self, sp: Box<str>) -> Self {
        self.sp = sp.into();
        self
    }

    fn fmt<T: Into<Option<Vec<FmtType>>>>(mut self, fmt: T) -> Self {
        self.fmt = fmt.into();
        self
    }
}

// Highlight collection structure
#[derive(Clone, Debug)]
pub struct Highlights {
    common: HashMap<Box<str>, HighlightGroup>,
    syntax: HashMap<Box<str>, HighlightGroup>,
    treesitter: HashMap<Box<str>, HighlightGroup>,
    lsp: Option<HashMap<Box<str>, HighlightGroup>>,
    plugins: HashMap<Box<str>, HashMap<Box<str>, HighlightGroup>>,
    langs: HashMap<Box<str>, HashMap<Box<str>, HighlightGroup>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ConfigHighlights {
    common: Option<HashMap<Box<str>, HighlightGroup>>,
    syntax: Option<HashMap<Box<str>, HighlightGroup>>,
    treesitter: Option<HashMap<Box<str>, HighlightGroup>>,
    lsp: Option<HashMap<Box<str>, HighlightGroup>>,
    plugins: Option<HashMap<Box<str>, HashMap<Box<str>, HighlightGroup>>>,
    langs: Option<HashMap<Box<str>, HashMap<Box<str>, HighlightGroup>>>,
}

impl Default for Highlights {
    fn default() -> Self {
        use FmtType::{Bold, Italic, NoCombine, Reverse, StrikeThrough, Undercurl, Underline};
        let palette = crate::palette::merge_palletes();

        let transparent;
        let ending_tildes;
        let diagnostics;
        let cmp_itemkind_reverse;
        let code_style;

        {
            match GLOBAL_CONFIG.read() {
                Ok(config) => {
                    transparent = config.transparent;
                    ending_tildes = config.ending_tildes;
                    cmp_itemkind_reverse = config.cmp_itemkind_reverse;
                    diagnostics = DiagnosticsConfig {
                        darker: config.diagnostics.darker,
                        undercurl: config.diagnostics.undercurl,
                        background: config.diagnostics.background,
                    };
                    code_style = config.code_style.clone();
                }
                Err(_) => {
                    let conf = OneDarkConfig::default();
                    transparent = conf.transparent;
                    ending_tildes = conf.ending_tildes;
                    cmp_itemkind_reverse = conf.cmp_itemkind_reverse;
                    diagnostics = conf.diagnostics;
                    code_style = conf.code_style;
                }
            }
        }

        let common_float_border = HighlightGroup::new()
            .fg(palette.grey.clone())
            .bg(palette.bg1.clone());
        let common_normal_float = HighlightGroup::new()
            .fg(palette.fg.clone())
            .bg(palette.bg1.clone());
        let common_added = HighlightGroup::new().fg(palette.green.clone());
        let common_removed = HighlightGroup::new().fg(palette.red.clone());
        let common_diff_delete = HighlightGroup::new()
            .fg("none".into())
            .bg(palette.diff_delete);
        let common_diff_text = HighlightGroup::new()
            .fg("none".into())
            .bg(palette.diff_text.clone());
        let common_diff_add = HighlightGroup::new().fg("none".into()).bg(palette.diff_add);
        let common_diff_change = HighlightGroup::new()
            .fg("none".into())
            .bg(palette.diff_change);
        let common_inc_search = HighlightGroup::new()
            .fg(palette.bg0.clone())
            .bg(palette.orange.clone());
        let common_directory = HighlightGroup::new().fg(palette.blue.clone());

        // Common highlights
        let hl_common = HashMap::from([
            (
                "Normal".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "Terminal".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "EndOfBuffer".into(),
                HighlightGroup::new()
                    .fg(if ending_tildes {
                        palette.bg2.clone()
                    } else {
                        palette.bg0.clone()
                    })
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "FoldColumn".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg1.clone()
                    }),
            ),
            (
                "Folded".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg1.clone()
                    }),
            ),
            (
                "SignColumn".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "ToolbarLine".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            ("Cursor".into(), HighlightGroup::new().fmt(vec![Reverse])),
            ("vCursor".into(), HighlightGroup::new().fmt(vec![Reverse])),
            ("iCursor".into(), HighlightGroup::new().fmt(vec![Reverse])),
            ("lCursor".into(), HighlightGroup::new().fmt(vec![Reverse])),
            ("CursorIM".into(), HighlightGroup::new().fmt(vec![Reverse])),
            (
                "CursorColumn".into(),
                HighlightGroup::new().bg(palette.bg1.clone()),
            ),
            (
                "CursorLine".into(),
                HighlightGroup::new().bg(palette.bg1.clone()),
            ),
            (
                "ColorColumn".into(),
                HighlightGroup::new().bg(palette.bg1.clone()),
            ),
            (
                "CursorLineNr".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "LineNr".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "Conceal".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            ("Added".into(), common_added.clone()),
            ("Removed".into(), common_removed.clone()),
            (
                "Changed".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            ("DiffAdd".into(), common_diff_add.clone()),
            ("DiffChange".into(), common_diff_change.clone()),
            ("DiffDelete".into(), common_diff_delete.clone()),
            ("DiffText".into(), common_diff_text.clone()),
            (
                "DiffAdded".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "DiffChanged".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffRemoved".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "DiffDeleted".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "DiffFile".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "DiffIndexLine".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            ("Directory".into(), common_directory.clone()),
            (
                "ErrorMsg".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "WarningMsg".into(),
                HighlightGroup::new()
                    .fg(palette.yellow.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MoreMsg".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "CurSearch".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.orange.clone()),
            ),
            ("IncSearch".into(), common_inc_search.clone()),
            (
                "Search".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.bg_yellow.clone()),
            ),
            (
                "Substitute".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.green.clone()),
            ),
            (
                "MatchParen".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .bg(palette.grey.clone()),
            ),
            (
                "NonText".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "Whitespace".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "SpecialKey".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "Pmenu".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "PmenuSbar".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .bg(palette.bg1.clone()),
            ),
            (
                "PmenuSel".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.bg_blue.clone()),
            ),
            (
                "WildMenu".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.blue.clone()),
            ),
            (
                "PmenuThumb".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .bg(palette.grey.clone()),
            ),
            (
                "Question".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "SpellBad".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .fmt(vec![Undercurl])
                    .sp(palette.red.clone()),
            ),
            (
                "SpellCap".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .fmt(vec![Undercurl])
                    .sp(palette.yellow.clone()),
            ),
            (
                "SpellLocal".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .fmt(vec![Undercurl])
                    .sp(palette.blue.clone()),
            ),
            (
                "SpellRare".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .fmt(vec![Undercurl])
                    .sp(palette.purple.clone()),
            ),
            (
                "StatusLine".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg2.clone()),
            ),
            (
                "StatusLineTerm".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg2.clone()),
            ),
            (
                "StatusLineNC".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "StatusLineTermNC".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "TabLine".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "TabLineFill".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "TabLineSel".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.fg.clone()),
            ),
            (
                "WinSeparator".into(),
                HighlightGroup::new().fg(palette.bg3.clone()),
            ),
            (
                "Visual".into(),
                HighlightGroup::new().bg(palette.bg3.clone()),
            ),
            (
                "VisualNOS".into(),
                HighlightGroup::new()
                    .fg("none".into())
                    .bg(palette.bg2.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "QuickFixLine".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "Debug".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "debugPC".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.green.clone()),
            ),
            (
                "debugBreakpoint".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.red.clone()),
            ),
            (
                "ToolbarButton".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.bg_blue.clone()),
            ),
            ("FloatBorder".into(), common_float_border.clone()),
            ("NormalFloat".into(), common_normal_float.clone()),
        ]);

        let syntax_comment = HighlightGroup::new()
            .fg(palette.grey.clone())
            .fmt(code_style.comments.clone());
        let syntax_title = HighlightGroup::new().fg(palette.cyan.clone());
        let syntax_special = HighlightGroup::new().fg(palette.red.clone());
        let syntax_delimiter = HighlightGroup::new().fg(palette.light_grey.clone());

        // Syntax highlights
        let hl_syntax = HashMap::from([
            (
                "String".into(),
                HighlightGroup::new()
                    .fg(palette.green.clone())
                    .fmt(code_style.strings.clone()),
            ),
            (
                "Character".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "Number".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "Float".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "Boolean".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "Type".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "Structure".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "StorageClass".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "Identifier".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(code_style.variables.clone()),
            ),
            (
                "Constant".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "PreProc".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "PreCondit".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Include".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Keyword".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "Define".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Typedef".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "Exception".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Conditional".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "Repeat".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "Statement".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Macro".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "Error".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "Label".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            ("Special".into(), syntax_special.clone()),
            (
                "SpecialChar".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "Function".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "Operator".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            ("Title".into(), syntax_title.clone()),
            (
                "Tag".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            ("Delimiter".into(), syntax_delimiter.clone()),
            ("Comment".into(), syntax_comment.clone()),
            (
                "SpecialComment".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(code_style.comments.clone()),
            ),
            (
                "Todo".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(code_style.comments.clone()),
            ),
        ]);

        // TreeSitter highlights
        let hl_treesitter = HashMap::from([
            (
                "TSAnnotation".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSAttribute".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TSBoolean".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSCharacter".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSComment".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(code_style.comments.clone()),
            ),
            (
                "TSConditional".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "TSConstant".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSConstBuiltin".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSConstMacro".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSConstructor".into(),
                HighlightGroup::new()
                    .fg(palette.yellow.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "TSError".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSException".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "TSField".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TSFloat".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSFunction".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "TSFuncBuiltin".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "TSFuncMacro".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "TSInclude".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "TSKeyword".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "TSKeywordFunction".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "TSKeywordOperator".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "TSLabel".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "TSMethod".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "TSNamespace".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "TSNone".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSNumber".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSOperator".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSParameter".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "TSParameterReference".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSProperty".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TSPunctDelimiter".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "TSPunctBracket".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "TSPunctSpecial".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "TSRepeat".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "TSString".into(),
                HighlightGroup::new()
                    .fg(palette.green.clone())
                    .fmt(code_style.strings.clone()),
            ),
            (
                "TSStringRegex".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(code_style.strings.clone()),
            ),
            (
                "TSStringEscape".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(code_style.strings.clone()),
            ),
            (
                "TSSymbol".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TSTag".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "TSTagDelimiter".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "TSText".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSStrong".into(),
                HighlightGroup::new().fg(palette.fg.clone()).fmt(vec![Bold]),
            ),
            (
                "TSEmphasis".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(vec![Italic]),
            ),
            (
                "TSUnderline".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "TSStrike".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(vec![StrikeThrough]),
            ),
            (
                "TSTitle".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "TSLiteral".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "TSURI".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "TSMath".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSTextReference".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "TSEnvironment".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSEnvironmentName".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSNote".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSWarning".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSDanger".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "TSType".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "TSTypeBuiltin".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "TSVariable".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(code_style.variables.clone()),
            ),
            (
                "TSVariableBuiltin".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(code_style.variables.clone()),
            ),
        ]);

        // LSP plugin highlights
        let diagnostics_error_color = if diagnostics.darker {
            palette.dark_red.clone()
        } else {
            palette.red.clone()
        };

        let diagnostics_hint_color = if diagnostics.darker {
            palette.dark_purple.clone()
        } else {
            palette.purple.clone()
        };

        let diagnostics_warn_color = if diagnostics.darker {
            palette.dark_yellow.clone()
        } else {
            palette.yellow.clone()
        };

        let diagnostics_info_color = if diagnostics.darker {
            palette.dark_cyan.clone()
        } else {
            palette.cyan.clone()
        };

        let error_bg = if diagnostics.background {
            match util::darken(
                diagnostics_error_color.clone(),
                0.1,
                Some(palette.bg0.clone()),
            ) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let warn_bg = if diagnostics.background {
            match util::darken(
                diagnostics_warn_color.clone(),
                0.1,
                Some(palette.bg0.clone()),
            ) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let info_bg = if diagnostics.background {
            match util::darken(
                diagnostics_info_color.clone(),
                0.1,
                Some(palette.bg0.clone()),
            ) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };
        let hint_bg = if diagnostics.background {
            match util::darken(
                diagnostics_hint_color.clone(),
                0.1,
                Some(palette.bg0.clone()),
            ) {
                Ok(color) => color,
                Err(_) => "none".into(),
            }
        } else {
            "none".into()
        };

        // vec![Underline] diagnostics
        let underline_fmt = if diagnostics.undercurl {
            vec![Undercurl]
        } else {
            vec![Underline]
        };

        let diagnostic_error = HighlightGroup::new().fg(palette.red.clone());
        let diagnostic_hint = HighlightGroup::new().fg(palette.purple.clone());
        let diagnostic_info = HighlightGroup::new().fg(palette.cyan.clone());
        let diagnostic_warn = HighlightGroup::new().fg(palette.yellow.clone());
        let diagnostic_virtual_text_error = HighlightGroup::new()
            .fg(diagnostics_error_color)
            .bg(error_bg);
        let diagnostic_virtual_text_warn =
            HighlightGroup::new().fg(diagnostics_warn_color).bg(warn_bg);
        let diagnostic_virtual_text_info =
            HighlightGroup::new().fg(diagnostics_info_color).bg(info_bg);
        let diagnostic_virtual_text_hint = HighlightGroup::new()
            .fg(diagnostics_hint_color.clone())
            .bg(hint_bg);
        let diagnostic_underline_error = HighlightGroup::new()
            .fmt(underline_fmt.clone())
            .sp(palette.red.clone());
        let diagnostic_underline_warn = HighlightGroup::new()
            .fmt(underline_fmt.clone())
            .sp(palette.yellow.clone());
        let diagnostic_underline_info = HighlightGroup::new()
            .fmt(underline_fmt.clone())
            .sp(palette.blue.clone());
        let diagnostic_underline_hint = HighlightGroup::new()
            .fmt(underline_fmt)
            .sp(palette.purple.clone());

        let lsp_plugin = HashMap::from([
            (
                "LspCxxHlGroupEnumConstant".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "LspCxxHlGroupMemberVariable".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "LspCxxHlGroupNamespace".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "LspCxxHlSkippedRegion".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "LspCxxHlSkippedRegionBeginEnd".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            ("DiagnosticError".into(), diagnostic_error.clone()),
            ("DiagnosticHint".into(), diagnostic_hint.clone()),
            ("DiagnosticInfo".into(), diagnostic_info.clone()),
            ("DiagnosticWarn".into(), diagnostic_warn.clone()),
            (
                "DiagnosticVirtualTextError".into(),
                diagnostic_virtual_text_error.clone(),
            ),
            (
                "DiagnosticVirtualTextWarn".into(),
                diagnostic_virtual_text_warn.clone(),
            ),
            (
                "DiagnosticVirtualTextInfo".into(),
                diagnostic_virtual_text_info.clone(),
            ),
            (
                "DiagnosticVirtualTextHint".into(),
                diagnostic_virtual_text_hint.clone(),
            ),
            (
                "DiagnosticUnderlineInfo".into(),
                diagnostic_virtual_text_info.clone(),
            ),
            (
                "DiagnosticUnderlineWarn".into(),
                diagnostic_virtual_text_warn.clone(),
            ),
            (
                "LspReferenceText".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            (
                "LspReferenceWrite".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            (
                "LspReferenceRead".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            (
                "LspCodeLens".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(code_style.comments.clone()),
            ),
            (
                "LspCodeLensSeparator".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "LspDiagnosticsDefaultError".into(),
                diagnostic_error.clone(),
            ),
            ("LspDiagnosticsDefaultHint".into(), diagnostic_hint.clone()),
            (
                "LspDiagnosticsDefaultInformation".into(),
                diagnostic_info.clone(),
            ),
            (
                "LspDiagnosticsDefaultWarning".into(),
                diagnostic_warn.clone(),
            ),
            (
                "LspDiagnosticsUnderlineError".into(),
                diagnostic_underline_error,
            ),
            (
                "LspDiagnosticsUnderlineHint".into(),
                diagnostic_underline_hint,
            ),
            (
                "LspDiagnosticsUnderlineInformation".into(),
                diagnostic_underline_info,
            ),
            (
                "LspDiagnosticsUnderlineWarning".into(),
                diagnostic_underline_warn,
            ),
            (
                "LspDiagnosticsVirtualTextError".into(),
                diagnostic_virtual_text_error,
            ),
            (
                "LspDiagnosticsVirtualTextWarning".into(),
                diagnostic_virtual_text_warn,
            ),
            (
                "LspDiagnosticsVirtualTextInformation".into(),
                diagnostic_virtual_text_info,
            ),
            (
                "LspDiagnosticsVirtualTextHint".into(),
                diagnostic_virtual_text_hint,
            ),
        ]);

        let lsp_kind = HashMap::from([
            ("Default", palette.purple.clone()),
            ("Array", palette.yellow.clone()),
            ("Boolean", palette.orange.clone()),
            ("Class", palette.yellow.clone()),
            ("Color", palette.green.clone()),
            ("Constant", palette.orange.clone()),
            ("Constructor", palette.blue.clone()),
            ("Enum", palette.purple.clone()),
            ("EnumMember", palette.yellow.clone()),
            ("Event", palette.yellow.clone()),
            ("Field", palette.purple.clone()),
            ("File", palette.blue.clone()),
            ("Folder", palette.orange.clone()),
            ("Function", palette.blue.clone()),
            ("Interface", palette.green.clone()),
            ("Key", palette.cyan.clone()),
            ("Keyword", palette.cyan.clone()),
            ("Method", palette.blue.clone()),
            ("Module", palette.orange.clone()),
            ("Namespace", palette.red.clone()),
            ("Null", palette.grey.clone()),
            ("Number", palette.orange.clone()),
            ("Object", palette.red.clone()),
            ("Operator", palette.red.clone()),
            ("Package", palette.yellow.clone()),
            ("Property", palette.cyan.clone()),
            ("Reference", palette.orange.clone()),
            ("Snippet", palette.red.clone()),
            ("String", palette.green.clone()),
            ("Struct", palette.purple.clone()),
            ("Text", palette.light_grey.clone()),
            ("TypeParameter", palette.red.clone()),
            ("Unit", palette.green.clone()),
            ("Value", palette.orange.clone()),
            ("Variable", palette.purple.clone()),
        ]);

        let cmp_item_kind_fmt: Option<Vec<FmtType>> = if cmp_itemkind_reverse {
            vec![Reverse].into()
        } else {
            None.into()
        };
        // CMP plugin highlights
        let mut cmp_highlights = HashMap::from([
            (
                "CmpItemAbbr".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "CmpItemAbbrDeprecated".into(),
                HighlightGroup::new()
                    .fg(palette.light_grey.clone())
                    .fmt(vec![StrikeThrough]),
            ),
            (
                "CmpItemAbbrMatch".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "CmpItemAbbrMatchFuzzy".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "CmpItemMenu".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "CmpItemKind".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(cmp_item_kind_fmt.unwrap_or_default()),
            ),
        ]);

        for (kind, color) in lsp_kind.iter() {
            let lsp_kind_cmp: Option<Vec<FmtType>> = if cmp_itemkind_reverse {
                vec![Reverse].into()
            } else {
                None.into()
            };
            cmp_highlights.insert(
                format!("CmpItemKind{}", kind).into(),
                HighlightGroup::new()
                    .fg(color.clone())
                    .fmt(lsp_kind_cmp.unwrap_or_default()),
            );
        }

        // WhichKey plugin highlights
        let whichkey_highlights = HashMap::from([
            (
                "WhichKey".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "WhichKeyDesc".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "WhichKeyGroup".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "WhichKeySeparator".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
        ]);

        // GitGutter plugin highlights
        let gitgutter_highlights = HashMap::from([
            (
                "GitGutterAdd".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "GitGutterChange".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "GitGutterDelete".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
        ]);

        // DiffView plugin highlights
        let diffview_highlights = HashMap::from([
            (
                "DiffviewFilePanelTitle".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "DiffviewFilePanelCounter".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "DiffviewFilePanelFileName".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "DiffviewNormal".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "DiffviewCursorLine".into(),
                HighlightGroup::new().bg(palette.bg1.clone()),
            ),
            (
                "DiffviewVertSplit".into(),
                HighlightGroup::new().fg(palette.bg3),
            ),
            (
                "DiffviewSignColumn".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "DiffviewStatusLine".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg2.clone()),
            ),
            (
                "DiffviewStatusLineNC".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "DiffviewEndOfBuffer".into(),
                HighlightGroup::new()
                    .fg(if ending_tildes {
                        palette.bg2.clone()
                    } else {
                        palette.bg0.clone()
                    })
                    .bg(if transparent {
                        "none".into()
                    } else {
                        palette.bg0.clone()
                    }),
            ),
            (
                "DiffviewFilePanelRootPath".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "DiffviewFilePanelPath".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "DiffviewFilePanelInsertions".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "DiffviewFilePanelDeletions".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "DiffviewStatusAdded".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "DiffviewStatusUntracked".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusModified".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusRenamed".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusCopied".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusTypeChange".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusUnmerged".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "DiffviewStatusUnknown".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "DiffviewStatusDeleted".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "DiffviewStatusBroken".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
        ]);

        // GitSigns plugin highlights
        let gitsigns_highlights = HashMap::from([
            (
                "GitSignsAdd".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "GitSignsAddLn".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "GitSignsAddNr".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "GitSignsChange".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "GitSignsChangeLn".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "GitSignsChangeNr".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "GitSignsDelete".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "GitSignsDeleteLn".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "GitSignsDeleteNr".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
        ]);

        // Telescope plugin highlights
        let telescope_highlights = HashMap::from([
            (
                "TelescopeBorder".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "TelescopePromptBorder".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TelescopeResultsBorder".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TelescopePreviewBorder".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "TelescopeMatching".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "TelescopePromptPrefix".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "TelescopeSelection".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            (
                "TelescopeSelectionCaret".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
        ]);

        let plugin_indent_line = HashMap::from([
            (
                "IndentBlankLine1".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "IndentBlankLine2".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "IndentBlankLine3".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "IndentBlankLine4".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "IndentBlankLine5".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "IndentBlankLine6".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "IndentBlanklineChar".into(),
                HighlightGroup::new()
                    .fg(palette.bg1.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "IndentBlanklineContextChar".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "IndentBlanklineContextStart".into(),
                HighlightGroup::new()
                    .sp(palette.grey.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "IndentBlanklineContextSpaceChar".into(),
                HighlightGroup::new().fmt(vec![NoCombine]),
            ),
            (
                "IblIndent".into(),
                HighlightGroup::new()
                    .fg(palette.bg1.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "IblWhitespace".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "IblScope".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(vec![NoCombine]),
            ),
        ]);

        let mini = HashMap::from([
            (
                "MiniAnimateCursor".into(),
                HighlightGroup::new().fmt(vec![Reverse, NoCombine]),
            ),
            ("MiniAnimateNormalFloat".into(), common_normal_float.clone()),
            ("MiniClueBorder".into(), common_float_border.clone()),
            ("MiniClueDescGroup".into(), diagnostic_warn.clone()),
            ("MiniClueDescSingle".into(), common_normal_float.clone()),
            ("MiniClueNextKey".into(), diagnostic_hint.clone()),
            (
                "MiniClueNextKeyWithPostkeys".into(),
                diagnostic_error.clone(),
            ),
            ("MiniClueSeparator".into(), diagnostic_info.clone()),
            (
                "MiniClueTitle".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "MiniCompletionActiveParameter".into(),
                HighlightGroup::new().fmt(vec![Underline]),
            ),
            (
                "MiniCursorword".into(),
                HighlightGroup::new().fmt(vec![Underline]),
            ),
            (
                "MiniCursorwordCurrent".into(),
                HighlightGroup::new().fmt(vec![Underline]),
            ),
            ("MiniDepsChangeAdded".into(), common_added),
            ("MiniDepsChangeRemoved".into(), common_removed),
            ("MiniDepsHint".into(), diagnostic_hint.clone()),
            ("MiniDepsInfo".into(), diagnostic_info.clone()),
            ("MiniDepsMsgBreaking".into(), diagnostic_warn.clone()),
            ("MiniDepsPlaceholder".into(), syntax_comment.clone()),
            ("MiniDepsTitle".into(), syntax_title.clone()),
            ("MiniDepsTitleError".into(), common_diff_delete.clone()),
            ("MiniDepsTitleSame".into(), common_diff_text.clone()),
            ("MiniDepsTitleUpdate".into(), common_diff_add.clone()),
            (
                "MiniDiffSignAdd".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "MiniDiffSignChange".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "MiniDiffSignDelete".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            ("MiniDiffOverAdd".into(), common_diff_add),
            ("MiniDiffOverChange".into(), common_diff_text),
            ("MiniDiffOverContext".into(), common_diff_change),
            ("MiniDiffOverDelete".into(), common_diff_delete),
            ("MiniFilesBorder".into(), common_float_border.clone()),
            ("MiniFilesBorderModified".into(), diagnostic_warn.clone()),
            (
                "MiniFilesCursorLine".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            ("MiniFilesDirectory".into(), common_directory.clone()),
            (
                "MiniFilesFile".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            ("MiniFilesNormal".into(), common_normal_float.clone()),
            (
                "MiniFilesTitle".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "MiniFilesTitleFocused".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniHipatternsFixme".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniHipatternsHack".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.yellow.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniHipatternsNote".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.cyan.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniHipatternsTodo".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniIconsAzure".into(),
                HighlightGroup::new().fg(palette.bg_blue),
            ),
            (
                "MiniIconsBlue".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "MiniIconsCyan".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "MiniIconsGreen".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "MiniIconsGrey".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "MiniIconsOrange".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "MiniIconsPurple".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "MiniIconsRed".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "MiniIconsYellow".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "MiniIndentscopeSymbol".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "MiniIndentscopePrefix".into(),
                HighlightGroup::new().fmt(vec![NoCombine]),
            ),
            (
                "MiniJump".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(vec![Underline])
                    .sp(palette.purple.clone()),
            ),
            (
                "MiniJump2dDim".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "MiniJump2dSpot".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(vec![Bold, NoCombine]),
            ),
            (
                "MiniJump2dSpotAhead".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .bg(palette.bg0.clone())
                    .fmt(vec![NoCombine]),
            ),
            (
                "MiniJump2dSpotUnique".into(),
                HighlightGroup::new()
                    .fg(palette.yellow.clone())
                    .fmt(vec![Bold, NoCombine]),
            ),
            ("MiniMapNormal".into(), common_normal_float.clone()),
            ("MiniMapSymbolCount".into(), syntax_special.clone()),
            ("MiniMapSymbolLine".into(), syntax_title.clone()),
            ("MiniMapSymbolView".into(), syntax_delimiter.clone()),
            ("MiniNotifyBorder".into(), common_float_border.clone()),
            ("MiniNotifyNormal".into(), common_normal_float.clone()),
            (
                "MiniNotifyTitle".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "MiniOperatorsExchangeFrom".into(),
                common_inc_search.clone(),
            ),
            ("MiniPickBorder".into(), common_float_border),
            ("MiniPickBorderBusy".into(), diagnostic_warn),
            (
                "MiniPickBorderText".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(vec![Bold]),
            ),
            ("MiniPickIconDirectory".into(), common_directory),
            ("MiniPickIconFile".into(), common_normal_float.clone()),
            ("MiniPickHeader".into(), diagnostic_hint.clone()),
            (
                "MiniPickMatchCurrent".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            (
                "MiniPickMatchMarked".into(),
                HighlightGroup::new().bg(palette.diff_text),
            ),
            ("MiniPickMatchRanges".into(), diagnostic_hint.clone()),
            ("MiniPickNormal".into(), common_normal_float.clone()),
            (
                "MiniPickPreviewLine".into(),
                HighlightGroup::new().bg(palette.bg2.clone()),
            ),
            ("MiniPickPreviewRegion".into(), common_inc_search.clone()),
            ("MiniPickPrompt".into(), diagnostic_info),
            (
                "MiniStarterCurrent".into(),
                HighlightGroup::new().fmt(vec![NoCombine]),
            ),
            (
                "MiniStarterFooter".into(),
                HighlightGroup::new().fg(palette.dark_red).fmt(vec![Italic]),
            ),
            (
                "MiniStarterHeader".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "MiniStarterInactive".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(code_style.comments.clone()),
            ),
            (
                "MiniStarterItem".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg0.clone()),
            ),
            (
                "MiniStarterItemBullet".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "MiniStarterItemPrefix".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "MiniStarterSection".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "MiniStarterQuery".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "MiniStatuslineDevinfo".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg2.clone()),
            ),
            (
                "MiniStatuslineFileinfo".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg2.clone()),
            ),
            (
                "MiniStatuslineFilename".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "MiniStatuslineInactive".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg0.clone()),
            ),
            (
                "MiniStatuslineModeCommand".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.yellow.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniStatuslineModeInsert".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.blue.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniStatuslineModeNormal".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.green.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniStatuslineModeOther".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.cyan.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniStatuslineModeReplace".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniStatuslineModeVisual".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniSurround".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.orange.clone()),
            ),
            (
                "MiniTablineCurrent".into(),
                HighlightGroup::new().fmt(vec![Bold]),
            ),
            (
                "MiniTablineFill".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "MiniTablineHidden".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .bg(palette.bg1.clone()),
            ),
            (
                "MiniTablineModifiedCurrent".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(vec![Bold, Italic]),
            ),
            (
                "MiniTablineModifiedHidden".into(),
                HighlightGroup::new()
                    .fg(palette.light_grey.clone())
                    .bg(palette.bg1)
                    .fmt(vec![Italic]),
            ),
            (
                "MiniTablineModifiedVisible".into(),
                HighlightGroup::new()
                    .fg(palette.yellow.clone())
                    .bg(palette.bg0.clone())
                    .fmt(vec![Italic]),
            ),
            (
                "MiniTablineTabpagesection".into(),
                HighlightGroup::new()
                    .fg(palette.bg0.clone())
                    .bg(palette.bg_yellow),
            ),
            (
                "MiniTablineVisible".into(),
                HighlightGroup::new()
                    .fg(palette.light_grey.clone())
                    .bg(palette.bg0.clone()),
            ),
            (
                "MiniTestEmphasis".into(),
                HighlightGroup::new().fmt(vec![Bold]),
            ),
            (
                "MiniTestFail".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniTestPass".into(),
                HighlightGroup::new()
                    .fg(palette.green.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "MiniTrailspace".into(),
                HighlightGroup::new().bg(palette.red.clone()),
            ),
        ]);

        // Language specific highlights

        // C language highlights
        let c_highlights = HashMap::from([
            (
                "cInclude".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "cStorageClass".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "cTypedef".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "cDefine".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "cTSInclude".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "cTSConstant".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "cTSConstMacro".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "cTSOperator".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
        ]);

        // C++ language highlights
        let cpp_highlights = HashMap::from([
            (
                "cppStatement".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "cppTSInclude".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "cppTSConstant".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "cppTSConstMacro".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "cppTSOperator".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
        ]);

        // Markdown language highlights
        let markdown_highlights = HashMap::from([
            (
                "markdownBlockquote".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            ("markdownBold".into(), HighlightGroup::new().fmt(vec![Bold])),
            (
                "markdownBoldDelimiter".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownCode".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "markdownCodeBlock".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "markdownCodeDelimiter".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "markdownH1".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownH2".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownH3".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownH4".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownH5".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownH6".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "markdownHeadingDelimiter".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownHeadingRule".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownId".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "markdownIdDeclaration".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "markdownItalic".into(),
                HighlightGroup::new().fmt(vec![Italic]),
            ),
            (
                "markdownItalicDelimiter".into(),
                HighlightGroup::new()
                    .fg(palette.grey.clone())
                    .fmt(vec![Italic]),
            ),
            (
                "markdownLinkDelimiter".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownLinkText".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "markdownLinkTextDelimiter".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownListMarker".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "markdownOrderedListMarker".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "markdownRule".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "markdownUrl".into(),
                HighlightGroup::new()
                    .fg(palette.blue.clone())
                    .fmt(vec![Underline]),
            ),
            (
                "markdownUrlDelimiter".into(),
                HighlightGroup::new().fg(palette.grey.clone()),
            ),
            (
                "markdownUrlTitleDelimiter".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
        ]);

        // PHP language highlights
        let php_highlights = HashMap::from([
            (
                "phpFunctions".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "phpMethods".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "phpStructure".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "phpOperator".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "phpMemberSelector".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "phpVarSelector".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(code_style.variables.clone()),
            ),
            (
                "phpIdentifier".into(),
                HighlightGroup::new()
                    .fg(palette.orange.clone())
                    .fmt(code_style.variables.clone()),
            ),
            (
                "phpBoolean".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "phpNumber".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "phpHereDoc".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "phpNowDoc".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "phpSCKeyword".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "phpFCKeyword".into(),
                HighlightGroup::new()
                    .fg(palette.purple.clone())
                    .fmt(code_style.keywords.clone()),
            ),
            (
                "phpRegion".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
        ]);

        // Scala language highlights
        let scala_highlights = HashMap::from([
            (
                "scalaNameDefinition".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "scalaInterpolationBoundary".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "scalaInterpolation".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "scalaTypeOperator".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "scalaOperator".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "scalaKeywordModifier".into(),
                HighlightGroup::new()
                    .fg(palette.red.clone())
                    .fmt(code_style.keywords.clone()),
            ),
        ]);

        // TeX language highlights
        let tex_highlights = HashMap::from([
            (
                "latexTSInclude".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "latexTSFuncMacro".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(code_style.functions.clone()),
            ),
            (
                "latexTSEnvironment".into(),
                HighlightGroup::new()
                    .fg(palette.cyan.clone())
                    .fmt(vec![Bold]),
            ),
            (
                "latexTSEnvironmentName".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "texCmdEnv".into(),
                HighlightGroup::new().fg(palette.cyan.clone()),
            ),
            (
                "texEnvArgName".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "latexTSTitle".into(),
                HighlightGroup::new().fg(palette.green.clone()),
            ),
            (
                "latexTSType".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "latexTSMath".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "texMathZoneX".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "texMathZoneXX".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "texMathDelimZone".into(),
                HighlightGroup::new().fg(palette.light_grey.clone()),
            ),
            (
                "texMathDelim".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "texMathOper".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "texCmd".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "texCmdPart".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "texCmdPackage".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "texPgfType".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
        ]);

        // Vim language highlights
        let vim_highlights = HashMap::from([
            (
                "vimOption".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "vimSetEqual".into(),
                HighlightGroup::new().fg(palette.yellow.clone()),
            ),
            (
                "vimMap".into(),
                HighlightGroup::new().fg(palette.purple.clone()),
            ),
            (
                "vimMapModKey".into(),
                HighlightGroup::new().fg(palette.orange.clone()),
            ),
            (
                "vimNotation".into(),
                HighlightGroup::new().fg(palette.red.clone()),
            ),
            (
                "vimMapLhs".into(),
                HighlightGroup::new().fg(palette.fg.clone()),
            ),
            (
                "vimMapRhs".into(),
                HighlightGroup::new().fg(palette.blue.clone()),
            ),
            (
                "vimVar".into(),
                HighlightGroup::new()
                    .fg(palette.fg.clone())
                    .fmt(code_style.variables.clone()),
            ),
            (
                "vimCommentTitle".into(),
                HighlightGroup::new()
                    .fg(palette.light_grey.clone())
                    .fmt(code_style.comments.clone()),
            ),
        ]);

        let hl_langs = HashMap::from([
            ("c".into(), c_highlights),
            ("cpp".into(), cpp_highlights),
            ("markdown".into(), markdown_highlights),
            ("php".into(), php_highlights),
            ("scala".into(), scala_highlights),
            ("tex".into(), tex_highlights),
            ("vim".into(), vim_highlights),
        ]);

        let hl_plugins = HashMap::from([
            ("lsp".into(), lsp_plugin),
            // Ale
            // Barbar
            ("cmp".into(), cmp_highlights),
            // Coc
            ("whichkey".into(), whichkey_highlights),
            ("gitgutter".into(), gitgutter_highlights),
            // Hop
            ("diffview".into(), diffview_highlights),
            ("gitsigns".into(), gitsigns_highlights),
            // Neo_tree
            // Neotest
            // Nvim_tree
            ("telescope".into(), telescope_highlights),
            // Dashboard
            // Outline
            // Navic
            // Ts_rainbow
            // Ts_rainbow2
            // Rainbow_delimiters
            ("indent_blankline".into(), plugin_indent_line),
            ("mini".into(), mini),
            // Illuminate
        ]);

        Highlights {
            common: hl_common,
            syntax: hl_syntax,
            treesitter: hl_treesitter,
            lsp: None,
            plugins: hl_plugins,
            langs: hl_langs,
        }
    }
}

// Apply vim highlights
fn vim_highlights(highlights: &HashMap<Box<str>, HighlightGroup>) -> Result<()> {
    for (group_name, group_settings) in highlights {
        let fmt = group_settings.fmt.as_ref();
        let opts = SetHighlightOpts::builder()
            .foreground(&group_settings.fg)
            .background(&group_settings.bg)
            .special(&group_settings.sp)
            .italic(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Italic)))
            .bold(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Bold)))
            .underline(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Underline)))
            .undercurl(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Undercurl)))
            .reverse(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Reverse)))
            .nocombine(fmt.map_or(false, |fmt| fmt.contains(&FmtType::NoCombine)))
            .build();

        api::set_hl(0, group_name, &opts)?;
    }
    Ok(())
}

// Merge user-defined highlights without overwriting with "none"
fn merge_highlights(highlights: &HashMap<Box<str>, HighlightGroup>) -> Result<()> {
    for (group_name, group_settings) in highlights {
        let fmt = group_settings.fmt.as_ref();
        let mut opts = SetHighlightOpts::builder();

        if &*group_settings.fg != "none" {
            opts.foreground(&group_settings.fg);
        }
        if &*group_settings.bg != "none" {
            opts.background(&group_settings.bg);
        }
        if &*group_settings.sp != "none" {
            opts.special(&group_settings.sp);
        }
        if group_settings.fmt.is_some() {
            opts.italic(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Italic)))
                .bold(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Bold)))
                .underline(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Underline)))
                .undercurl(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Undercurl)))
                .reverse(fmt.map_or(false, |fmt| fmt.contains(&FmtType::Reverse)))
                .nocombine(fmt.map_or(false, |fmt| fmt.contains(&FmtType::NoCombine)));
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

    let config = GLOBAL_CONFIG.read().map_err(|e| Other(format!("{}", e)))?;
    // Apply user-defined highlights
    if let Some(highlights) = &config.highlights {
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
