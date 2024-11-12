use std::error::Error;

use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakOptions, ComrakPlugins,
};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      🦀 ENTRYPOINT 🦀                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// parse a markdown source into its optional frontmatter and the HTML string.
pub fn parse_markdown_source(
    source: &str,
) -> Result<(Option<frontmatter::Yaml>, String), Box<dyn Error>> {
    let (fm, content) = extract_frontmatter(source)?;
    let html = md_to_html(content);
    Ok((fm, html))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

fn extract_frontmatter(input: &str) -> Result<(Option<frontmatter::Yaml>, &str), Box<dyn Error>> {
    let (fm, content) = frontmatter::parse_and_find_content(input)?;
    Ok((fm, content))
}

fn md_to_html(s: &str) -> String {
    let options = ComrakOptions {
        parse: comrak::ComrakParseOptions {
            ..comrak::ComrakParseOptions::default()
        },

        extension: comrak::ComrakExtensionOptions {
            autolink: true,
            table: true,
            description_lists: true,
            superscript: true,
            strikethrough: true,
            footnotes: true,
            ..comrak::ComrakExtensionOptions::default()
        },

        render: comrak::ComrakRenderOptions {
            unsafe_: true,
            ..comrak::ComrakRenderOptions::default()
        },
    };
    let mut plugins = ComrakPlugins::default();

    let adapter = SyntectAdapter::new("InspiredGitHub");
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    // Return the html
    markdown_to_html_with_plugins(s, &options, &plugins)
}
