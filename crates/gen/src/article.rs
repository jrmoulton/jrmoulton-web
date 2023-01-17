use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};

#[derive(Default, Clone)]
pub struct Article {
    pub head: &'static str,
    pub date: DateTime<Utc>,
    pub file_name_cap: String,
    pub file_name: String,
    pub header: &'static str,
    pub footer: &'static str,
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
    where S: serde::Serializer {
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
        state.serialize_field("footer", &self.footer)?;
        state.serialize_field("theme_divs", &self.theme_divs)?;
        state.serialize_field("page_section", &self.page_section)?;
        state.end()
    }
}

#[derive(Default, Debug)]
pub struct ArticlePreview {
    theme_divs: String,
    date: DateTime<Utc>,
    article_link: String,
    article_title: String,
    short_content: String,
}
impl Serialize for ArticlePreview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("ArticlePreview", 6)?;
        state.serialize_field("theme_divs", &self.theme_divs)?;
        state.serialize_field(
            "date",
            &self.date.date_naive().format("%B %-d, %C%y").to_string(),
        )?;
        state.serialize_field("article_link", &self.article_link)?;
        state.serialize_field("article_title", &self.article_title)?;
        state.serialize_field("short_content", &self.short_content)?;
        state.end()
    }
}
impl From<Article> for ArticlePreview {
    fn from(article: Article) -> Self {
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"(?s:<p>(.*?)</p>)").unwrap();
        }
        let article_link = format!("/articles/{}.html", article.file_name);
        let short_content: String = RE
            .captures_iter(&article.page_section)
            .take(2)
            .map(|p| p.get(0).unwrap().as_str().to_string())
            .collect();
        Self {
            theme_divs: article.theme_divs,
            date: article.date,
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
