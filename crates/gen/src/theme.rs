use handlebars::Handlebars;
use serde::Serialize;

#[derive(Default)]
pub struct Themes {
    pub themes: Vec<Theme>,
}
impl Themes {
    pub fn theme_divs(&self, registry: &Handlebars) -> String {
        let mut divs_string = String::new();
        // self.themes.sort();
        for theme in &self.themes {
            divs_string.push_str(&registry.render("theme_div", &theme).unwrap());
        }
        divs_string
    }

    pub fn sort_themes(&mut self) {
        self.themes.sort();
    }
}

#[derive(Default, Clone, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Theme {
    pub base_name: String,
}
impl Theme {
    pub fn tree_painter_theme(&self) -> tree_painter::Theme {
        tree_painter::Theme::from_helix(
            &std::fs::read_to_string(format!("./themes/{}.toml", &self.base_name)).unwrap(),
        )
        .unwrap()
    }
}
