use leptos::*;
use leptos_meta::*;
use leptos_query::*;
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::{Route, Router, Routes};

use crate::components::navbar::Navbar;
use crate::components::tailwind_indicator::TailwindIndicator;
use crate::error_template::{AppError, ErrorTemplate};
use crate::routes::page_home::HomePage;
use crate::routes::page_todos::PageTodos;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provides Query Client for entire app.
    provide_query_client_with_options_and_persister(
        Default::default(),
        query_persister::LocalStoragePersister,
    );

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum.css" />
        <Title text="Rust UI" />

        <LeptosQueryDevtools />

        <body class="min-h-screen font-sans antialiased">
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors /> }.into_view()
            }>
                <Navbar />
                <TailwindIndicator />
                <main class="flex relative flex-col mx-auto w-full min-h-screen">
                    <Routes>
                        <Route path="" view=HomePage />
                        <Route path="/todos" view=PageTodos />
                    </Routes>
                </main>
            </Router>
        </body>
    }
}
