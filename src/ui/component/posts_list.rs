use std::{
    fs::DirEntry,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::utils::posts_files::read_posts_files_sorted;

use crate::ui::component::Post;
use chrono::{DateTime, Datelike, Utc};
use indexmap::IndexMap;
use leptos::{component, view, CollectView, IntoView};

fn split_files_by_year(files: Vec<DirEntry>) -> IndexMap<i32, Vec<DirEntry>> {
    let mut grouped_files: IndexMap<i32, Vec<DirEntry>> = IndexMap::new();

    for file in files {
        let year = <SystemTime as Into<DateTime<Utc>>>::into(
            file.metadata()
                .expect("the metadata to be readable")
                .created()
                .unwrap_or_else(|_| UNIX_EPOCH + Duration::from_secs(1721924110)), // TODO refactor
        )
        .year();

        grouped_files.entry(year).or_default().push(file)
    }

    grouped_files
}

#[component]
pub fn PostsList() -> impl IntoView {
    let files = read_posts_files_sorted().unwrap_or_default();

    if files.is_empty() {
        return view! { <p class="text-lg fond-semibold text-center m-16">"There are no posts yet."</p> }
            .into_view();
    }

    let grouped_files = split_files_by_year(files);

    view! {
        {grouped_files
            .into_iter()
            .map(|(year, files)| {
                view! {
                    <p class="text-lg font-bold mt-6 mb-2">{year}</p>
                    {files
                        .into_iter()
                        .map(|file| {
                            view! { <Post file=&file/> }
                        })
                        .collect_view()}
                }
            })
            .collect_view()}
    }
    .into_view()
}
