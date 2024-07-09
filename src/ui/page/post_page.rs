use std::fs;

use markdown::Options;

use leptos::{component, view, IntoView, Params, SignalWith};
use leptos_router::{use_params, Params};

use crate::utils::posts_files::{read_posts_files, StringUrlQuery};

use super::NotFoundPage;

#[derive(Params, PartialEq)]
struct PostFile {
    filename: String,
}

#[component]
pub fn PostPage() -> impl IntoView {
    // Read the URL for filename
    let query = use_params::<PostFile>();
    let filename = query.with(|params| {
        params
            .as_ref()
            .map(|params| params.filename.clone())
            .unwrap_or_default()
    });

    // Find the corresponding file on the fs, 404 if not found
    let Some(mut files) = read_posts_files() else {
        return view! { <NotFoundPage/> }.into_view();
    };
    files.retain(|file| {
        file.file_name()
            .into_string()
            .expect("the filename to be readable")
            .to_ui_name()
            .to_url_query()
            == filename
    });
    let Some(file) = files.first() else {
        return view! { <NotFoundPage/> }.into_view();
    };

    let file_content = fs::read_to_string(file.path()).expect("to be able to read the file");

    view! {
        <div class="my-12">
            <a
                href="/"
                class="text-blue-600 text-xl font-semibold underline-offset-2 decoration-2 hover:text-blue-400 hover:underline"
            >
                "Go back"
            </a>
        </div>

        <article
            id="markdown"
            inner_html=format!(
                "<h1>{}</h1>\r\n{}",
                file.file_name().into_string().expect("the filename to be readable").to_ui_name(),
                markdown::to_html_with_options(&file_content, &Options::gfm())
                    .expect("the Markdown to be properly formatted"),
            )
        >
        </article>
    }
    .into_view()
}
