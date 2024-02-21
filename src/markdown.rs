#[cfg(feature = "ssr")]
use comrak::plugins::syntect::SyntectAdapterBuilder;
#[cfg(feature = "ssr")]
use comrak::{
    markdown_to_html_with_plugins,
    ComrakOptions, ComrakPlugins,
};
use leptos::*;
#[cfg(feature = "ssr")]
use std::io::Cursor;
#[cfg(feature = "ssr")]
use syntect::highlighting::ThemeSet;

#[cfg(feature = "ssr")]
const NIGHT_OWL: &[u8; 27913] =
    include_bytes!("../night-owlish.tmtheme");

#[cfg(feature = "ssr")]
pub fn compile(input: &str) -> String {
    // let adapter = SyntectAdapter::new("Solarized
    // (dark)");
    let mut cursor = Cursor::new(NIGHT_OWL);

    let theme_night_owl =
        ThemeSet::load_from_reader(&mut cursor)
            .expect("expect markdown theme to be loadable");
    let mut theme_set = ThemeSet::new();
    theme_set
        .themes
        .entry("Night Owl".to_string())
        .or_insert(theme_night_owl);
    // let theme_set = ;
    let adapter = SyntectAdapterBuilder::new()
        .theme_set(theme_set)
        .theme("Night Owl")
        .build();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter =
        Some(&adapter);
    let mut options = ComrakOptions::default();
    // UNSAFE HTML TAGS!
    options.render.unsafe_ = false;
    // extensions, like those on github
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = false;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = false;
    options.extension.front_matter_delimiter = None;
    options.extension.multiline_block_quotes = true;

    
    markdown_to_html_with_plugins(
        input, &options, &plugins,
    )
}

#[server(MarkdownCompileServer, "/api")]
pub async fn markdown_compile_server(
    code: String,
) -> Result<String, ServerFnError> {
    Ok(compile(&code))
}
