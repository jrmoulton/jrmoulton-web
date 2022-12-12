use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use pulldown_cmark::{CodeBlockKind, Event};
use rayon::prelude::*;

mod structs;
use structs::*;

fn main() {
    // Copy css and javascript over to the build folder
    let mut dir_copy_options = fs_extra::dir::CopyOptions::new();
    dir_copy_options.overwrite = true;
    let mut file_copy_options = fs_extra::file::CopyOptions::new();
    file_copy_options.overwrite = true;
    let _ = fs_extra::dir::create("build/", false);
    fs_extra::dir::copy("styles/", "build/", &dir_copy_options).expect("Failed to copy css");
    fs_extra::dir::copy("js/", "build/", &dir_copy_options)
        .expect("failed to move js to build folder");
    for icon in std::fs::read_dir("icons/").unwrap() {
        let icon = icon.unwrap();
        fs_extra::file::copy(
            icon.path(),
            format!("build/{}", icon.file_name().to_string_lossy()),
            &file_copy_options,
        )
        .expect("failed to copy icon to build folder");
    }

    // Register the templates
    let mut templ_reg = Handlebars::new();
    templ_reg
        .register_template_string(
            "theme_div",
            include_str!("../../../templates/theme_div.html"),
        )
        .unwrap();
    templ_reg
        .register_template_string("article", include_str!("../../../templates/article.html"))
        .unwrap();
    templ_reg
        .register_template_string(
            "article_preview",
            include_str!("../../../templates/article_preview.html"),
        )
        .unwrap();
    templ_reg
        .register_template_string("index", include_str!("../../../templates/index.html"))
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

    // sort the themes so that they are in alphabetical order when the site is generated
    themes.sort_themes();

    let latest_articles = write_articles(&mut templ_reg, &mut themes);
    // things I need for an index.html
    //
    // A full article
    let mut index_article = Article {
        head: include_str!("../../../templates/head.html"),
        date: dateparser::parse("12/12/2022").unwrap(),
        file_name_cap: "Jared Moutlon".to_string(),
        file_name: "index".to_string(),
        header: include_str!("../../../templates/header.html"),
        theme_divs: themes.theme_divs(&templ_reg),
        page_section: String::from(r#"<div class="previews">"#),
    };
    for article in latest_articles.articles {
        index_article.page_section.push_str(
            &templ_reg
                .render("article_preview", &ArticlePreview::from(article))
                .unwrap(),
        );
    }
    index_article.page_section.push_str("</div>");
    std::fs::write(
        "build/index.html",
        templ_reg.render("article", &index_article).unwrap(),
    )
    .unwrap();
}

fn write_articles(templ_reg: &mut Handlebars, themes: &mut Themes) -> LatestArticles {
    let files: Vec<_> = std::fs::read_dir("./content/")
        .expect("couldnt read the content directory")
        .collect();
    let theme =
        tree_painter::Theme::from_helix(include_str!("../../../themes/onedark_dark.toml")).unwrap();
    let latest_articles = Arc::new(Mutex::new(LatestArticles::new()));
    files.par_iter().for_each(|file| {
        let mut date = dateparser::parse("1/1/2000").unwrap();
        let article_string = file.as_ref().unwrap();
        let input = std::fs::read_to_string(article_string.path()).unwrap();
        let mut renderer = tree_painter::Renderer::new(theme.clone());

        let mut next_lang = String::new();
        let mut heading = false;
        let parser = pulldown_cmark::Parser::new(&input).map(|event| match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                next_lang = lang.to_string();
                Event::SoftBreak
            }
            Event::Text(code) if next_lang == "date" => {
                date = dateparser::parse(code.trim()).unwrap();
                Event::Html(
                    format!(
                        r#"<div class="date">{}</div>"#,
                        date.date_naive().format("%A,  %B %-d, %C%y")
                    )
                    .into(),
                )
            }
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
            date,
            file_name_cap,
            file_name: file_name.to_string(),
            header: include_str!("../../../templates/header.html"),
            theme_divs: themes.theme_divs(templ_reg),
            page_section: mark_out,
        };
        let final_output = templ_reg.render("article", &article).unwrap();
        latest_articles.lock().unwrap().add_if_latest(article);

        std::fs::write(format!("build/{}.html", file_name), final_output).unwrap();
    });
    let lock = Arc::try_unwrap(latest_articles).expect("Lock still has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}
