use pulldown_cmark::{CodeBlockKind, Event};

fn main() {
    std::fs::create_dir_all("build/styles").unwrap();
    std::process::Command::new("sass")
        .arg("styles/common.scss")
        .arg("build/styles/common.css")
        .spawn()
        .unwrap();
    for file in std::fs::read_dir("./content/").unwrap() {
        let file = file.unwrap();
        let input = std::fs::read_to_string(file.path()).unwrap();
        let mut next_lang = String::new();
        let custom =
            tree_painter::Theme::from_helix(include_str!("../themes/onedark.toml")).unwrap();
        let mut renderer = tree_painter::Renderer::new(custom);

        let parser = pulldown_cmark::Parser::new(&input).map(|event| match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                next_lang = lang.to_string();
                Event::SoftBreak
            }
            Event::Text(code) if !next_lang.is_empty() => {
                if next_lang == "date" {
                    Event::Html(format!(r#"<div class="date">{}</div>"#, code.trim()).into())
                } else {
                    let lang = tree_painter::Lang::from_name(&next_lang).unwrap();
                    let mut code_str = String::new();
                    code_str.push_str(&renderer.render(&lang, code.as_bytes()).unwrap());
                    Event::Html(code_str.into())
                }
            }
            Event::End(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.to_string() == next_lang =>
            {
                next_lang = "".to_string();
                Event::SoftBreak
            }
            _ => event,
        });
        let mut mark_out = String::new();
        pulldown_cmark::html::push_html(&mut mark_out, parser);
        let final_output = String::from(&format!(
            r#"
<!DOCTYPE html>
<head>
    <title>tree-painter highlighting</title>
    <link rel="stylesheet" href="styles/common.css">
</head>
<body>
    <div class="page-section">
        {mark_out}
    </div>
</body>
    "#
        ));
        std::fs::write(
            format!(
                "build/{}.html",
                file.path().file_stem().unwrap().to_str().unwrap()
            ),
            final_output,
        )
        .unwrap()
    }
}
