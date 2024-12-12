mod article;
mod metadata;
mod theme;

use std::sync::{Arc, Mutex};

use article::{Article, ArticlePreview, LatestArticles};
use chrono::format;
use handlebars::Handlebars;
use metadata::Metadata;
use pulldown_cmark::{CodeBlockKind, Event};
use rayon::prelude::*;
use theme::{Theme, Themes};

fn main() {
    // delete the current build folder
    // let _ = std::fs::remove_dir_all("build");

    // Create the build folder
    let _ = fs_extra::dir::create("build/", false);
    let _ = fs_extra::dir::create("build/articles/", false);

    // Copy directories to build folder
    let dir_copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        ..Default::default()
    };
    for folder in vec!["styles", "js", "images", "fonts", "files"] {
        fs_extra::dir::copy(folder, "build/", &dir_copy_options)
            .expect("Failed to copy {folder} folder to build folder");
    }

    // Copy some files
    let file_copy_options = fs_extra::file::CopyOptions {
        overwrite: true,
        ..Default::default()
    };
    for icon in std::fs::read_dir("icons/").unwrap() {
        let icon = icon.unwrap();
        fs_extra::file::copy(
            icon.path(),
            format!("build/{}", icon.file_name().to_string_lossy()),
            &file_copy_options,
        )
        .expect("failed to copy icon to build folder");
    }
    for file in vec!["resume.pdf"] {
        fs_extra::file::copy(
            std::path::PathBuf::from(format!("files/{file}")),
            format!("build/files/{}", file),
            &file_copy_options,
        )
        .expect(&format!("Failed to copy file {file}"));
    }

    // Register the templates
    let mut templ_reg = Handlebars::new();
    for template in vec!["theme_div", "article", "article_preview", "index", "blurb"] {
        templ_reg
            .register_template_string(
                template,
                std::fs::read_to_string(format!("templates/{template}.html")).unwrap(),
            )
            .unwrap();
    }

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
        themes.themes.push(Theme { base_name: theme_base_name });
    }
    // Generate the css from the themes
    for theme in &themes.themes {
        std::fs::write(
            format!("./build/styles/{}.css", theme.base_name),
            tree_painter::Renderer::new(theme.tree_painter_theme()).css(),
        )
        .unwrap();
    }

    // sort the themes so that they are in alphabetical order when the site is
    // generated
    themes.sort_themes();

    let latest_articles = write_articles(&mut templ_reg, &mut themes);
    let mut index_article = Article {
        head: include_str!("../../../templates/head.html"),
        date: dateparser::parse("12/12/2022").unwrap(),
        file_name_cap: "Jared Moutlon".to_string(),
        file_name: "index".to_string(),
        header: include_str!("../../../templates/header.html"),
        footer: include_str!("../../../templates/footer.html"),
        theme_divs: themes.theme_divs(&templ_reg),
        page_section: String::new(),
    };
    let mut articles_page = index_article.clone();
    articles_page.file_name_cap = "Articles".to_string();
    index_article
        .page_section
        .push_str(include_str!("../../../templates/blurb.html"));
    index_article
        .page_section
        .push_str(r#"<div class="previews">"#);
    articles_page
        .page_section
        .push_str(r#"<div class="previews">"#);
    for article in latest_articles.articles {
        let article_string = templ_reg
            .render("article_preview", &ArticlePreview::from(article.clone()))
            .unwrap();
        index_article.page_section.push_str(&article_string);
        articles_page.page_section.push_str(&article_string);
    }
    index_article.page_section.push_str("</div>");
    articles_page.page_section.push_str("</div>");
    std::fs::write(
        "build/index.html",
        templ_reg.render("article", &index_article).unwrap(),
    )
    .unwrap();
    std::fs::write(
        "build/articles.html",
        templ_reg.render("article", &articles_page).unwrap(),
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
        let mut publish = true;

        let article_string = file.as_ref().unwrap();

        let input = std::fs::read_to_string(article_string.path()).unwrap();

        let mut renderer = tree_painter::Renderer::new(theme.clone());

        let mut next_lang = String::new();
        let mut heading = false;

        let parser = pulldown_cmark::Parser::new(&input).map(|event| match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                next_lang = lang.to_string();
                Event::SoftBreak
            },
            Event::Text(code) if next_lang == "date" => {
                date = dateparser::parse(code.trim()).unwrap();
                Event::Html(
                    format!(
                        r#"<div class="date">{}</div>"#,
                        date.date_naive().format("%A,  %B %-d, %C%y")
                    )
                    .into(),
                )
            },
            Event::Text(toml) if next_lang == "metadata" => {
                let metadata: Metadata = toml::from_str(&toml).unwrap();
                if metadata.develop {
                    publish = false;
                }
                Event::SoftBreak
            },
            Event::Text(code) if !next_lang.is_empty() => {
                let lang = tree_painter::Lang::from_name(&next_lang)
                    .unwrap_or_else(|| panic!("lange {next_lang}"));
                Event::Html(renderer.render(lang, code.as_bytes()).unwrap().into())
            },
            Event::End(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.to_string() == next_lang =>
            {
                next_lang = "".to_string();
                Event::SoftBreak
            },
            Event::Start(pulldown_cmark::Tag::Heading(inner1, inner2, inner3)) => {
                heading = true;
                Event::Start(pulldown_cmark::Tag::Heading(inner1, inner2, inner3))
            },
            Event::Text(text) if heading => Event::Text(titlecase::titlecase(&text).into()),
            Event::End(pulldown_cmark::Tag::Heading(inner1, inner2, inner3)) => {
                heading = false;
                Event::End(pulldown_cmark::Tag::Heading(inner1, inner2, inner3))
            },
            _ => event,
        });
        let mut mark_out = String::new();
        pulldown_cmark::html::push_html(&mut mark_out, parser);

        if publish || cfg!(debug_assertions) {
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
                footer: include_str!("../../../templates/footer.html"),
                theme_divs: themes.theme_divs(templ_reg),
                page_section: mark_out,
            };
            let final_output = templ_reg.render("article", &article).unwrap();
            let mut latest_articles = latest_articles.lock().unwrap();
            latest_articles.add_if_latest(article);
            std::fs::write(format!("build/articles/{}.html", file_name), final_output).unwrap();
        }
    });
    let lock = Arc::try_unwrap(latest_articles).expect("Lock still has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}
