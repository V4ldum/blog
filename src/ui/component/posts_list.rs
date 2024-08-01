use crate::{
    model::{post::Posts, Post},
    ui::component::Post,
};
use chrono::Datelike;
use indexmap::IndexMap;
use leptos::{component, view, CollectView, IntoView};

fn split_files_by_year(posts: Vec<Post>) -> IndexMap<i32, Vec<Post>> {
    let mut grouped_posts: IndexMap<i32, Vec<Post>> = IndexMap::new();

    for post in posts {
        let year = post.metadata.published.year();

        grouped_posts.entry(year).or_default().push(post)
    }

    grouped_posts
}

#[component]
pub fn PostsList() -> impl IntoView {
    let posts = Posts::read_sorted("posts").posts;

    if posts.is_empty() {
        return view! { <p class="text-lg fond-semibold text-center m-16">"There are no posts yet."</p> }
            .into_view();
    }

    let grouped_files = split_files_by_year(posts);

    view! {
        {grouped_files
            .into_iter()
            .map(|(year, posts)| {
                view! {
                    <p class="text-lg font-bold mt-6 mb-2">{year}</p>
                    {posts
                        .into_iter()
                        .map(|post| {
                            view! { <Post post=&post/> }
                        })
                        .collect_view()}
                }
            })
            .collect_view()}
    }
    .into_view()
}
