use std::fmt::Debug;

use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde::ser::SerializeStruct;
use serde::Serialize;

#[derive(Default, Clone)]
pub struct Article {
    pub head: &'static str,
    pub date: DateTime<Utc>,
    pub file_name_cap: String,
    pub file_name: String,
    pub header: &'static str,
    pub theme_divs: String,
    pub page_section: String,
}
impl Debug for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Article")
            .field("date", &self.date)
            .finish_non_exhaustive()
    }
}
impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 6)?;
        state.serialize_field("head", &self.head)?;
        state.serialize_field(
            "date",
            &self
                .date
                .date_naive()
                .format("%A,  %B %-d, %C%y")
                .to_string(),
        )?;
        state.serialize_field("file_name_cap", &self.file_name_cap)?;
        state.serialize_field("file_name", &self.file_name)?;
        state.serialize_field("header", &self.header)?;
        state.serialize_field("theme_divs", &self.theme_divs)?;
        state.serialize_field("page_section", &self.page_section)?;
        state.end()
    }
}

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

#[derive(Default, Debug, Serialize)]
pub struct ArticlePreview {
    theme_divs: String,
    article_link: String,
    article_title: String,
    short_content: String,
}

impl From<Article> for ArticlePreview {
    fn from(article: Article) -> Self {
        let article_link = format!("/{}.html", article.file_name);
        let re = regex::Regex::new(r"<p>(.*?)</p>").unwrap();
        let short_content: String = re
            .captures_iter(&article.page_section)
            .take(2)
            .map(|p| format!(r"{}", p.get(0).unwrap().as_str()))
            .collect();
        Self {
            theme_divs: article.theme_divs,
            article_link,
            article_title: article.file_name_cap,
            short_content,
        }
    }
}

#[derive(Default, Debug)]
pub struct LatestArticles {
    pub articles: Vec<Article>,
}
impl LatestArticles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_if_latest(&mut self, article: Article) {
        self.articles.push(article);
        self.articles.sort_by(|a, b| b.date.cmp(&a.date));
        if self.articles.len() > 9 {
            self.articles.pop();
        }
    }
}
