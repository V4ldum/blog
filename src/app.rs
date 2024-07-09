use crate::ui::page::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css"/>

        // sets the document title
        <Title text="Valdum's Blog"/>

        // content
        <Router fallback=|| view! { <NotFoundPage/> }>
            <main class="flex flex-col max-w-3xl mx-auto px-6">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path=":filename" view=PostPage/>
                </Routes>
            </main>
        </Router>
    }
}
