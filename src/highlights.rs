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
    util::darken,
    OneDarkConfig, GLOBAL_CONFIG,
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

// Create a custom trait that abstracts the conversion
pub trait IntoOptionColor<'a> {
    fn into_option_color(self) -> Option<Cow<'a, Color>>;
}

// Implement for &Color
impl<'a> IntoOptionColor<'a> for &'a Color {
    fn into_option_color(self) -> Option<Cow<'a, Color>> {
        Some(Cow::Borrowed(self))
    }
}

// Implement for Option<&Color>
impl<'a> IntoOptionColor<'a> for Option<&'a Color> {
    fn into_option_color(self) -> Option<Cow<'a, Color>> {
        self.map(Cow::Borrowed)
    }
}

impl<'a> IntoOptionColor<'a> for Option<Cow<'a, Color>> {
    fn into_option_color(self) -> Option<Cow<'a, Color>> {
        self
    }
}

impl<'a> Into<Cow<'a, Color>> for &'a Color {
    fn into(self) -> Cow<'a, Color> {
        Cow::Borrowed(self)
    }
}

impl<'a> HighlightGroup<'a> {
    fn with_fg<T: IntoOptionColor<'a>>(fg: T) -> Self {
        Self {
            fg: fg.into_option_color(),
            ..Default::default()
        }
    }

    fn with_bg<T: IntoOptionColor<'a>>(bg: T) -> Self {
        Self {
            bg: bg.into_option_color(),
            ..Default::default()
        }
    }

    fn with_sp<T: IntoOptionColor<'a>>(sp: T) -> Self {
        Self {
            sp: sp.into_option_color(),
            ..Default::default()
        }
    }

    fn with_fmt<T: Into<Option<Vec<FmtType>>>>(fmt: T) -> Self {
        Self {
            fmt: fmt.into(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn fg<T: IntoOptionColor<'a>>(mut self, fg: T) -> Self {
        self.fg = fg.into_option_color();
        self
    }

    fn bg<T: IntoOptionColor<'a>>(mut self, bg: T) -> Self {
        self.bg = bg.into_option_color();
        self
    }

    fn sp<T: IntoOptionColor<'a>>(mut self, sp: T) -> Self {
        self.sp = sp.into_option_color();
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

    let common_float_border = HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1);
    let common_normal_float = HighlightGroup::with_fg(&palette.fg).bg(&palette.bg1);
    let common_added = HighlightGroup::with_fg(&palette.green);
    let common_removed = HighlightGroup::with_fg(&palette.red);
    let common_diff_delete = HighlightGroup::with_bg(&palette.diff_delete);
    let common_diff_text = HighlightGroup::with_bg(&palette.diff_text);
    let common_diff_add = HighlightGroup::with_bg(&palette.diff_add);
    let common_diff_change = HighlightGroup::with_bg(&palette.diff_change);
    let common_inc_search = HighlightGroup::with_fg(&palette.bg0).bg(&palette.orange);
    let common_directory = HighlightGroup::with_fg(&palette.blue);

    // Common highlights
    let hl_common = HashMap::from([
        (
            "Normal",
            HighlightGroup::with_fg(&palette.fg).bg(transparent.then_some(&palette.bg0)),
        ),
        (
            "Terminal",
            HighlightGroup::with_fg(&palette.fg).bg(transparent.then_some(&palette.bg0)),
        ),
        (
            "EndOfBuffer",
            HighlightGroup::with_fg(if ending_tildes {
                &palette.bg2
            } else {
                &palette.bg0
            })
            .bg(transparent.then_some(&palette.bg0)),
        ),
        (
            "FoldColumn",
            HighlightGroup::with_fg(&palette.fg).bg(transparent.then_some(&palette.bg1)),
        ),
        (
            "Folded",
            HighlightGroup::with_fg(&palette.fg).bg(transparent.then_some(&palette.bg1)),
        ),
        (
            "SignColumn",
            HighlightGroup::with_fg(&palette.fg).bg(transparent.then_some(&palette.bg0)),
        ),
        ("ToolbarLine", HighlightGroup::with_fg(&palette.fg)),
        ("Cursor", HighlightGroup::with_fmt(vec![Reverse])),
        ("vCursor", HighlightGroup::with_fmt(vec![Reverse])),
        ("iCursor", HighlightGroup::with_fmt(vec![Reverse])),
        ("lCursor", HighlightGroup::with_fmt(vec![Reverse])),
        ("CursorIM", HighlightGroup::with_fmt(vec![Reverse])),
        ("CursorColumn", HighlightGroup::with_bg(&palette.bg1)),
        ("CursorLine", HighlightGroup::with_bg(&palette.bg1)),
        ("ColorColumn", HighlightGroup::with_bg(&palette.bg1)),
        ("CursorLineNr", HighlightGroup::with_fg(&palette.fg)),
        ("LineNr", HighlightGroup::with_fg(&palette.grey)),
        (
            "Conceal",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        ("Added", common_added.clone()),
        ("Removed", common_removed.clone()),
        ("Changed", HighlightGroup::with_fg(&palette.blue)),
        ("DiffAdd", common_diff_add.clone()),
        ("DiffChange", common_diff_change.clone()),
        ("DiffDelete", common_diff_delete.clone()),
        ("DiffText", common_diff_text.clone()),
        ("DiffAdded", HighlightGroup::with_fg(&palette.green)),
        ("DiffChanged", HighlightGroup::with_fg(&palette.blue)),
        ("DiffRemoved", HighlightGroup::with_fg(&palette.red)),
        ("DiffDeleted", HighlightGroup::with_fg(&palette.red)),
        ("DiffFile", HighlightGroup::with_fg(&palette.cyan)),
        ("DiffIndexLine", HighlightGroup::with_fg(&palette.grey)),
        ("Directory", common_directory.clone()),
        (
            "ErrorMsg",
            HighlightGroup::with_fg(&palette.red).fmt(vec![Bold]),
        ),
        (
            "WarningMsg",
            HighlightGroup::with_fg(&palette.yellow).fmt(vec![Bold]),
        ),
        (
            "MoreMsg",
            HighlightGroup::with_fg(&palette.blue).fmt(vec![Bold]),
        ),
        (
            "CurSearch",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.orange),
        ),
        ("IncSearch", common_inc_search.clone()),
        (
            "Search",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.bg_yellow),
        ),
        (
            "Substitute",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.green),
        ),
        ("MatchParen", HighlightGroup::with_bg(&palette.grey)),
        ("NonText", HighlightGroup::with_fg(&palette.grey)),
        ("Whitespace", HighlightGroup::with_fg(&palette.grey)),
        ("SpecialKey", HighlightGroup::with_fg(&palette.grey)),
        (
            "Pmenu",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg1),
        ),
        ("PmenuSbar", HighlightGroup::with_bg(&palette.bg1)),
        (
            "PmenuSel",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.bg_blue),
        ),
        (
            "WildMenu",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.blue),
        ),
        ("PmenuThumb", HighlightGroup::with_bg(&palette.grey)),
        ("Question", HighlightGroup::with_fg(&palette.yellow)),
        (
            "SpellBad",
            HighlightGroup::with_sp(&palette.red).fmt(vec![Undercurl]),
        ),
        (
            "SpellCap",
            HighlightGroup::with_sp(&palette.yellow).fmt(vec![Undercurl]),
        ),
        (
            "SpellLocal",
            HighlightGroup::with_sp(&palette.blue).fmt(vec![Undercurl]),
        ),
        (
            "SpellRare",
            HighlightGroup::with_sp(&palette.purple).fmt(vec![Undercurl]),
        ),
        (
            "StatusLine",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg2),
        ),
        (
            "StatusLineTerm",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg2),
        ),
        (
            "StatusLineNC",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "StatusLineTermNC",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "TabLine",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg1),
        ),
        (
            "TabLineFill",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "TabLineSel",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.fg),
        ),
        ("WinSeparator", HighlightGroup::with_fg(&palette.bg3)),
        ("Visual", HighlightGroup::with_bg(&palette.bg3)),
        (
            "VisualNOS",
            HighlightGroup::with_bg(&palette.bg2).fmt(vec![Underline]),
        ),
        (
            "QuickFixLine",
            HighlightGroup::with_fg(&palette.blue).fmt(vec![Underline]),
        ),
        ("Debug", HighlightGroup::with_fg(&palette.yellow)),
        (
            "debugPC",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.green),
        ),
        (
            "debugBreakpoint",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.red),
        ),
        (
            "ToolbarButton",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.bg_blue),
        ),
        ("FloatBorder", common_float_border.clone()),
        ("NormalFloat", common_normal_float.clone()),
    ]);

    let syntax_comment = HighlightGroup::with_fg(&palette.grey).fmt(code_style.comments.clone());
    let syntax_title = HighlightGroup::with_fg(&palette.cyan);
    let syntax_special = HighlightGroup::with_fg(&palette.red);
    let syntax_delimiter = HighlightGroup::with_fg(&palette.light_grey);

    // Syntax highlights
    let hl_syntax = HashMap::from([
        (
            "String",
            HighlightGroup::with_fg(&palette.green).fmt(code_style.strings.clone()),
        ),
        ("Character", HighlightGroup::with_fg(&palette.orange)),
        ("Number", HighlightGroup::with_fg(&palette.orange)),
        ("Float", HighlightGroup::with_fg(&palette.orange)),
        ("Boolean", HighlightGroup::with_fg(&palette.orange)),
        ("Type", HighlightGroup::with_fg(&palette.yellow)),
        ("Structure", HighlightGroup::with_fg(&palette.yellow)),
        ("StorageClass", HighlightGroup::with_fg(&palette.yellow)),
        (
            "Identifier",
            HighlightGroup::with_fg(&palette.red).fmt(code_style.variables.clone()),
        ),
        ("Constant", HighlightGroup::with_fg(&palette.cyan)),
        ("PreProc", HighlightGroup::with_fg(&palette.purple)),
        ("PreCondit", HighlightGroup::with_fg(&palette.purple)),
        ("Include", HighlightGroup::with_fg(&palette.purple)),
        (
            "Keyword",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        ("Define", HighlightGroup::with_fg(&palette.purple)),
        ("Typedef", HighlightGroup::with_fg(&palette.yellow)),
        ("Exception", HighlightGroup::with_fg(&palette.purple)),
        (
            "Conditional",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        (
            "Repeat",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        ("Statement", HighlightGroup::with_fg(&palette.purple)),
        ("Macro", HighlightGroup::with_fg(&palette.red)),
        ("Error", HighlightGroup::with_fg(&palette.purple)),
        ("Label", HighlightGroup::with_fg(&palette.purple)),
        ("Special", syntax_special.clone()),
        ("SpecialChar", HighlightGroup::with_fg(&palette.red)),
        (
            "Function",
            HighlightGroup::with_fg(&palette.blue).fmt(code_style.functions.clone()),
        ),
        ("Operator", HighlightGroup::with_fg(&palette.purple)),
        ("Title", syntax_title.clone()),
        ("Tag", HighlightGroup::with_fg(&palette.green)),
        ("Delimiter", syntax_delimiter.clone()),
        ("Comment", syntax_comment.clone()),
        (
            "SpecialComment",
            HighlightGroup::with_fg(&palette.grey).fmt(code_style.comments.clone()),
        ),
        (
            "Todo",
            HighlightGroup::with_fg(&palette.red).fmt(code_style.comments.clone()),
        ),
    ]);

    // TreeSitter highlights
    let hl_treesitter = HashMap::from([
        ("TSAnnotation", HighlightGroup::with_fg(&palette.fg)),
        ("TSAttribute", HighlightGroup::with_fg(&palette.cyan)),
        ("TSBoolean", HighlightGroup::with_fg(&palette.orange)),
        ("TSCharacter", HighlightGroup::with_fg(&palette.orange)),
        (
            "TSComment",
            HighlightGroup::with_fg(&palette.grey).fmt(code_style.comments.clone()),
        ),
        (
            "TSConditional",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        ("TSConstant", HighlightGroup::with_fg(&palette.orange)),
        ("TSConstBuiltin", HighlightGroup::with_fg(&palette.orange)),
        ("TSConstMacro", HighlightGroup::with_fg(&palette.orange)),
        (
            "TSConstructor",
            HighlightGroup::with_fg(&palette.yellow).fmt(vec![Bold]),
        ),
        ("TSError", HighlightGroup::with_fg(&palette.fg)),
        ("TSException", HighlightGroup::with_fg(&palette.purple)),
        ("TSField", HighlightGroup::with_fg(&palette.cyan)),
        ("TSFloat", HighlightGroup::with_fg(&palette.orange)),
        (
            "TSFunction",
            HighlightGroup::with_fg(&palette.blue).fmt(code_style.functions.clone()),
        ),
        (
            "TSFuncBuiltin",
            HighlightGroup::with_fg(&palette.cyan).fmt(code_style.functions.clone()),
        ),
        (
            "TSFuncMacro",
            HighlightGroup::with_fg(&palette.cyan).fmt(code_style.functions.clone()),
        ),
        ("TSInclude", HighlightGroup::with_fg(&palette.purple)),
        (
            "TSKeyword",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        (
            "TSKeywordFunction",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.functions.clone()),
        ),
        (
            "TSKeywordOperator",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        ("TSLabel", HighlightGroup::with_fg(&palette.red)),
        (
            "TSMethod",
            HighlightGroup::with_fg(&palette.blue).fmt(code_style.functions.clone()),
        ),
        ("TSNamespace", HighlightGroup::with_fg(&palette.yellow)),
        ("TSNone", HighlightGroup::with_fg(&palette.fg)),
        ("TSNumber", HighlightGroup::with_fg(&palette.orange)),
        ("TSOperator", HighlightGroup::with_fg(&palette.fg)),
        ("TSParameter", HighlightGroup::with_fg(&palette.red)),
        ("TSParameterReference", HighlightGroup::with_fg(&palette.fg)),
        ("TSProperty", HighlightGroup::with_fg(&palette.cyan)),
        (
            "TSPunctDelimiter",
            HighlightGroup::with_fg(&palette.light_grey),
        ),
        (
            "TSPunctBracket",
            HighlightGroup::with_fg(&palette.light_grey),
        ),
        ("TSPunctSpecial", HighlightGroup::with_fg(&palette.red)),
        (
            "TSRepeat",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        (
            "TSString",
            HighlightGroup::with_fg(&palette.green).fmt(code_style.strings.clone()),
        ),
        (
            "TSStringRegex",
            HighlightGroup::with_fg(&palette.orange).fmt(code_style.strings.clone()),
        ),
        (
            "TSStringEscape",
            HighlightGroup::with_fg(&palette.red).fmt(code_style.strings.clone()),
        ),
        ("TSSymbol", HighlightGroup::with_fg(&palette.cyan)),
        ("TSTag", HighlightGroup::with_fg(&palette.purple)),
        ("TSTagDelimiter", HighlightGroup::with_fg(&palette.purple)),
        ("TSText", HighlightGroup::with_fg(&palette.fg)),
        (
            "TSStrong",
            HighlightGroup::with_fg(&palette.fg).fmt(vec![Bold]),
        ),
        (
            "TSEmphasis",
            HighlightGroup::with_fg(&palette.fg).fmt(vec![Italic]),
        ),
        (
            "TSUnderline",
            HighlightGroup::with_fg(&palette.fg).fmt(vec![Underline]),
        ),
        (
            "TSStrike",
            HighlightGroup::with_fg(&palette.fg).fmt(vec![StrikeThrough]),
        ),
        (
            "TSTitle",
            HighlightGroup::with_fg(&palette.orange).fmt(vec![Bold]),
        ),
        ("TSLiteral", HighlightGroup::with_fg(&palette.green)),
        (
            "TSURI",
            HighlightGroup::with_fg(&palette.cyan).fmt(vec![Underline]),
        ),
        ("TSMath", HighlightGroup::with_fg(&palette.fg)),
        ("TSTextReference", HighlightGroup::with_fg(&palette.blue)),
        ("TSEnvironment", HighlightGroup::with_fg(&palette.fg)),
        ("TSEnvironmentName", HighlightGroup::with_fg(&palette.fg)),
        ("TSNote", HighlightGroup::with_fg(&palette.fg)),
        ("TSWarning", HighlightGroup::with_fg(&palette.fg)),
        ("TSDanger", HighlightGroup::with_fg(&palette.fg)),
        ("TSType", HighlightGroup::with_fg(&palette.yellow)),
        ("TSTypeBuiltin", HighlightGroup::with_fg(&palette.orange)),
        (
            "TSVariable",
            HighlightGroup::with_fg(&palette.fg).fmt(code_style.variables.clone()),
        ),
        (
            "TSVariableBuiltin",
            HighlightGroup::with_fg(&palette.red).fmt(code_style.variables.clone()),
        ),
    ]);

    // LSP plugin highlights
    let diagnostics_error_color = if diagnostics.darker {
        &palette.dark_red
    } else {
        &palette.red
    };

    let diagnostics_hint_color = if diagnostics.darker {
        &palette.dark_purple
    } else {
        &palette.purple
    };

    let diagnostics_warn_color = if diagnostics.darker {
        &palette.dark_yellow
    } else {
        &palette.yellow
    };

    let diagnostics_info_color = if diagnostics.darker {
        &palette.dark_cyan
    } else {
        &palette.cyan
    };

    let error_bg = diagnostics
        .background
        .then(|| darken(diagnostics_error_color, &0.1, &palette.bg0));
    let warn_bg = diagnostics
        .background
        .then(|| darken(diagnostics_warn_color, &0.1, &palette.bg0));
    let info_bg = diagnostics
        .background
        .then(|| darken(diagnostics_info_color, &0.1, &palette.bg0));
    let hint_bg = diagnostics
        .background
        .then(|| darken(diagnostics_hint_color, &0.1, &palette.bg0));

    // vec![Underline] diagnostics
    let underline_fmt = if diagnostics.undercurl {
        vec![Undercurl]
    } else {
        vec![Underline]
    };

    let diagnostic_error = HighlightGroup::with_fg(&palette.red);
    let diagnostic_hint = HighlightGroup::with_fg(&palette.purple);
    let diagnostic_info = HighlightGroup::with_fg(&palette.cyan);
    let diagnostic_warn = HighlightGroup::with_fg(&palette.yellow);
    let diagnostic_virtual_text_error =
        HighlightGroup::with_fg(diagnostics_error_color).bg(error_bg);
    let diagnostic_virtual_text_warn = HighlightGroup::with_fg(diagnostics_warn_color).bg(warn_bg);
    let diagnostic_virtual_text_info = HighlightGroup::with_fg(diagnostics_info_color).bg(info_bg);
    let diagnostic_virtual_text_hint = HighlightGroup::with_fg(diagnostics_hint_color).bg(hint_bg);
    let diagnostic_underline_error =
        HighlightGroup::with_fmt(underline_fmt.clone()).sp(&palette.red);
    let diagnostic_underline_warn =
        HighlightGroup::with_fmt(underline_fmt.clone()).sp(&palette.yellow);
    let diagnostic_underline_info =
        HighlightGroup::with_fmt(underline_fmt.clone()).sp(&palette.blue);
    let diagnostic_underline_hint = HighlightGroup::with_fmt(underline_fmt).sp(&palette.purple);

    let lsp_plugin = HashMap::from([
        (
            "LspCxxHlGroupEnumConstant",
            HighlightGroup::with_fg(&palette.orange),
        ),
        (
            "LspCxxHlGroupMemberVariable",
            HighlightGroup::with_fg(&palette.orange),
        ),
        (
            "LspCxxHlGroupNamespace",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "LspCxxHlSkippedRegion",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "LspCxxHlSkippedRegionBeginEnd",
            HighlightGroup::with_fg(&palette.red),
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
        ("LspReferenceText", HighlightGroup::with_bg(&palette.bg2)),
        ("LspReferenceWrite", HighlightGroup::with_bg(&palette.bg2)),
        ("LspReferenceRead", HighlightGroup::with_bg(&palette.bg2)),
        (
            "LspCodeLens",
            HighlightGroup::with_fg(&palette.grey).fmt(code_style.comments.clone()),
        ),
        (
            "LspCodeLensSeparator",
            HighlightGroup::with_fg(&palette.grey),
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

    let cmp_item_kind_fmt: Option<Vec<FmtType>> = cmp_itemkind_reverse.then_some(vec![Reverse]);

    // CMP plugin highlights
    let mut cmp_highlights = HashMap::from([
        ("CmpItemAbbr", HighlightGroup::with_fg(&palette.fg)),
        (
            "CmpItemAbbrDeprecated",
            HighlightGroup::with_fg(&palette.light_grey).fmt(vec![StrikeThrough]),
        ),
        ("CmpItemAbbrMatch", HighlightGroup::with_fg(&palette.cyan)),
        (
            "CmpItemAbbrMatchFuzzy",
            HighlightGroup::with_fg(&palette.cyan).fmt(vec![Underline]),
        ),
        ("CmpItemMenu", HighlightGroup::with_fg(&palette.light_grey)),
        (
            "CmpItemKind",
            HighlightGroup::with_fg(&palette.purple).fmt(cmp_item_kind_fmt.unwrap_or_default()),
        ),
    ]);

    let lsp_kind = HashMap::from([
        ("CmpItemKindDefault", &palette.purple),
        ("CmpItemKindArray", &palette.yellow),
        ("CmpItemKindBoolean", &palette.orange),
        ("CmpItemKindClass", &palette.yellow),
        ("CmpItemKindColor", &palette.green),
        ("CmpItemKindConstant", &palette.orange),
        ("CmpItemKindConstructor", &palette.blue),
        ("CmpItemKindEnum", &palette.purple),
        ("CmpItemKindEnumMember", &palette.yellow),
        ("CmpItemKindEvent", &palette.yellow),
        ("CmpItemKindField", &palette.purple),
        ("CmpItemKindFile", &palette.blue),
        ("CmpItemKindFolder", &palette.orange),
        ("CmpItemKindFunction", &palette.blue),
        ("CmpItemKindInterface", &palette.green),
        ("CmpItemKindKey", &palette.cyan),
        ("CmpItemKindKeyword", &palette.cyan),
        ("CmpItemKindMethod", &palette.blue),
        ("CmpItemKindModule", &palette.orange),
        ("CmpItemKindNamespace", &palette.red),
        ("CmpItemKindNull", &palette.grey),
        ("CmpItemKindNumber", &palette.orange),
        ("CmpItemKindObject", &palette.red),
        ("CmpItemKindOperator", &palette.red),
        ("CmpItemKindPackage", &palette.yellow),
        ("CmpItemKindProperty", &palette.cyan),
        ("CmpItemKindReference", &palette.orange),
        ("CmpItemKindSnippet", &palette.red),
        ("CmpItemKindString", &palette.green),
        ("CmpItemKindStruct", &palette.purple),
        ("CmpItemKindText", &palette.light_grey),
        ("CmpItemKindTypeParameter", &palette.red),
        ("CmpItemKindUnit", &palette.green),
        ("CmpItemKindValue", &palette.orange),
        ("CmpItemKindVariable", &palette.purple),
    ]);

    for (kind, color) in lsp_kind.iter() {
        let lsp_kind_cmp: Option<Vec<FmtType>> = cmp_itemkind_reverse.then_some(vec![Reverse]);
        cmp_highlights.insert(
            kind,
            HighlightGroup::with_fg(*color).fmt(lsp_kind_cmp.unwrap_or_default()),
        );
    }

    // WhichKey plugin highlights
    let whichkey_highlights = HashMap::from([
        ("WhichKey", HighlightGroup::with_fg(&palette.red)),
        ("WhichKeyDesc", HighlightGroup::with_fg(&palette.blue)),
        ("WhichKeyGroup", HighlightGroup::with_fg(&palette.orange)),
        ("WhichKeySeparator", HighlightGroup::with_fg(&palette.green)),
    ]);

    // GitGutter plugin highlights
    let gitgutter_highlights = HashMap::from([
        ("GitGutterAdd", HighlightGroup::with_fg(&palette.green)),
        ("GitGutterChange", HighlightGroup::with_fg(&palette.blue)),
        ("GitGutterDelete", HighlightGroup::with_fg(&palette.red)),
    ]);

    // DiffView plugin highlights
    let diffview_highlights = HashMap::from([
        (
            "DiffviewFilePanelTitle",
            HighlightGroup::with_fg(&palette.blue).fmt(vec![Bold]),
        ),
        (
            "DiffviewFilePanelCounter",
            HighlightGroup::with_fg(&palette.purple).fmt(vec![Bold]),
        ),
        (
            "DiffviewFilePanelFileName",
            HighlightGroup::with_fg(&palette.fg),
        ),
        (
            "DiffviewNormal",
            HighlightGroup::with_fg(&palette.fg)
                .bg(transparent.then_some(Cow::Borrowed(&palette.bg0))),
        ),
        ("DiffviewCursorLine", HighlightGroup::with_bg(&palette.bg1)),
        ("DiffviewVertSplit", HighlightGroup::with_fg(&palette.bg3)),
        (
            "DiffviewSignColumn",
            HighlightGroup::with_fg(&palette.fg)
                .bg(transparent.then_some(Cow::Borrowed(&palette.bg0))),
        ),
        (
            "DiffviewStatusLine",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg2),
        ),
        (
            "DiffviewStatusLineNC",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "DiffviewEndOfBuffer",
            HighlightGroup::with_fg(if ending_tildes {
                &palette.bg2
            } else {
                &palette.bg0
            })
            .bg(transparent.then_some(&palette.bg0)),
        ),
        (
            "DiffviewFilePanelRootPath",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "DiffviewFilePanelPath",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "DiffviewFilePanelInsertions",
            HighlightGroup::with_fg(&palette.green),
        ),
        (
            "DiffviewFilePanelDeletions",
            HighlightGroup::with_fg(&palette.red),
        ),
        (
            "DiffviewStatusAdded",
            HighlightGroup::with_fg(&palette.green),
        ),
        (
            "DiffviewStatusUntracked",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusModified",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusRenamed",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusCopied",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusTypeChange",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusUnmerged",
            HighlightGroup::with_fg(&palette.blue),
        ),
        (
            "DiffviewStatusUnknown",
            HighlightGroup::with_fg(&palette.red),
        ),
        (
            "DiffviewStatusDeleted",
            HighlightGroup::with_fg(&palette.red),
        ),
        (
            "DiffviewStatusBroken",
            HighlightGroup::with_fg(&palette.red),
        ),
    ]);

    // GitSigns plugin highlights
    let gitsigns_highlights = HashMap::from([
        ("GitSignsAdd", HighlightGroup::with_fg(&palette.green)),
        ("GitSignsAddLn", HighlightGroup::with_fg(&palette.green)),
        ("GitSignsAddNr", HighlightGroup::with_fg(&palette.green)),
        ("GitSignsChange", HighlightGroup::with_fg(&palette.blue)),
        ("GitSignsChangeLn", HighlightGroup::with_fg(&palette.blue)),
        ("GitSignsChangeNr", HighlightGroup::with_fg(&palette.blue)),
        ("GitSignsDelete", HighlightGroup::with_fg(&palette.red)),
        ("GitSignsDeleteLn", HighlightGroup::with_fg(&palette.red)),
        ("GitSignsDeleteNr", HighlightGroup::with_fg(&palette.red)),
    ]);

    // Telescope plugin highlights
    let telescope_highlights = HashMap::from([
        ("TelescopeBorder", HighlightGroup::with_fg(&palette.red)),
        (
            "TelescopePromptBorder",
            HighlightGroup::with_fg(&palette.cyan),
        ),
        (
            "TelescopeResultsBorder",
            HighlightGroup::with_fg(&palette.cyan),
        ),
        (
            "TelescopePreviewBorder",
            HighlightGroup::with_fg(&palette.cyan),
        ),
        (
            "TelescopeMatching",
            HighlightGroup::with_fg(&palette.orange).fmt(vec![Bold]),
        ),
        (
            "TelescopePromptPrefix",
            HighlightGroup::with_fg(&palette.green),
        ),
        ("TelescopeSelection", HighlightGroup::with_bg(&palette.bg2)),
        (
            "TelescopeSelectionCaret",
            HighlightGroup::with_fg(&palette.yellow),
        ),
    ]);

    let plugin_indent_line = HashMap::from([
        ("IndentBlankLine1", HighlightGroup::with_fg(&palette.blue)),
        ("IndentBlankLine2", HighlightGroup::with_fg(&palette.green)),
        ("IndentBlankLine3", HighlightGroup::with_fg(&palette.cyan)),
        (
            "IndentBlankLine4",
            HighlightGroup::with_fg(&palette.light_grey),
        ),
        ("IndentBlankLine5", HighlightGroup::with_fg(&palette.purple)),
        ("IndentBlankLine6", HighlightGroup::with_fg(&palette.red)),
        (
            "IndentBlanklineChar",
            HighlightGroup::with_fg(&palette.bg1).fmt(vec![NoCombine]),
        ),
        (
            "IndentBlanklineContextChar",
            HighlightGroup::with_fg(&palette.grey).fmt(vec![NoCombine]),
        ),
        (
            "IndentBlanklineContextStart",
            HighlightGroup::with_sp(&palette.grey).fmt(vec![Underline]),
        ),
        (
            "IndentBlanklineContextSpaceChar",
            HighlightGroup::with_fmt(vec![NoCombine]),
        ),
        (
            "IblIndent",
            HighlightGroup::with_fg(&palette.bg1).fmt(vec![NoCombine]),
        ),
        (
            "IblWhitespace",
            HighlightGroup::with_fg(&palette.grey).fmt(vec![NoCombine]),
        ),
        (
            "IblScope",
            HighlightGroup::with_fg(&palette.grey).fmt(vec![NoCombine]),
        ),
    ]);

    let mini = HashMap::from([
        (
            "MiniAnimateCursor",
            HighlightGroup::with_fmt(vec![Reverse, NoCombine]),
        ),
        ("MiniAnimateNormalFloat", common_normal_float.clone()),
        ("MiniClueBorder", common_float_border.clone()),
        ("MiniClueDescGroup", diagnostic_warn.clone()),
        ("MiniClueDescSingle", common_normal_float.clone()),
        ("MiniClueNextKey", diagnostic_hint.clone()),
        ("MiniClueNextKeyWithPostkeys", diagnostic_error.clone()),
        ("MiniClueSeparator", diagnostic_info.clone()),
        ("MiniClueTitle", HighlightGroup::with_fg(&palette.cyan)),
        (
            "MiniCompletionActiveParameter",
            HighlightGroup::with_fmt(vec![Underline]),
        ),
        ("MiniCursorword", HighlightGroup::with_fmt(vec![Underline])),
        (
            "MiniCursorwordCurrent",
            HighlightGroup::with_fmt(vec![Underline]),
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
        ("MiniDiffSignAdd", HighlightGroup::with_fg(&palette.green)),
        ("MiniDiffSignChange", HighlightGroup::with_fg(&palette.blue)),
        ("MiniDiffSignDelete", HighlightGroup::with_fg(&palette.red)),
        ("MiniDiffOverAdd", common_diff_add),
        ("MiniDiffOverChange", common_diff_text),
        ("MiniDiffOverContext", common_diff_change),
        ("MiniDiffOverDelete", common_diff_delete),
        ("MiniFilesBorder", common_float_border.clone()),
        ("MiniFilesBorderModified", diagnostic_warn.clone()),
        ("MiniFilesCursorLine", HighlightGroup::with_bg(&palette.bg2)),
        ("MiniFilesDirectory", common_directory.clone()),
        ("MiniFilesFile", HighlightGroup::with_fg(&palette.fg)),
        ("MiniFilesNormal", common_normal_float.clone()),
        ("MiniFilesTitle", HighlightGroup::with_fg(&palette.cyan)),
        (
            "MiniFilesTitleFocused",
            HighlightGroup::with_fg(&palette.cyan).fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsFixme",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.red)
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsHack",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.yellow)
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsNote",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.cyan)
                .fmt(vec![Bold]),
        ),
        (
            "MiniHipatternsTodo",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.purple)
                .fmt(vec![Bold]),
        ),
        ("MiniIconsAzure", HighlightGroup::with_fg(&palette.bg_blue)),
        ("MiniIconsBlue", HighlightGroup::with_fg(&palette.blue)),
        ("MiniIconsCyan", HighlightGroup::with_fg(&palette.cyan)),
        ("MiniIconsGreen", HighlightGroup::with_fg(&palette.green)),
        ("MiniIconsGrey", HighlightGroup::with_fg(&palette.fg)),
        ("MiniIconsOrange", HighlightGroup::with_fg(&palette.orange)),
        ("MiniIconsPurple", HighlightGroup::with_fg(&palette.purple)),
        ("MiniIconsRed", HighlightGroup::with_fg(&palette.red)),
        ("MiniIconsYellow", HighlightGroup::with_fg(&palette.yellow)),
        (
            "MiniIndentscopeSymbol",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "MiniIndentscopePrefix",
            HighlightGroup::with_fmt(vec![NoCombine]),
        ),
        (
            "MiniJump",
            HighlightGroup::with_fg(&palette.purple)
                .fmt(vec![Underline])
                .sp(&palette.purple),
        ),
        (
            "MiniJump2dDim",
            HighlightGroup::with_fg(&palette.grey).fmt(vec![NoCombine]),
        ),
        (
            "MiniJump2dSpot",
            HighlightGroup::with_fg(&palette.red).fmt(vec![Bold, NoCombine]),
        ),
        (
            "MiniJump2dSpotAhead",
            HighlightGroup::with_fg(&palette.cyan)
                .bg(&palette.bg0)
                .fmt(vec![NoCombine]),
        ),
        (
            "MiniJump2dSpotUnique",
            HighlightGroup::with_fg(&palette.yellow).fmt(vec![Bold, NoCombine]),
        ),
        ("MiniMapNormal", common_normal_float.clone()),
        ("MiniMapSymbolCount", syntax_special.clone()),
        ("MiniMapSymbolLine", syntax_title.clone()),
        ("MiniMapSymbolView", syntax_delimiter.clone()),
        ("MiniNotifyBorder", common_float_border.clone()),
        ("MiniNotifyNormal", common_normal_float.clone()),
        ("MiniNotifyTitle", HighlightGroup::with_fg(&palette.cyan)),
        ("MiniOperatorsExchangeFrom", common_inc_search.clone()),
        ("MiniPickBorder", common_float_border),
        ("MiniPickBorderBusy", diagnostic_warn),
        (
            "MiniPickBorderText",
            HighlightGroup::with_fg(&palette.cyan).fmt(vec![Bold]),
        ),
        ("MiniPickIconDirectory", common_directory),
        ("MiniPickIconFile", common_normal_float.clone()),
        ("MiniPickHeader", diagnostic_hint.clone()),
        (
            "MiniPickMatchCurrent",
            HighlightGroup::with_bg(&palette.bg2),
        ),
        (
            "MiniPickMatchMarked",
            HighlightGroup::with_bg(&palette.diff_text),
        ),
        ("MiniPickMatchRanges", diagnostic_hint.clone()),
        ("MiniPickNormal", common_normal_float.clone()),
        ("MiniPickPreviewLine", HighlightGroup::with_bg(&palette.bg2)),
        ("MiniPickPreviewRegion", common_inc_search.clone()),
        ("MiniPickPrompt", diagnostic_info),
        (
            "MiniStarterCurrent",
            HighlightGroup::with_fmt(vec![NoCombine]),
        ),
        (
            "MiniStarterFooter",
            HighlightGroup::with_fg(&palette.dark_red).fmt(vec![Italic]),
        ),
        (
            "MiniStarterHeader",
            HighlightGroup::with_fg(&palette.yellow),
        ),
        (
            "MiniStarterInactive",
            HighlightGroup::with_fg(&palette.grey).fmt(code_style.comments.clone()),
        ),
        (
            "MiniStarterItem",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg0),
        ),
        (
            "MiniStarterItemBullet",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "MiniStarterItemPrefix",
            HighlightGroup::with_fg(&palette.yellow),
        ),
        (
            "MiniStarterSection",
            HighlightGroup::with_fg(&palette.light_grey),
        ),
        ("MiniStarterQuery", HighlightGroup::with_fg(&palette.cyan)),
        (
            "MiniStatuslineDevinfo",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg2),
        ),
        (
            "MiniStatuslineFileinfo",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg2),
        ),
        (
            "MiniStatuslineFilename",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "MiniStatuslineInactive",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg0),
        ),
        (
            "MiniStatuslineModeCommand",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.yellow)
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeInsert",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.blue)
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeNormal",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.green)
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeOther",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.cyan)
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeReplace",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.red)
                .fmt(vec![Bold]),
        ),
        (
            "MiniStatuslineModeVisual",
            HighlightGroup::with_fg(&palette.bg0)
                .bg(&palette.purple)
                .fmt(vec![Bold]),
        ),
        (
            "MiniSurround",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.orange),
        ),
        ("MiniTablineCurrent", HighlightGroup::with_fmt(vec![Bold])),
        (
            "MiniTablineFill",
            HighlightGroup::with_fg(&palette.grey).bg(&palette.bg1),
        ),
        (
            "MiniTablineHidden",
            HighlightGroup::with_fg(&palette.fg).bg(&palette.bg1),
        ),
        (
            "MiniTablineModifiedCurrent",
            HighlightGroup::with_fg(&palette.orange).fmt(vec![Bold, Italic]),
        ),
        (
            "MiniTablineModifiedHidden",
            HighlightGroup::with_fg(&palette.light_grey)
                .bg(&palette.bg1)
                .fmt(vec![Italic]),
        ),
        (
            "MiniTablineModifiedVisible",
            HighlightGroup::with_fg(&palette.yellow)
                .bg(&palette.bg0)
                .fmt(vec![Italic]),
        ),
        (
            "MiniTablineTabpagesection",
            HighlightGroup::with_fg(&palette.bg0).bg(&palette.bg_yellow),
        ),
        (
            "MiniTablineVisible",
            HighlightGroup::with_fg(&palette.light_grey).bg(&palette.bg0),
        ),
        ("MiniTestEmphasis", HighlightGroup::with_fmt(vec![Bold])),
        (
            "MiniTestFail",
            HighlightGroup::with_fg(&palette.red).fmt(vec![Bold]),
        ),
        (
            "MiniTestPass",
            HighlightGroup::with_fg(&palette.green).fmt(vec![Bold]),
        ),
        ("MiniTrailspace", HighlightGroup::with_bg(&palette.red)),
    ]);

    // Language specific highlights

    // C language highlights
    let c_highlights = HashMap::from([
        ("cInclude", HighlightGroup::with_fg(&palette.blue)),
        ("cStorageClass", HighlightGroup::with_fg(&palette.purple)),
        ("cTypedef", HighlightGroup::with_fg(&palette.purple)),
        ("cDefine", HighlightGroup::with_fg(&palette.cyan)),
        ("cTSInclude", HighlightGroup::with_fg(&palette.blue)),
        ("cTSConstant", HighlightGroup::with_fg(&palette.cyan)),
        ("cTSConstMacro", HighlightGroup::with_fg(&palette.purple)),
        ("cTSOperator", HighlightGroup::with_fg(&palette.purple)),
    ]);

    // C++ language highlights
    let cpp_highlights = HashMap::from([
        (
            "cppStatement",
            HighlightGroup::with_fg(&palette.purple).fmt(vec![Bold]),
        ),
        ("cppTSInclude", HighlightGroup::with_fg(&palette.blue)),
        ("cppTSConstant", HighlightGroup::with_fg(&palette.cyan)),
        ("cppTSConstMacro", HighlightGroup::with_fg(&palette.purple)),
        ("cppTSOperator", HighlightGroup::with_fg(&palette.purple)),
    ]);

    // Markdown language highlights
    let markdown_highlights = HashMap::from([
        ("markdownBlockquote", HighlightGroup::with_fg(&palette.grey)),
        ("markdownBold", HighlightGroup::with_fmt(vec![Bold])),
        (
            "markdownBoldDelimiter",
            HighlightGroup::with_fg(&palette.grey),
        ),
        ("markdownCode", HighlightGroup::with_fg(&palette.green)),
        ("markdownCodeBlock", HighlightGroup::with_fg(&palette.green)),
        (
            "markdownCodeDelimiter",
            HighlightGroup::with_fg(&palette.yellow),
        ),
        (
            "markdownH1",
            HighlightGroup::with_fg(&palette.red).fmt(vec![Bold]),
        ),
        (
            "markdownH2",
            HighlightGroup::with_fg(&palette.purple).fmt(vec![Bold]),
        ),
        (
            "markdownH3",
            HighlightGroup::with_fg(&palette.orange).fmt(vec![Bold]),
        ),
        (
            "markdownH4",
            HighlightGroup::with_fg(&palette.red).fmt(vec![Bold]),
        ),
        (
            "markdownH5",
            HighlightGroup::with_fg(&palette.purple).fmt(vec![Bold]),
        ),
        (
            "markdownH6",
            HighlightGroup::with_fg(&palette.orange).fmt(vec![Bold]),
        ),
        (
            "markdownHeadingDelimiter",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "markdownHeadingRule",
            HighlightGroup::with_fg(&palette.grey),
        ),
        ("markdownId", HighlightGroup::with_fg(&palette.yellow)),
        (
            "markdownIdDeclaration",
            HighlightGroup::with_fg(&palette.red),
        ),
        ("markdownItalic", HighlightGroup::with_fmt(vec![Italic])),
        (
            "markdownItalicDelimiter",
            HighlightGroup::with_fg(&palette.grey).fmt(vec![Italic]),
        ),
        (
            "markdownLinkDelimiter",
            HighlightGroup::with_fg(&palette.grey),
        ),
        ("markdownLinkText", HighlightGroup::with_fg(&palette.red)),
        (
            "markdownLinkTextDelimiter",
            HighlightGroup::with_fg(&palette.grey),
        ),
        ("markdownListMarker", HighlightGroup::with_fg(&palette.red)),
        (
            "markdownOrderedListMarker",
            HighlightGroup::with_fg(&palette.red),
        ),
        ("markdownRule", HighlightGroup::with_fg(&palette.purple)),
        (
            "markdownUrl",
            HighlightGroup::with_fg(&palette.blue).fmt(vec![Underline]),
        ),
        (
            "markdownUrlDelimiter",
            HighlightGroup::with_fg(&palette.grey),
        ),
        (
            "markdownUrlTitleDelimiter",
            HighlightGroup::with_fg(&palette.green),
        ),
    ]);

    // PHP language highlights
    let php_highlights = HashMap::from([
        (
            "phpFunctions",
            HighlightGroup::with_fg(&palette.fg).fmt(code_style.functions.clone()),
        ),
        ("phpMethods", HighlightGroup::with_fg(&palette.cyan)),
        ("phpStructure", HighlightGroup::with_fg(&palette.purple)),
        ("phpOperator", HighlightGroup::with_fg(&palette.purple)),
        ("phpMemberSelector", HighlightGroup::with_fg(&palette.fg)),
        (
            "phpVarSelector",
            HighlightGroup::with_fg(&palette.orange).fmt(code_style.variables.clone()),
        ),
        (
            "phpIdentifier",
            HighlightGroup::with_fg(&palette.orange).fmt(code_style.variables.clone()),
        ),
        ("phpBoolean", HighlightGroup::with_fg(&palette.cyan)),
        ("phpNumber", HighlightGroup::with_fg(&palette.orange)),
        ("phpHereDoc", HighlightGroup::with_fg(&palette.green)),
        ("phpNowDoc", HighlightGroup::with_fg(&palette.green)),
        (
            "phpSCKeyword",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        (
            "phpFCKeyword",
            HighlightGroup::with_fg(&palette.purple).fmt(code_style.keywords.clone()),
        ),
        ("phpRegion", HighlightGroup::with_fg(&palette.blue)),
    ]);

    // Scala language highlights
    let scala_highlights = HashMap::from([
        ("scalaNameDefinition", HighlightGroup::with_fg(&palette.fg)),
        (
            "scalaInterpolationBoundary",
            HighlightGroup::with_fg(&palette.purple),
        ),
        (
            "scalaInterpolation",
            HighlightGroup::with_fg(&palette.purple),
        ),
        ("scalaTypeOperator", HighlightGroup::with_fg(&palette.red)),
        ("scalaOperator", HighlightGroup::with_fg(&palette.red)),
        (
            "scalaKeywordModifier",
            HighlightGroup::with_fg(&palette.red).fmt(code_style.keywords.clone()),
        ),
    ]);

    // TeX language highlights
    let tex_highlights = HashMap::from([
        ("latexTSInclude", HighlightGroup::with_fg(&palette.blue)),
        (
            "latexTSFuncMacro",
            HighlightGroup::with_fg(&palette.fg).fmt(code_style.functions.clone()),
        ),
        (
            "latexTSEnvironment",
            HighlightGroup::with_fg(&palette.cyan).fmt(vec![Bold]),
        ),
        (
            "latexTSEnvironmentName",
            HighlightGroup::with_fg(&palette.yellow),
        ),
        ("texCmdEnv", HighlightGroup::with_fg(&palette.cyan)),
        ("texEnvArgName", HighlightGroup::with_fg(&palette.yellow)),
        ("latexTSTitle", HighlightGroup::with_fg(&palette.green)),
        ("latexTSType", HighlightGroup::with_fg(&palette.blue)),
        ("latexTSMath", HighlightGroup::with_fg(&palette.orange)),
        ("texMathZoneX", HighlightGroup::with_fg(&palette.orange)),
        ("texMathZoneXX", HighlightGroup::with_fg(&palette.orange)),
        (
            "texMathDelimZone",
            HighlightGroup::with_fg(&palette.light_grey),
        ),
        ("texMathDelim", HighlightGroup::with_fg(&palette.purple)),
        ("texMathOper", HighlightGroup::with_fg(&palette.red)),
        ("texCmd", HighlightGroup::with_fg(&palette.purple)),
        ("texCmdPart", HighlightGroup::with_fg(&palette.blue)),
        ("texCmdPackage", HighlightGroup::with_fg(&palette.blue)),
        ("texPgfType", HighlightGroup::with_fg(&palette.yellow)),
    ]);

    // Vim language highlights
    let vim_highlights = HashMap::from([
        ("vimOption", HighlightGroup::with_fg(&palette.red)),
        ("vimSetEqual", HighlightGroup::with_fg(&palette.yellow)),
        ("vimMap", HighlightGroup::with_fg(&palette.purple)),
        ("vimMapModKey", HighlightGroup::with_fg(&palette.orange)),
        ("vimNotation", HighlightGroup::with_fg(&palette.red)),
        ("vimMapLhs", HighlightGroup::with_fg(&palette.fg)),
        ("vimMapRhs", HighlightGroup::with_fg(&palette.blue)),
        (
            "vimVar",
            HighlightGroup::with_fg(&palette.fg).fmt(code_style.variables.clone()),
        ),
        (
            "vimCommentTitle",
            HighlightGroup::with_fg(&palette.light_grey).fmt(code_style.comments.clone()),
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
