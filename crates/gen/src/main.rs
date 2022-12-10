use pulldown_cmark::{CodeBlockKind, Event};

fn main() {
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::dir::copy("styles/", "build/", &copy_options).expect("Failed to copy css");
    fs_extra::dir::copy("js/", "build/", &copy_options).expect("failed to move js to build folder");
    for file in std::fs::read_dir("./content/").unwrap() {
        let file = file.unwrap();
        let input = std::fs::read_to_string(file.path()).unwrap();
        let mut next_lang = String::new();
        let mut custom = None;
        let mut theme_divs = String::new();
        let mut themes: Vec<_> = std::fs::read_dir("./themes/")
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        themes.sort_by_key(|a| a.path());
        for theme in themes {
            // <div class = "theme-drop-but" onmouseover="setTheme(this)" onclick="setTheme(this)" href="styles/onedark_darker.css">Link 1</div>
            let theme = theme.path();
            let theme = theme.file_stem().unwrap().to_str().unwrap();
            custom = Some(
                tree_painter::Theme::from_helix(
                    &std::fs::read_to_string(format!("./themes/{}.toml", theme)).unwrap(),
                )
                .unwrap(),
            );
            std::fs::write(
                format!("./build/styles/{}.css", theme),
                tree_painter::Renderer::new(custom.clone().unwrap()).css(),
            )
            .unwrap();
            theme_divs.push_str(&format!(r#"<div class="theme-drop-but" onmouseover="setTheme(this)" onclick="setTheme(this)" href="styles/{}.css">{}</div>"#, &theme, &theme));
        }
        let mut renderer = tree_painter::Renderer::new(custom.unwrap());
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
        let file_name = file.path();
        let file_name = file_name.file_stem().unwrap().to_str().unwrap();
        let file_name_cap = titlecase::titlecase(&file_name.replace('_', " "));
        let final_output = String::from(&format!(
            r#"
<!DOCTYPE html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{file_name_cap}</title>
    <link rel="stylesheet" href="/styles/common.css">
    <link id="theme" type="text/css" rel="stylesheet" href="/styles/onedark_dark.css">
    <script src="js/theme.js"></script>
</head>
<body>
    <div class = header>
        <div class="header-left">
            {}
        </div>
        <div class="header-right">
            <div class="dropdown">
                <button onclick="myFunction()" class="dropbtn">&#9660 Theme</button>
                <div id="myDropdown" class="dropdown-content">
                    {theme_divs}
                </div>
            </div>
        </div>
    </div>
    <div class="page-section">
        {mark_out}
    </div>
</body>
    "#,
            include_str!("../../../templates/header.html")
        ));
        std::fs::write(format!("build/{}.html", file_name), final_output).unwrap();
    }
}
