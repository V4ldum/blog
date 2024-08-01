use leptos::{component, view, IntoView};

use crate::model;

#[component]
pub fn Post<'a>(post: &'a model::Post) -> impl IntoView
where
    'a: 'a, // TODO try removing lifetimes in a future update
{
    view! {
        <div class="flex flex-row items-center my-2">
            <p class="text-base basis-24 grow-0 shrink-0">{format!("{}", &post.metadata.published.format("%d %b"))}</p>
            <a
                class="text-blue-600 text-lg font-semibold underline-offset-2 decoration-2 hover:text-blue-400 hover:underline"
                href=format!("/{}", &post.filename)
            >
                {&post.metadata.title}
            </a>
        </div>
    }
}
