use leptos::leptos_dom::logging;
use std::{fs::{self, DirEntry}, time::{UNIX_EPOCH, Duration}};

pub fn read_posts_files_sorted() -> Option<Vec<DirEntry>> {
    let mut files = read_posts_files()?;

    // Sort files by created date
    files.sort_by(|a, b| {
        b.metadata()
            .expect("the metadata to be readable")
            .created()
            .unwrap_or_else(|_| UNIX_EPOCH + Duration::from_nanos(1721924110)) // TODO refactor
            .cmp(
                &a.metadata()
                    .expect("the metadata date to be readable")
                    .created()
                    .unwrap_or_else(|_| UNIX_EPOCH + Duration::from_nanos(1721924110)) // TODO refactor
                )
    });

    Some(files)
}

pub fn read_posts_files() -> Option<Vec<DirEntry>> {
    let mut files: Vec<_> = fs::read_dir("posts").ok()?.collect();

    // Filter unreadable files
    files.retain(|file| {
        //file.as_ref().is_ok_and(|file| file.metadata().is_ok());
        file.as_ref().is_ok_and(|file| match file.metadata() {
            Err(err) => {
                logging::console_warn(
                    format!("Error reading metadata, reason: {:#?}", err).as_str(),
                );
                false
            }
            Ok(_) => true,
        })
    });

    let mut files = files
        .into_iter()
        .map(|path| path.unwrap())
        .collect::<Vec<_>>();

    // Filter non Markdown files
    files.retain(|file| {
        file.file_name()
            .into_string()
            .expect("the filename to be readable")
            .ends_with(".md")
    });

    Some(files)
}

pub trait StringUrlQuery {
    fn to_ui_name(&self) -> Self;
    fn to_url_query(&self) -> Self;
}

impl StringUrlQuery for String {
    fn to_ui_name(&self) -> Self {
        self.replace(".md", "")
    }
    fn to_url_query(&self) -> Self {
        self.replace(' ', "-").to_ascii_lowercase()
    }
}
