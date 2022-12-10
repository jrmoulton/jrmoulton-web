use handlebars::Handlebars;
use pulldown_cmark::{CodeBlockKind, Event};

mod structs;
use structs::*;

fn main() {
    // Copy css and javascript over to the build folder
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::dir::copy("styles/", "build/", &copy_options).expect("Failed to copy css");
    fs_extra::dir::copy("js/", "build/", &copy_options).expect("failed to move js to build folder");

    // Register the templates
    let mut templ_reg = Handlebars::new();
    templ_reg
        .register_template_string(
            "theme_div",
            include_str!("../../../templates/theme_div.html"),
        )
        .unwrap();

    // Get all the themes
    let mut themes = Themes::default();
    let theme_files: Vec<_> = std::fs::read_dir("./themes/")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    for theme_file in theme_files {
        let theme_path = theme_file.path();
        let theme_base_name = theme_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        themes.themes.push(Theme {
            base_name: theme_base_name,
        });
    }
    // Generate the css from the themes
    for theme in &themes.themes {
        std::fs::write(
            format!("./build/styles/{}.css", theme.base_name),
            tree_painter::Renderer::new(theme.tree_painter_theme()).css(),
        )
        .unwrap();
    }

    write_articles(&mut templ_reg, &mut themes);
}

fn write_articles(templ_reg: &mut Handlebars, themes: &mut Themes) {
    templ_reg
        .register_template_string("article", include_str!("../../../templates/article.html"))
        .unwrap();
    for article_string in std::fs::read_dir("./content/").unwrap() {
        let article_string = article_string.unwrap();
        let input = std::fs::read_to_string(article_string.path()).unwrap();
        let mut renderer = tree_painter::Renderer::new(
            tree_painter::Theme::from_helix(include_str!("../../../themes/onedark_dark.toml"))
                .unwrap(),
        );

        let mut next_lang = String::new();
        let mut heading = false;
        let parser = pulldown_cmark::Parser::new(&input).map(|event| match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                next_lang = lang.to_string();
                Event::SoftBreak
            }
            Event::Text(code) if next_lang == "date" => Event::Html(
                format!(
                    r#"<div class="date">{}</div>"#,
                    dateparser::parse(code.trim())
                        .unwrap()
                        .date_naive()
                        .format("%A,  %B %-d, %C%y")
                )
                .into(),
            ),
            Event::Text(code) if !next_lang.is_empty() => {
                let lang = tree_painter::Lang::from_name(&next_lang).unwrap();
                let mut code_str = String::new();
                code_str.push_str(&renderer.render(&lang, code.as_bytes()).unwrap());
                Event::Html(code_str.into())
            }
            Event::End(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.to_string() == next_lang =>
            {
                next_lang = "".to_string();
                Event::SoftBreak
            }
            Event::Start(pulldown_cmark::Tag::Heading(inner1, inner2, inner3)) => {
                heading = true;
                Event::Start(pulldown_cmark::Tag::Heading(inner1, inner2, inner3))
            }
            Event::Text(text) if heading => Event::Text(titlecase::titlecase(&text).into()),
            Event::End(pulldown_cmark::Tag::Heading(inner1, inner2, inner3)) => {
                heading = false;
                Event::End(pulldown_cmark::Tag::Heading(inner1, inner2, inner3))
            }
            _ => event,
        });
        let mut mark_out = String::new();
        pulldown_cmark::html::push_html(&mut mark_out, parser);

        // Write the article
        let file_name = article_string.path();
        let file_name = file_name.file_stem().unwrap().to_str().unwrap();
        let file_name_cap = titlecase::titlecase(&file_name.replace('_', " "));
        let article = Article {
            head: include_str!("../../../templates/head.html"),
            file_name_cap,
            header: include_str!("../../../templates/header.html"),
            theme_divs: themes.theme_divs(templ_reg),
            page_section: mark_out,
        };
        let final_output = templ_reg.render("article", &article).unwrap();

        std::fs::write(format!("build/{}.html", file_name), final_output).unwrap();
    }
}
