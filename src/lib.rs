pub mod app;
pub mod model;
pub mod ui;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    //use crate::app::*;
    console_error_panic_hook::set_once();

    // No mount in island mode
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
    // leptos::mount_to_body(App);
}
