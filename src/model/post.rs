use std::fs;

use chrono::NaiveDate;
use gray_matter::{engine::YAML, Matter, ParsedEntityStruct};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    pub title: Option<String>,
    pub published: Option<String>, // TODO Date
    pub visible: Option<bool>,
}

pub struct Posts {
    pub posts: Vec<Post>,
}

pub struct Post {
    pub metadata: Metadata,
    pub markdown: String,
    pub filename: String,
}

#[derive(Debug, Default)]
pub struct Metadata {
    pub title: String,
    pub published: NaiveDate,
    pub visible: bool,
}

impl Posts {
    pub fn read_sorted(path: &str) -> Self {
        let mut posts = Self::read(path);

        // Sort files by created date
        posts
            .posts
            .sort_by(|a, b| b.metadata.published.cmp(&a.metadata.published));

        posts
    }

    pub fn read(path: &str) -> Self {
        let mut posts: Vec<_> = match fs::read_dir(path) {
            Ok(posts) => posts.collect(),
            Err(_) => Vec::new(),
        };

        // Filter unreadable files
        posts.retain(|post| post.is_ok());

        let mut posts = posts
            .into_iter()
            .map(|path| path.unwrap())
            .collect::<Vec<_>>();

        // Filter non Markdown files
        posts.retain(|post| {
            post.file_name()
                .into_string()
                .expect("the filename to be readable")
                .ends_with(".md")
        });

        // Map to Post model
        let mut posts = posts
            .into_iter()
            .map(|post| Post::read(post.path().to_str().unwrap().to_owned()).unwrap()) // TODO refactor error
            .collect::<Vec<_>>();

        // Filter files that should not be visible
        posts.retain(|post| post.metadata.visible);

        Posts { posts }
    }
}

impl Post {
    pub fn read(path: String) -> Result<Self, String> {
        // TODO refactor error
        let content = fs::read_to_string(&path).map_err(|_| "")?;
        let metadata = Metadata::read(&content);
        let markdown = content.split("---").last().unwrap().to_owned();
        let filename = path
            .split(&['/', '\\'])
            .last()
            .unwrap()
            .split('.')
            .take(1)
            .collect::<String>()
            .replace(' ', "-")
            .to_lowercase();

        Ok(Post {
            metadata,
            markdown,
            filename,
        })
    }
}

impl Metadata {
    pub fn read(content: &str) -> Self {
        let matter: Matter<YAML> = Matter::new();
        let result: Option<ParsedEntityStruct<FrontMatter>> = matter.parse_with_struct(content);

        match result {
            None => Metadata {
                title: String::from("NO TITLE"),
                ..Default::default()
            },
            Some(result) => Metadata {
                title: result.data.title.unwrap_or_default(),
                published: NaiveDate::parse_from_str(
                    &result.data.published.unwrap_or_default(),
                    "%d-%m-%Y",
                )
                .unwrap_or_default(),
                visible: result.data.visible.unwrap_or_default(),
            },
        }
    }
}
