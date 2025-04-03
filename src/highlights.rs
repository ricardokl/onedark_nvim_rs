use nvim_oxi::{
    api::{self, opts::SetHighlightOpts, Error::Other},
    Result,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

use crate::{
    config::DiagnosticsConfig,
    palette::{Color, ColorPalette},
    util, OneDarkConfig, GLOBAL_CONFIG,
};

// Highlight group structure
#[derive(Clone, Debug, Deserialize)]
struct HighlightGroup<'a> {
    fg: Option<Cow<'a, Color>>,
    bg: Option<Cow<'a, Color>>,
    sp: Option<Cow<'a, Color>>,
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

impl Default for HighlightGroup<'_> {
    fn default() -> Self {
        HighlightGroup {
            fg: None,
            bg: None,
            sp: None,
            fmt: None,
        }
    }
}

impl<'a> HighlightGroup<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn fg<T: Into<Option<Cow<'a, Color>>>>(mut self, fg: T) -> Self {
        self.fg = fg.into();
        self
    }

    fn bg<T: Into<Option<Cow<'a, Color>>>>(mut self, bg: T) -> Self {
        self.bg = bg.into();
        self
    }

    fn sp<T: Into<Option<Cow<'a, Color>>>>(mut self, sp: T) -> Self {
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
pub struct Highlights<'a> {
    common: HashMap<&'a str, HighlightGroup<'a>>,
    syntax: HashMap<&'a str, HighlightGroup<'a>>,
    treesitter: HashMap<&'a str, HighlightGroup<'a>>,
    lsp: Option<HashMap<&'a str, HighlightGroup<'a>>>,
    plugins: HashMap<&'a str, HashMap<&'a str, HighlightGroup<'a>>>,
    langs: HashMap<&'a str, HashMap<&'a str, HighlightGroup<'a>>>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ConfigHighlights<'a> {
    common: Option<HashMap<Box<str>, HighlightGroup<'a>>>,
    syntax: Option<HashMap<Box<str>, HighlightGroup<'a>>>,
    treesitter: Option<HashMap<Box<str>, HighlightGroup<'a>>>,
    lsp: Option<HashMap<Box<str>, HighlightGroup<'a>>>,
    plugins: Option<HashMap<Box<str>, HashMap<Box<str>, HighlightGroup<'a>>>>,
    langs: Option<HashMap<Box<str>, HashMap<Box<str>, HighlightGroup<'a>>>>,
}

