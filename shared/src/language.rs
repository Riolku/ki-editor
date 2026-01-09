use grammar::grammar::GrammarConfiguration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tree_sitter::Query;

pub(crate) use crate::process_command::ProcessCommand;
use crate::{formatter::Formatter, ts_highlight_query::get_highlight_query};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
/// As defined by the LSP protocol.
/// See sections below https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#range
pub struct LanguageId(String);

impl std::fmt::Display for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl LanguageId {
    pub fn new(id: &'static str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}
impl Command {
    pub fn new(command: &'static str, arguments: &[&'static str]) -> Self {
        Self {
            command: command.to_string(),
            arguments: arguments.iter().map(|arg| arg.to_string()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Language {
    #[schemars(example = &["ts", "tsx"])]
    pub(crate) extensions: Vec<String>,
    /// For files without extensions.
    #[schemars(example = &["Dockerfile"])]
    pub(crate) file_names: Vec<String>,
    pub(crate) lsp_language_id: Option<LanguageId>,
    pub(crate) lsp_command: Option<LspCommand>,
    pub(crate) tree_sitter_grammar_config: Option<GrammarConfig>,
    /// The formatter command will receive the content from STDIN
    /// and is expected to return the formatted output to STDOUT.
    pub(crate) formatter: Option<Command>,
    #[schemars(example = "//")]
    pub(crate) line_comment_prefix: Option<String>,
    #[schemars(example = ("/*", "*/"))]
    pub(crate) block_comment_affixes: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum CargoLinkedTreesitterLanguage {
    Typescript,
    TSX,
    Python,
    Julia,
    Scheme,
    OCaml,
    OCamlInterface,
    Rust,
    Graphql,
    Javascript,
    JSX,
    Svelte,
    JSON,
    YAML,
    HTML,
    XML,
    Zig,
    Markdown,
    Go,
    Lua,
    Gleam,
    Bash,
    C,
    CPP,
    CSS,
    Ruby,
    Nix,
    Fish,
    Diff,
    Elixir,
    Swift,
    Heex,
    Toml,
    KiQuickfix,
    Haskell,
    Hcl,
}

impl CargoLinkedTreesitterLanguage {
    pub(crate) fn to_tree_sitter_language(&self) -> tree_sitter::Language {
        match self {
            Self::Typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::TSX => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::Julia => tree_sitter_julia::LANGUAGE.into(),
            Self::Scheme => tree_sitter_scheme::LANGUAGE.into(),
            Self::OCaml => tree_sitter_ocaml::LANGUAGE_OCAML.into(),
            Self::OCamlInterface => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into(),
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Graphql => tree_sitter_graphql::LANGUAGE.into(),
            Self::Javascript => tree_sitter_javascript::LANGUAGE.into(),
            Self::JSX => tree_sitter_javascript::LANGUAGE.into(),
            Self::Svelte => tree_sitter_svelte_ng::LANGUAGE.into(),
            Self::JSON => tree_sitter_json::LANGUAGE.into(),
            Self::YAML => tree_sitter_yaml::LANGUAGE.into(),
            Self::HTML => tree_sitter_html::LANGUAGE.into(),
            Self::Haskell => tree_sitter_haskell::LANGUAGE.into(),
            Self::XML => tree_sitter_xml::LANGUAGE_XML.into(),
            Self::Zig => tree_sitter_zig::LANGUAGE.into(),
            Self::Markdown => tree_sitter_md::LANGUAGE.into(),
            Self::Go => tree_sitter_go::LANGUAGE.into(),
            Self::Lua => tree_sitter_lua::LANGUAGE.into(),
            Self::Gleam => tree_sitter_gleam::LANGUAGE.into(),
            Self::Bash => tree_sitter_bash::LANGUAGE.into(),
            Self::C => tree_sitter_c::LANGUAGE.into(),
            Self::CPP => tree_sitter_cpp::LANGUAGE.into(),
            Self::CSS => tree_sitter_css::LANGUAGE.into(),
            Self::Ruby => tree_sitter_ruby::LANGUAGE.into(),
            Self::Nix => tree_sitter_nix::LANGUAGE.into(),
            Self::Fish => tree_sitter_fish::language(),
            Self::Diff => tree_sitter_diff::LANGUAGE.into(),
            Self::Elixir => tree_sitter_elixir::LANGUAGE.into(),
            Self::Swift => tree_sitter_swift::LANGUAGE.into(),
            Self::Heex => tree_sitter_heex::LANGUAGE.into(),
            Self::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
            Self::KiQuickfix => tree_sitter_quickfix::language(),
            Self::Hcl => tree_sitter_hcl::LANGUAGE.into(),
        }
    }

    fn default_highlight_query(&self) -> Option<&str> {
        match self {
            Self::Typescript => Some(tree_sitter_typescript::HIGHLIGHTS_QUERY),
            Self::TSX => Some(tree_sitter_typescript::HIGHLIGHTS_QUERY),
            Self::Python => Some(tree_sitter_python::HIGHLIGHTS_QUERY),
            Self::Julia => None,
            Self::Scheme => Some(tree_sitter_scheme::HIGHLIGHTS_QUERY),
            Self::OCaml => Some(tree_sitter_ocaml::HIGHLIGHTS_QUERY),
            Self::OCamlInterface => Some(tree_sitter_ocaml::HIGHLIGHTS_QUERY),
            Self::Rust => Some(tree_sitter_rust::HIGHLIGHTS_QUERY),
            Self::Graphql => None,
            Self::Javascript => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            Self::JSX => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            Self::Svelte => Some(tree_sitter_svelte_ng::HIGHLIGHTS_QUERY),
            Self::JSON => Some(tree_sitter_json::HIGHLIGHTS_QUERY),
            Self::YAML => Some(tree_sitter_yaml::HIGHLIGHTS_QUERY),
            Self::HTML => Some(tree_sitter_html::HIGHLIGHTS_QUERY),
            Self::Haskell => Some(tree_sitter_haskell::HIGHLIGHTS_QUERY),
            Self::XML => Some(tree_sitter_xml::XML_HIGHLIGHT_QUERY),
            Self::Zig => Some(tree_sitter_zig::HIGHLIGHTS_QUERY),
            Self::Markdown => Some(tree_sitter_md::HIGHLIGHT_QUERY_BLOCK),
            Self::Go => Some(tree_sitter_go::HIGHLIGHTS_QUERY),
            Self::Lua => Some(tree_sitter_lua::HIGHLIGHTS_QUERY),
            Self::Gleam => Some(tree_sitter_gleam::HIGHLIGHT_QUERY),
            Self::Bash => Some(tree_sitter_bash::HIGHLIGHT_QUERY),
            Self::C => Some(tree_sitter_c::HIGHLIGHT_QUERY),
            Self::CPP => Some(tree_sitter_cpp::HIGHLIGHT_QUERY),
            Self::CSS => Some(tree_sitter_css::HIGHLIGHTS_QUERY),
            Self::Ruby => Some(tree_sitter_ruby::HIGHLIGHTS_QUERY),
            Self::Nix => Some(tree_sitter_nix::HIGHLIGHTS_QUERY),
            Self::Fish => Some(tree_sitter_fish::HIGHLIGHTS_QUERY),
            Self::Diff => Some(tree_sitter_diff::HIGHLIGHTS_QUERY),
            Self::Elixir => Some(tree_sitter_elixir::HIGHLIGHTS_QUERY),
            Self::Swift => Some(tree_sitter_swift::HIGHLIGHTS_QUERY),
            Self::Heex => Some(tree_sitter_heex::HIGHLIGHTS_QUERY),
            Self::Toml => Some(tree_sitter_toml_ng::HIGHLIGHTS_QUERY),
            Self::KiQuickfix => Some(r#" (header) @keyword"#),
            Self::Hcl => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct LspCommand {
    pub(crate) command: Command,
    pub(crate) initialization_options: Option<serde_json::Value>,
}

impl Language {
    pub const fn new() -> Self {
        Self {
            extensions: Vec::new(),
            file_names: Vec::new(),
            lsp_language_id: None,
            lsp_command: None,
            tree_sitter_grammar_config: None,
            formatter: None,
            line_comment_prefix: None,
            block_comment_affixes: None,
        }
    }

    pub fn file_names(&self) -> &Vec<String> {
        &self.file_names
    }

    pub fn lsp_language_id(&self) -> &Option<LanguageId> {
        &self.lsp_language_id
    }

    pub fn line_comment_prefix(&self) -> Option<String> {
        self.line_comment_prefix.clone()
    }

    pub fn block_comment_affixes(&self) -> Option<(String, String)> {
        self.block_comment_affixes.clone()
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GrammarConfig {
    pub id: String,
    pub kind: GrammarConfigKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum GrammarConfigKind {
    /// This is the recommended over `FromSource`, as `FromSource`
    /// is not reliable across different operating system.
    CargoLinked(CargoLinkedTreesitterLanguage),
    FromSource {
        url: String,
        commit: String,
        subpath: Option<String>,
    },
}

impl Language {
    pub fn extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    pub fn initialization_options(&self) -> Option<Value> {
        self.lsp_command.clone()?.initialization_options
    }

    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        let config = self.tree_sitter_grammar_config.as_ref()?;
        match &config.kind {
            GrammarConfigKind::CargoLinked(language) => Some(language.to_tree_sitter_language()),
            GrammarConfigKind::FromSource { .. } => grammar::grammar::get_language(&config.id).ok(),
        }
    }

    pub fn tree_sitter_grammar_config(&self) -> Option<GrammarConfiguration> {
        match &self.tree_sitter_grammar_config.as_ref()?.kind {
            GrammarConfigKind::CargoLinked(_) => None,
            GrammarConfigKind::FromSource {
                url,
                commit,
                subpath,
            } => self.tree_sitter_grammar_config.as_ref().map(|config| {
                GrammarConfiguration::remote(&config.id, url, commit, subpath.clone())
            }),
        }
    }

    /// We prioritize using highlight queries from nvim-treesitter
    /// over the default highlight queries provided by each Treesitter grammar
    /// repositories because the former produces better syntax highlighting.
    ///
    /// However, in the event that the tree-sitter-highlight crates cannot
    /// handle the nvim-treesitter query due to issues like Neovim-specific directives
    /// (this is validated through the use of `tree_sitter::Query::new`),
    /// we will fallback to the default highlight queries.
    pub fn highlight_query(&self) -> Option<String> {
        if let Some(query) = self.highlight_query_nvim_treesitter() {
            match Query::new(&self.tree_sitter_language()?, &query) {
                Ok(_) => return Some(query),
                Err(error) => {
                    log::error!(
                        "[Language::highlight_query]: Falling back to default query; unable to use highlight query of {} from nvim-treesitter due to error: {error:?}",
                        self.tree_sitter_grammar_config.clone()?.id
                    )
                }
            }
        }
        self.highlight_query_default()
    }

    pub fn highlight_query_nvim_treesitter(&self) -> Option<String> {
        get_highlight_query(&self.tree_sitter_grammar_config.clone()?.id).map(|result| {
            result
                .query
                // Replace `nvim-treesitter`-specific predicates with builtin predicates supported by `tree-sitter-highlight` crate
                // Reference: https://github.com/nvim-treesitter/nvim-treesitter/blob/23ba63028c6acca29be6462c0a291fc4a1b9eae8/CONTRIBUTING.md#predicates
                .replace("lua-match", "match")
                .replace("vim-match", "match")
                // Remove non-highlight captures, as they are not handled by this editor
                // See https://github.com/nvim-treesitter/nvim-treesitter/blob/23ba63028c6acca29be6462c0a291fc4a1b9eae8/CONTRIBUTING.md#non-highlighting-captures
                .replace("@none", "")
                .replace("@conceal", "")
                .replace("@spell", "")
                .replace("@nospell", "")
        })
    }

    fn highlight_query_default(&self) -> Option<String> {
        let config = self.tree_sitter_grammar_config.as_ref()?;
        match &config.kind {
            GrammarConfigKind::CargoLinked(language) => {
                Some(language.default_highlight_query()?.to_string())
            }
            GrammarConfigKind::FromSource { .. } => grammar::grammar::load_runtime_file(
                &self.tree_sitter_grammar_id()?,
                "highlights.scm",
            )
            .ok(),
        }
    }

    pub fn locals_query(&self) -> Option<&'static str> {
        None
    }

    pub fn injection_query(&self) -> Option<&'static str> {
        None
    }

    pub fn lsp_process_command(&self) -> Option<ProcessCommand> {
        self.lsp_command.as_ref().map(|command| {
            ProcessCommand::new(&command.command.command, &command.command.arguments)
        })
    }

    pub fn tree_sitter_grammar_id(&self) -> Option<String> {
        Some(self.tree_sitter_grammar_config.as_ref()?.id.to_string())
    }

    pub fn id(&self) -> Option<LanguageId> {
        self.lsp_language_id.clone()
    }

    fn formatter_command(&self) -> Option<ProcessCommand> {
        self.formatter
            .as_ref()
            .map(|command| ProcessCommand::new(&command.command, &command.arguments))
    }

    pub fn formatter(&self) -> Option<Formatter> {
        self.formatter_command().map(Formatter::from)
    }
}
