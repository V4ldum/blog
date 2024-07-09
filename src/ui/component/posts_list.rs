use std::{fs::DirEntry, time::SystemTime};

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
                .expect("the created date to be readable"),
        )
        .year();

        grouped_files.entry(year).or_default().push(file)
    }

    grouped_files
}

#[component]
pub fn PostsList() -> impl IntoView {
    let Some(files) = read_posts_files_sorted() else {
        return view! { <p class="text-lg fond-semibold text-center m-16">"There are no posts yet."</p> }
            .into_view();
    };
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
