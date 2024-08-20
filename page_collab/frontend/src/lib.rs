use api::Api;
use leptos::*;
use leptos_meta::*;
use leptos_query::{provide_query_client_with_options, DefaultQueryOptions};
use leptos_router::*;

// Modules
mod api;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Provide the query client with options to use local resource.
    provide_query_client_with_options(DefaultQueryOptions {
        resource_option: leptos_query::ResourceOption::Local,
        ..DefaultQueryOptions::default()
    });

    // Provide api
    let api = create_rw_signal(Api::default());
    provide_context(api);

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Page collab" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