fn default<'a>(palette: &'a ColorPalette) -> Highlights<'a> {
    use FmtType::{Bold, Italic, NoCombine, Reverse, StrikeThrough, Undercurl, Underline};

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
        .fg(Cow::Borrowed(&palette.grey))
        .bg(Cow::Borrowed(&palette.bg1));
    let common_normal_float = HighlightGroup::new()
        .fg(Cow::Borrowed(&palette.fg))
        .bg(Cow::Borrowed(&palette.bg1));
    let common_added = HighlightGroup::new().fg(Cow::Borrowed(&palette.green));
    let common_removed = HighlightGroup::new().fg(Cow::Borrowed(&palette.red));
    let common_diff_delete = HighlightGroup::new().bg(Cow::Borrowed(&palette.diff_delete));
    let common_diff_text = HighlightGroup::new().bg(Cow::Borrowed(&palette.diff_text));
    let common_diff_add = HighlightGroup::new().bg(Cow::Borrowed(&palette.diff_add));
    let common_diff_change = HighlightGroup::new().bg(Cow::Borrowed(&palette.diff_change));
    let common_inc_search = HighlightGroup::new()
        .fg(Cow::Borrowed(&palette.bg0))
        .bg(Cow::Borrowed(&palette.orange));
    let common_directory = HighlightGroup::new().fg(Cow::Borrowed(&palette.blue));

    // Common highlights
    let hl_common = HashMap::from([
        (
            "Normal",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "Terminal",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "EndOfBuffer",
            HighlightGroup::new()
                .fg(if ending_tildes {
                    Cow::Borrowed(&palette.bg2)
                } else {
                    Cow::Borrowed(&palette.bg0)
                })
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "FoldColumn",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg1).into()
                }),
        ),
        (
            "Folded",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg1).into()
                }),
        ),
        (
            "SignColumn",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "ToolbarLine",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        ("Cursor", HighlightGroup::new().fmt(vec![Reverse])),
        ("vCursor", HighlightGroup::new().fmt(vec![Reverse])),
        ("iCursor", HighlightGroup::new().fmt(vec![Reverse])),
        ("lCursor", HighlightGroup::new().fmt(vec![Reverse])),
        ("CursorIM", HighlightGroup::new().fmt(vec![Reverse])),
        (
            "CursorColumn",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "CursorLine",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "ColorColumn",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "CursorLineNr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "LineNr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "Conceal",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        ("Added", common_added.clone()),
        ("Removed", common_removed.clone()),
        (
            "Changed",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        ("DiffAdd", common_diff_add.clone()),
        ("DiffChange", common_diff_change.clone()),
        ("DiffDelete", common_diff_delete.clone()),
        ("DiffText", common_diff_text.clone()),
        (
            "DiffAdded",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "DiffChanged",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffRemoved",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "DiffDeleted",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "DiffFile",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "DiffIndexLine",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        ("Directory", common_directory.clone()),
        (
            "ErrorMsg",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "WarningMsg",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.yellow))
                .fmt(vec![Bold]),
        ),
        (
            "MoreMsg",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(vec![Bold]),
        ),
        (
            "CurSearch",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.orange)),
        ),
        ("IncSearch", common_inc_search.clone()),
        (
            "Search",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.bg_yellow)),
        ),
        (
            "Substitute",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.green)),
        ),
        (
            "MatchParen",
            HighlightGroup::new()
                .fg(None)
                .bg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "NonText",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "Whitespace",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "SpecialKey",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "Pmenu",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "PmenuSbar",
            HighlightGroup::new()
                .fg(None)
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "PmenuSel",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.bg_blue)),
        ),
        (
            "WildMenu",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "PmenuThumb",
            HighlightGroup::new()
                .fg(None)
                .bg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "Question",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "SpellBad",
            HighlightGroup::new()
                .fg(None)
                .fmt(vec![Undercurl])
                .sp(Cow::Borrowed(&palette.red)),
        ),
        (
            "SpellCap",
            HighlightGroup::new()
                .fg(None)
                .fmt(vec![Undercurl])
                .sp(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "SpellLocal",
            HighlightGroup::new()
                .fg(None)
                .fmt(vec![Undercurl])
                .sp(Cow::Borrowed(&palette.blue)),
        ),
        (
            "SpellRare",
            HighlightGroup::new()
                .fg(None)
                .fmt(vec![Undercurl])
                .sp(Cow::Borrowed(&palette.purple)),
        ),
        (
            "StatusLine",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "StatusLineTerm",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "StatusLineNC",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "StatusLineTermNC",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "TabLine",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "TabLineFill",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "TabLineSel",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "WinSeparator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.bg3)),
        ),
        (
            "Visual",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg3)),
        ),
        (
            "VisualNOS",
            HighlightGroup::new()
                .fg(None)
                .bg(Cow::Borrowed(&palette.bg2))
                .fmt(vec![Underline]),
        ),
        (
            "QuickFixLine",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(vec![Underline]),
        ),
        (
            "Debug",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "debugPC",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.green)),
        ),
        (
            "debugBreakpoint",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.red)),
        ),
        (
            "ToolbarButton",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.bg_blue)),
        ),
        ("FloatBorder", common_float_border.clone()),
        ("NormalFloat", common_normal_float.clone()),
    ]);

    let syntax_comment = HighlightGroup::new()
        .fg(Cow::Borrowed(&palette.grey))
        .fmt(code_style.comments.clone());
    let syntax_title = HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan));
    let syntax_special = HighlightGroup::new().fg(Cow::Borrowed(&palette.red));
    let syntax_delimiter = HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey));

    // Syntax highlights
    let hl_syntax = HashMap::from([
        (
            "String",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.green))
                .fmt(code_style.strings.clone()),
        ),
        (
            "Character",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "Number",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "Float",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "Boolean",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "Type",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "Structure",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "StorageClass",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "Identifier",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(code_style.variables.clone()),
        ),
        (
            "Constant",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "PreProc",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "PreCondit",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Include",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Keyword",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "Define",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Typedef",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "Exception",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Conditional",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "Repeat",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "Statement",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Macro",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "Error",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "Label",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        ("Special", syntax_special.clone()),
        (
            "SpecialChar",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "Function",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(code_style.functions.clone()),
        ),
        (
            "Operator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        ("Title", syntax_title.clone()),
        (
            "Tag",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        ("Delimiter", syntax_delimiter.clone()),
        ("Comment", syntax_comment.clone()),
        (
            "SpecialComment",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(code_style.comments.clone()),
        ),
        (
            "Todo",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(code_style.comments.clone()),
        ),
    ]);

    // TreeSitter highlights
    let hl_treesitter = HashMap::from([
        (
            "TSAnnotation",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSAttribute",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TSBoolean",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSCharacter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSComment",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(code_style.comments.clone()),
        ),
        (
            "TSConditional",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "TSConstant",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSConstBuiltin",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSConstMacro",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSConstructor",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.yellow))
                .fmt(vec![Bold]),
        ),
        (
            "TSError",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSException",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "TSField",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TSFloat",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSFunction",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(code_style.functions.clone()),
        ),
        (
            "TSFuncBuiltin",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(code_style.functions.clone()),
        ),
        (
            "TSFuncMacro",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(code_style.functions.clone()),
        ),
        (
            "TSInclude",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "TSKeyword",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "TSKeywordFunction",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.functions.clone()),
        ),
        (
            "TSKeywordOperator",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "TSLabel",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "TSMethod",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(code_style.functions.clone()),
        ),
        (
            "TSNamespace",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "TSNone",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSNumber",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSParameter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "TSParameterReference",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSProperty",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TSPunctDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "TSPunctBracket",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "TSPunctSpecial",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "TSRepeat",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "TSString",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.green))
                .fmt(code_style.strings.clone()),
        ),
        (
            "TSStringRegex",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(code_style.strings.clone()),
        ),
        (
            "TSStringEscape",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(code_style.strings.clone()),
        ),
        (
            "TSSymbol",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TSTag",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "TSTagDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "TSText",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSStrong",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(vec![Bold]),
        ),
        (
            "TSEmphasis",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(vec![Italic]),
        ),
        (
            "TSUnderline",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(vec![Underline]),
        ),
        (
            "TSStrike",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(vec![StrikeThrough]),
        ),
        (
            "TSTitle",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(vec![Bold]),
        ),
        (
            "TSLiteral",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "TSURI",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Underline]),
        ),
        (
            "TSMath",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSTextReference",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "TSEnvironment",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSEnvironmentName",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSNote",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSWarning",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSDanger",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "TSType",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "TSTypeBuiltin",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "TSVariable",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(code_style.variables.clone()),
        ),
        (
            "TSVariableBuiltin",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(code_style.variables.clone()),
        ),
    ]);

    // LSP plugin highlights
    let diagnostics_error_color = if diagnostics.darker {
        Cow::Borrowed(&palette.dark_red)
    } else {
        Cow::Borrowed(&palette.red)
    };

    let diagnostics_hint_color = if diagnostics.darker {
        Cow::Borrowed(&palette.dark_purple)
    } else {
        Cow::Borrowed(&palette.purple)
    };

    let diagnostics_warn_color = if diagnostics.darker {
        Cow::Borrowed(&palette.dark_yellow)
    } else {
        Cow::Borrowed(&palette.yellow)
    };

    let diagnostics_info_color = if diagnostics.darker {
        Cow::Borrowed(&palette.dark_cyan)
    } else {
        Cow::Borrowed(&palette.cyan)
    };

    let error_bg = if diagnostics.background {
        Some(util::darken(
            &diagnostics_error_color,
            &0.1,
            Some(Cow::Borrowed(&palette.bg0)),
        ))
    } else {
        None
    };
    let warn_bg = if diagnostics.background {
        Some(util::darken(
            &diagnostics_warn_color,
            &0.1,
            Some(Cow::Borrowed(&palette.bg0)),
        ))
    } else {
        None
    };
    let info_bg = if diagnostics.background {
        Some(util::darken(
            &diagnostics_info_color,
            &0.1,
            Some(Cow::Borrowed(&palette.bg0)),
        ))
    } else {
        None
    };
    let hint_bg = if diagnostics.background {
        Some(util::darken(
            &diagnostics_hint_color,
            &0.1,
            Some(Cow::Borrowed(&palette.bg0)),
        ))
    } else {
        None
    };

    // vec![Underline] diagnostics
    let underline_fmt = if diagnostics.undercurl {
        vec![Undercurl]
    } else {
        vec![Underline]
    };

    let diagnostic_error = HighlightGroup::new().fg(Cow::Borrowed(&palette.red));
    let diagnostic_hint = HighlightGroup::new().fg(Cow::Borrowed(&palette.purple));
    let diagnostic_info = HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan));
    let diagnostic_warn = HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow));
    let diagnostic_virtual_text_error = HighlightGroup::new()
        .fg(diagnostics_error_color)
        .bg(error_bg);
    let diagnostic_virtual_text_warn = HighlightGroup::new().fg(diagnostics_warn_color).bg(warn_bg);
    let diagnostic_virtual_text_info = HighlightGroup::new().fg(diagnostics_info_color).bg(info_bg);
    let diagnostic_virtual_text_hint = HighlightGroup::new()
        .fg(diagnostics_hint_color.clone())
        .bg(hint_bg);
    let diagnostic_underline_error = HighlightGroup::new()
        .fmt(underline_fmt.clone())
        .sp(Cow::Borrowed(&palette.red));
    let diagnostic_underline_warn = HighlightGroup::new()
        .fmt(underline_fmt.clone())
        .sp(Cow::Borrowed(&palette.yellow));
    let diagnostic_underline_info = HighlightGroup::new()
        .fmt(underline_fmt.clone())
        .sp(Cow::Borrowed(&palette.blue));
    let diagnostic_underline_hint = HighlightGroup::new()
        .fmt(underline_fmt)
        .sp(Cow::Borrowed(&palette.purple));

    let lsp_plugin = HashMap::from([
        (
            "LspCxxHlGroupEnumConstant",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "LspCxxHlGroupMemberVariable",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "LspCxxHlGroupNamespace",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "LspCxxHlSkippedRegion",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "LspCxxHlSkippedRegionBeginEnd",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        ("DiagnosticError", diagnostic_error.clone()),
        ("DiagnosticHint", diagnostic_hint.clone()),
        ("DiagnosticInfo", diagnostic_info.clone()),
        ("DiagnosticWarn", diagnostic_warn.clone()),
        (
            "DiagnosticVirtualTextError",
            diagnostic_virtual_text_error.clone(),
        ),
        (
            "DiagnosticVirtualTextWarn",
            diagnostic_virtual_text_warn.clone(),
        ),
        (
            "DiagnosticVirtualTextInfo",
            diagnostic_virtual_text_info.clone(),
        ),
        (
            "DiagnosticVirtualTextHint",
            diagnostic_virtual_text_hint.clone(),
        ),
        (
            "DiagnosticUnderlineInfo",
            diagnostic_virtual_text_info.clone(),
        ),
        (
            "DiagnosticUnderlineWarn",
            diagnostic_virtual_text_warn.clone(),
        ),
        (
            "LspReferenceText",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "LspReferenceWrite",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "LspReferenceRead",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "LspCodeLens",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(code_style.comments.clone()),
        ),
        (
            "LspCodeLensSeparator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        ("LspDiagnosticsDefaultError", diagnostic_error.clone()),
        ("LspDiagnosticsDefaultHint", diagnostic_hint.clone()),
        ("LspDiagnosticsDefaultInformation", diagnostic_info.clone()),
        ("LspDiagnosticsDefaultWarning", diagnostic_warn.clone()),
        ("LspDiagnosticsUnderlineError", diagnostic_underline_error),
        ("LspDiagnosticsUnderlineHint", diagnostic_underline_hint),
        (
            "LspDiagnosticsUnderlineInformation",
            diagnostic_underline_info,
        ),
        ("LspDiagnosticsUnderlineWarning", diagnostic_underline_warn),
        (
            "LspDiagnosticsVirtualTextError",
            diagnostic_virtual_text_error,
        ),
        (
            "LspDiagnosticsVirtualTextWarning",
            diagnostic_virtual_text_warn,
        ),
        (
            "LspDiagnosticsVirtualTextInformation",
            diagnostic_virtual_text_info,
        ),
        (
            "LspDiagnosticsVirtualTextHint",
            diagnostic_virtual_text_hint,
        ),
    ]);

    let cmp_item_kind_fmt: Option<Vec<FmtType>> = if cmp_itemkind_reverse {
        vec![Reverse].into()
    } else {
        None.into()
    };
    // CMP plugin highlights
    let mut cmp_highlights = HashMap::from([
        (
            "CmpItemAbbr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "CmpItemAbbrDeprecated",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.light_grey))
                .fmt(vec![StrikeThrough]),
        ),
        (
            "CmpItemAbbrMatch",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "CmpItemAbbrMatchFuzzy",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Underline]),
        ),
        (
            "CmpItemMenu",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "CmpItemKind",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(cmp_item_kind_fmt.unwrap_or_default()),
        ),
    ]);

    let lsp_kind = HashMap::from([
        ("CmpItemKindDefault", Cow::Borrowed(&palette.purple)),
        ("CmpItemKindArray", Cow::Borrowed(&palette.yellow)),
        ("CmpItemKindBoolean", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindClass", Cow::Borrowed(&palette.yellow)),
        ("CmpItemKindColor", Cow::Borrowed(&palette.green)),
        ("CmpItemKindConstant", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindConstructor", Cow::Borrowed(&palette.blue)),
        ("CmpItemKindEnum", Cow::Borrowed(&palette.purple)),
        ("CmpItemKindEnumMember", Cow::Borrowed(&palette.yellow)),
        ("CmpItemKindEvent", Cow::Borrowed(&palette.yellow)),
        ("CmpItemKindField", Cow::Borrowed(&palette.purple)),
        ("CmpItemKindFile", Cow::Borrowed(&palette.blue)),
        ("CmpItemKindFolder", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindFunction", Cow::Borrowed(&palette.blue)),
        ("CmpItemKindInterface", Cow::Borrowed(&palette.green)),
        ("CmpItemKindKey", Cow::Borrowed(&palette.cyan)),
        ("CmpItemKindKeyword", Cow::Borrowed(&palette.cyan)),
        ("CmpItemKindMethod", Cow::Borrowed(&palette.blue)),
        ("CmpItemKindModule", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindNamespace", Cow::Borrowed(&palette.red)),
        ("CmpItemKindNull", Cow::Borrowed(&palette.grey)),
        ("CmpItemKindNumber", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindObject", Cow::Borrowed(&palette.red)),
        ("CmpItemKindOperator", Cow::Borrowed(&palette.red)),
        ("CmpItemKindPackage", Cow::Borrowed(&palette.yellow)),
        ("CmpItemKindProperty", Cow::Borrowed(&palette.cyan)),
        ("CmpItemKindReference", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindSnippet", Cow::Borrowed(&palette.red)),
        ("CmpItemKindString", Cow::Borrowed(&palette.green)),
        ("CmpItemKindStruct", Cow::Borrowed(&palette.purple)),
        ("CmpItemKindText", Cow::Borrowed(&palette.light_grey)),
        ("CmpItemKindTypeParameter", Cow::Borrowed(&palette.red)),
        ("CmpItemKindUnit", Cow::Borrowed(&palette.green)),
        ("CmpItemKindValue", Cow::Borrowed(&palette.orange)),
        ("CmpItemKindVariable", Cow::Borrowed(&palette.purple)),
    ]);

    for (kind, color) in lsp_kind.iter() {
        let lsp_kind_cmp: Option<Vec<FmtType>> = if cmp_itemkind_reverse {
            vec![Reverse].into()
        } else {
            None.into()
        };
        cmp_highlights.insert(
            kind,
            HighlightGroup::new()
                .fg(color.clone())
                .fmt(lsp_kind_cmp.unwrap_or_default()),
        );
    }

    // WhichKey plugin highlights
    let whichkey_highlights = HashMap::from([
        (
            "WhichKey",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "WhichKeyDesc",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "WhichKeyGroup",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "WhichKeySeparator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
    ]);

    // GitGutter plugin highlights
    let gitgutter_highlights = HashMap::from([
        (
            "GitGutterAdd",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "GitGutterChange",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "GitGutterDelete",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
    ]);

    // DiffView plugin highlights
    let diffview_highlights = HashMap::from([
        (
            "DiffviewFilePanelTitle",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(vec![Bold]),
        ),
        (
            "DiffviewFilePanelCounter",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "DiffviewFilePanelFileName",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "DiffviewNormal",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "DiffviewCursorLine",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "DiffviewVertSplit",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.bg3)),
        ),
        (
            "DiffviewSignColumn",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "DiffviewStatusLine",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "DiffviewStatusLineNC",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "DiffviewEndOfBuffer",
            HighlightGroup::new()
                .fg(if ending_tildes {
                    Cow::Borrowed(&palette.bg2)
                } else {
                    Cow::Borrowed(&palette.bg0)
                })
                .bg(if transparent {
                    None
                } else {
                    Cow::Borrowed(&palette.bg0).into()
                }),
        ),
        (
            "DiffviewFilePanelRootPath",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "DiffviewFilePanelPath",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "DiffviewFilePanelInsertions",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "DiffviewFilePanelDeletions",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "DiffviewStatusAdded",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "DiffviewStatusUntracked",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusModified",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusRenamed",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusCopied",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusTypeChange",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusUnmerged",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "DiffviewStatusUnknown",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "DiffviewStatusDeleted",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "DiffviewStatusBroken",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
    ]);

    // GitSigns plugin highlights
    let gitsigns_highlights = HashMap::from([
        (
            "GitSignsAdd",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "GitSignsAddLn",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "GitSignsAddNr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "GitSignsChange",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "GitSignsChangeLn",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "GitSignsChangeNr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "GitSignsDelete",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "GitSignsDeleteLn",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "GitSignsDeleteNr",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
    ]);

    // Telescope plugin highlights
    let telescope_highlights = HashMap::from([
        (
            "TelescopeBorder",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "TelescopePromptBorder",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TelescopeResultsBorder",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TelescopePreviewBorder",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "TelescopeMatching",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(vec![Bold]),
        ),
        (
            "TelescopePromptPrefix",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "TelescopeSelection",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "TelescopeSelectionCaret",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
    ]);

    let plugin_indent_line = HashMap::from([
        (
            "IndentBlankLine1",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "IndentBlankLine2",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "IndentBlankLine3",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "IndentBlankLine4",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "IndentBlankLine5",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "IndentBlankLine6",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "IndentBlanklineChar",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg1))
                .fmt(vec![NoCombine]),
        ),
        (
            "IndentBlanklineContextChar",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(vec![NoCombine]),
        ),
        (
            "IndentBlanklineContextStart",
            HighlightGroup::new()
                .sp(Cow::Borrowed(&palette.grey))
                .fmt(vec![Underline]),
        ),
        (
            "IndentBlanklineContextSpaceChar",
            HighlightGroup::new().fmt(vec![NoCombine]),
        ),
        (
            "IblIndent",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg1))
                .fmt(vec![NoCombine]),
        ),
        (
            "IblWhitespace",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(vec![NoCombine]),
        ),
        (
            "IblScope",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(vec![NoCombine]),
        ),
    ]);

    let mini = HashMap::from([
        (
            "MiniAnimateCursor",
            HighlightGroup::new().fmt(vec![Reverse, NoCombine]),
        ),
        ("MiniAnimateNormalFloat", common_normal_float.clone()),
        ("MiniClueBorder", common_float_border.clone()),
        ("MiniClueDescGroup", diagnostic_warn.clone()),
        ("MiniClueDescSingle", common_normal_float.clone()),
        ("MiniClueNextKey", diagnostic_hint.clone()),
        ("MiniClueNextKeyWithPostkeys", diagnostic_error.clone()),
        ("MiniClueSeparator", diagnostic_info.clone()),
        (
            "MiniClueTitle",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "MiniCompletionActiveParameter",
            HighlightGroup::new().fmt(vec![Underline]),
        ),
        ("MiniCursorword", HighlightGroup::new().fmt(vec![Underline])),
        (
            "MiniCursorwordCurrent",
            HighlightGroup::new().fmt(vec![Underline]),
        ),
        ("MiniDepsChangeAdded", common_added),
        ("MiniDepsChangeRemoved", common_removed),
        ("MiniDepsHint", diagnostic_hint.clone()),
        ("MiniDepsInfo", diagnostic_info.clone()),
        ("MiniDepsMsgBreaking", diagnostic_warn.clone()),
        ("MiniDepsPlaceholder", syntax_comment.clone()),
        ("MiniDepsTitle", syntax_title.clone()),
        ("MiniDepsTitleError", common_diff_delete.clone()),
        ("MiniDepsTitleSame", common_diff_text.clone()),
        ("MiniDepsTitleUpdate", common_diff_add.clone()),
        (
            "MiniDiffSignAdd",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "MiniDiffSignChange",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "MiniDiffSignDelete",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        ("MiniDiffOverAdd", common_diff_add),
        ("MiniDiffOverChange", common_diff_text),
        ("MiniDiffOverContext", common_diff_change),
        ("MiniDiffOverDelete", common_diff_delete),
        ("MiniFilesBorder", common_float_border.clone()),
        ("MiniFilesBorderModified", diagnostic_warn.clone()),
        (
            "MiniFilesCursorLine",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        ("MiniFilesDirectory", common_directory.clone()),
        (
            "MiniFilesFile",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        ("MiniFilesNormal", common_normal_float.clone()),
        (
            "MiniFilesTitle",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "MiniFilesTitleFocused",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsFixme",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsHack",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.yellow))
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsNote",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsTodo",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "MiniIconsAzure",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.bg_blue)),
        ),
        (
            "MiniIconsBlue",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "MiniIconsCyan",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "MiniIconsGreen",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "MiniIconsGrey",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "MiniIconsOrange",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "MiniIconsPurple",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "MiniIconsRed",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "MiniIconsYellow",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "MiniIndentscopeSymbol",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "MiniIndentscopePrefix",
            HighlightGroup::new().fmt(vec![NoCombine]),
        ),
        (
            "MiniJump",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Underline])
                .sp(Cow::Borrowed(&palette.purple)),
        ),
        (
            "MiniJump2dDim",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(vec![NoCombine]),
        ),
        (
            "MiniJump2dSpot",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold, NoCombine]),
        ),
        (
            "MiniJump2dSpotAhead",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .bg(Cow::Borrowed(&palette.bg0))
                .fmt(vec![NoCombine]),
        ),
        (
            "MiniJump2dSpotUnique",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.yellow))
                .fmt(vec![Bold, NoCombine]),
        ),
        ("MiniMapNormal", common_normal_float.clone()),
        ("MiniMapSymbolCount", syntax_special.clone()),
        ("MiniMapSymbolLine", syntax_title.clone()),
        ("MiniMapSymbolView", syntax_delimiter.clone()),
        ("MiniNotifyBorder", common_float_border.clone()),
        ("MiniNotifyNormal", common_normal_float.clone()),
        (
            "MiniNotifyTitle",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        ("MiniOperatorsExchangeFrom", common_inc_search.clone()),
        ("MiniPickBorder", common_float_border),
        ("MiniPickBorderBusy", diagnostic_warn),
        (
            "MiniPickBorderText",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Bold]),
        ),
        ("MiniPickIconDirectory", common_directory),
        ("MiniPickIconFile", common_normal_float.clone()),
        ("MiniPickHeader", diagnostic_hint.clone()),
        (
            "MiniPickMatchCurrent",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "MiniPickMatchMarked",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.diff_text)),
        ),
        ("MiniPickMatchRanges", diagnostic_hint.clone()),
        ("MiniPickNormal", common_normal_float.clone()),
        (
            "MiniPickPreviewLine",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.bg2)),
        ),
        ("MiniPickPreviewRegion", common_inc_search.clone()),
        ("MiniPickPrompt", diagnostic_info),
        (
            "MiniStarterCurrent",
            HighlightGroup::new().fmt(vec![NoCombine]),
        ),
        (
            "MiniStarterFooter",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.dark_red))
                .fmt(vec![Italic]),
        ),
        (
            "MiniStarterHeader",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "MiniStarterInactive",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(code_style.comments.clone()),
        ),
        (
            "MiniStarterItem",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg0)),
        ),
        (
            "MiniStarterItemBullet",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "MiniStarterItemPrefix",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "MiniStarterSection",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "MiniStarterQuery",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "MiniStatuslineDevinfo",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "MiniStatuslineFileinfo",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg2)),
        ),
        (
            "MiniStatuslineFilename",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "MiniStatuslineInactive",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg0)),
        ),
        (
            "MiniStatuslineModeCommand",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.yellow))
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeInsert",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.blue))
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeNormal",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.green))
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeOther",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeReplace",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeVisual",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "MiniSurround",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.orange)),
        ),
        ("MiniTablineCurrent", HighlightGroup::new().fmt(vec![Bold])),
        (
            "MiniTablineFill",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "MiniTablineHidden",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .bg(Cow::Borrowed(&palette.bg1)),
        ),
        (
            "MiniTablineModifiedCurrent",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(vec![Bold, Italic]),
        ),
        (
            "MiniTablineModifiedHidden",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.light_grey))
                .bg(Cow::Borrowed(&palette.bg1))
                .fmt(vec![Italic]),
        ),
        (
            "MiniTablineModifiedVisible",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.yellow))
                .bg(Cow::Borrowed(&palette.bg0))
                .fmt(vec![Italic]),
        ),
        (
            "MiniTablineTabpagesection",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.bg0))
                .bg(Cow::Borrowed(&palette.bg_yellow)),
        ),
        (
            "MiniTablineVisible",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.light_grey))
                .bg(Cow::Borrowed(&palette.bg0)),
        ),
        ("MiniTestEmphasis", HighlightGroup::new().fmt(vec![Bold])),
        (
            "MiniTestFail",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "MiniTestPass",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.green))
                .fmt(vec![Bold]),
        ),
        (
            "MiniTrailspace",
            HighlightGroup::new().bg(Cow::Borrowed(&palette.red)),
        ),
    ]);

    // Language specific highlights

    // C language highlights
    let c_highlights = HashMap::from([
        (
            "cInclude",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "cStorageClass",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "cTypedef",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "cDefine",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "cTSInclude",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "cTSConstant",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "cTSConstMacro",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "cTSOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
    ]);

    // C++ language highlights
    let cpp_highlights = HashMap::from([
        (
            "cppStatement",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "cppTSInclude",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "cppTSConstant",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "cppTSConstMacro",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "cppTSOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
    ]);

    // Markdown language highlights
    let markdown_highlights = HashMap::from([
        (
            "markdownBlockquote",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        ("markdownBold", HighlightGroup::new().fmt(vec![Bold])),
        (
            "markdownBoldDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownCode",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "markdownCodeBlock",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "markdownCodeDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "markdownH1",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "markdownH2",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "markdownH3",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(vec![Bold]),
        ),
        (
            "markdownH4",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(vec![Bold]),
        ),
        (
            "markdownH5",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(vec![Bold]),
        ),
        (
            "markdownH6",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(vec![Bold]),
        ),
        (
            "markdownHeadingDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownHeadingRule",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownId",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "markdownIdDeclaration",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        ("markdownItalic", HighlightGroup::new().fmt(vec![Italic])),
        (
            "markdownItalicDelimiter",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.grey))
                .fmt(vec![Italic]),
        ),
        (
            "markdownLinkDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownLinkText",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "markdownLinkTextDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownListMarker",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "markdownOrderedListMarker",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "markdownRule",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "markdownUrl",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.blue))
                .fmt(vec![Underline]),
        ),
        (
            "markdownUrlDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.grey)),
        ),
        (
            "markdownUrlTitleDelimiter",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
    ]);

    // PHP language highlights
    let php_highlights = HashMap::from([
        (
            "phpFunctions",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(code_style.functions.clone()),
        ),
        (
            "phpMethods",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "phpStructure",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "phpOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "phpMemberSelector",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "phpVarSelector",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(code_style.variables.clone()),
        ),
        (
            "phpIdentifier",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.orange))
                .fmt(code_style.variables.clone()),
        ),
        (
            "phpBoolean",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "phpNumber",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "phpHereDoc",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "phpNowDoc",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "phpSCKeyword",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "phpFCKeyword",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.purple))
                .fmt(code_style.keywords.clone()),
        ),
        (
            "phpRegion",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
    ]);

    // Scala language highlights
    let scala_highlights = HashMap::from([
        (
            "scalaNameDefinition",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "scalaInterpolationBoundary",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "scalaInterpolation",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "scalaTypeOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "scalaOperator",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "scalaKeywordModifier",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.red))
                .fmt(code_style.keywords.clone()),
        ),
    ]);

    // TeX language highlights
    let tex_highlights = HashMap::from([
        (
            "latexTSInclude",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "latexTSFuncMacro",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(code_style.functions.clone()),
        ),
        (
            "latexTSEnvironment",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.cyan))
                .fmt(vec![Bold]),
        ),
        (
            "latexTSEnvironmentName",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "texCmdEnv",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.cyan)),
        ),
        (
            "texEnvArgName",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "latexTSTitle",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.green)),
        ),
        (
            "latexTSType",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "latexTSMath",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "texMathZoneX",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "texMathZoneXX",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "texMathDelimZone",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.light_grey)),
        ),
        (
            "texMathDelim",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "texMathOper",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "texCmd",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "texCmdPart",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "texCmdPackage",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "texPgfType",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
    ]);

    // Vim language highlights
    let vim_highlights = HashMap::from([
        (
            "vimOption",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "vimSetEqual",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.yellow)),
        ),
        (
            "vimMap",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.purple)),
        ),
        (
            "vimMapModKey",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.orange)),
        ),
        (
            "vimNotation",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.red)),
        ),
        (
            "vimMapLhs",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.fg)),
        ),
        (
            "vimMapRhs",
            HighlightGroup::new().fg(Cow::Borrowed(&palette.blue)),
        ),
        (
            "vimVar",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.fg))
                .fmt(code_style.variables.clone()),
        ),
        (
            "vimCommentTitle",
            HighlightGroup::new()
                .fg(Cow::Borrowed(&palette.light_grey))
                .fmt(code_style.comments.clone()),
        ),
    ]);

    let hl_langs = HashMap::from([
        ("c", c_highlights),
        ("cpp", cpp_highlights),
        ("markdown", markdown_highlights),
        ("php", php_highlights),
        ("scala", scala_highlights),
        ("tex", tex_highlights),
        ("vim", vim_highlights),
    ]);

    let hl_plugins = HashMap::from([
        ("lsp", lsp_plugin),
        // Ale
        // Barbar
        ("cmp", cmp_highlights),
        // Coc
        ("whichkey", whichkey_highlights),
        ("gitgutter", gitgutter_highlights),
        // Hop
        ("diffview", diffview_highlights),
        ("gitsigns", gitsigns_highlights),
        // Neo_tree
        // Neotest
        // Nvim_tree
        ("telescope", telescope_highlights),
        // Dashboard
        // Outline
        // Navic
        // Ts_rainbow
        // Ts_rainbow2
        // Rainbow_delimiters
        ("indent_blankline", plugin_indent_line),
        ("mini", mini),
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

// Apply vim highlights
fn vim_highlights(highlights: &HashMap<&str, HighlightGroup>) -> Result<()> {
    for (group_name, group_settings) in highlights {
        let fmt = group_settings.fmt.as_ref();
        let fg = group_settings.fg.as_ref().map(|c| c.as_str());
        let bg = group_settings.bg.as_ref().map(|c| c.as_str());
        let sp = group_settings.sp.as_ref().map(|c| c.as_str());
        let opts = SetHighlightOpts::builder()
            .foreground(fg.unwrap_or("none"))
            .background(bg.unwrap_or("none"))
            .special(sp.unwrap_or("none"))
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

        if group_settings.fg.is_some() {
            opts.foreground(group_settings.fg.as_ref().unwrap().as_str());
        }
        if group_settings.bg.is_some() {
            opts.background(group_settings.bg.as_ref().unwrap().as_str());
        }
        if group_settings.sp.is_some() {
            opts.special(group_settings.sp.as_ref().unwrap().as_str());
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
    let palette = crate::palette::merge_palletes();
    let hl = default(&palette);

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
