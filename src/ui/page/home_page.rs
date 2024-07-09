use crate::ui::component::PostsList;
use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-5xl font-bold m-8 text-center">"Welcome to my blog !"</h1>
        <PostsList/>
    }
}
