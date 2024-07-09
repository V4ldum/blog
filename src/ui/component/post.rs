use chrono::offset::Utc;
use chrono::DateTime;
use leptos::{component, view, IntoView};
use std::{fs::DirEntry, time::SystemTime};

use crate::utils::posts_files::StringUrlQuery;

#[component]
pub fn Post<'a>(file: &'a DirEntry) -> impl IntoView
where
    'a: 'a, // TODO try removing lifetimes in a future update
{
    let filename = file
        .file_name()
        .into_string()
        .expect("the filename to be readable")
        .to_ui_name();
    let metadata = file.metadata().expect("the metadata to be readable");

    view! {
        <div class="flex flex-row items-center my-2">
            <p class="text-base basis-24 grow-0 shrink-0">
                {format!(
                    "{}",
                    <SystemTime as Into<
                        DateTime<Utc>,
                    >>::into(metadata.created().expect("the created date to be readable"))
                        .format("%d %b"),
                )}

            </p>
            <a
                class="text-blue-600 text-lg font-semibold underline-offset-2 decoration-2 hover:text-blue-400 hover:underline"
                href=format!("/{}", filename.to_url_query())
            >
                {filename}
            </a>
        </div>
    }
}
