use pulldown_cmark::{CodeBlockKind, Event};

fn main() {
    let input = std::fs::read_to_string("./content/starting_open_source.md").unwrap();
    let mut next_lang = String::new();
    let custom = tree_painter::Theme::from_helix(include_str!("../themes/onedark.toml")).unwrap();
    let mut renderer = tree_painter::Renderer::new(custom);
    let css = renderer.css();

    // let reg = regex::Regex::new(r"(?P<a>\w)\.(?P<b>\w)").unwrap();
    // let data = reg.replace_all(data, "$a-$b");
    let parser = pulldown_cmark::Parser::new(&input).map(|event| match event {
        Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            next_lang = lang.to_string();
            Event::SoftBreak
        }
        Event::Text(code) if !next_lang.is_empty() => {
            let rust_lang = tree_painter::Lang::Rust;
            let mut code_str = String::new();
            code_str.push_str(&renderer.render(&rust_lang, code.as_bytes()).unwrap());
            Event::Html(code_str.into())
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
        r"
    <!DOCTYPE html>
    <head>
      <title>tree-painter highlighting</title>
      {}
    </head>
    <body>
    {}
    </body>
    ",
        css, mark_out
    ));
    // final_output.push_str(&html_out.to_string());
    std::fs::write("out.html", final_output).unwrap()
}
