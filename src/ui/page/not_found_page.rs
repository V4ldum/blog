use leptos::{component, view, IntoView};

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="text-center absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
            <h1 class="text-3xl font-bold">"Page not found"</h1>
            <img class="size-40 mx-auto rounded-full mt-8 mb-6 border-1 border-red-600" src="/public/racoon.png"/>
            <p class="text-lg mb-4">"I don't know what you were looking for but it's not there."</p>
            <a
                class="text-blue-600 text-lg font-semibold underline-offset-2 decoration-2 hover:text-blue-400 hover:underline"
                href="/"
            >
                "Go home ?"
            </a>
        </div>
    }
}
