use leptos::*;
use leptos_query::*;

use crate::{
    models::model_server_books::{AllServerBooksTagFromClient, ServerBook},
    utils::hooks::queries::queries_server_books::useAllServerBooksQueryFromClient,
};

#[component]
pub fn AllServerBooksFromClient() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllServerBooksQueryFromClient().use_query(|| AllServerBooksTagFromClient);

    let books: Signal<Vec<ServerBook>> = Signal::derive(move || data.get().unwrap_or_default());

    create_effect(move |_| {
        let state = state.get();
        let log = match state {
            QueryState::Created => "created",
            QueryState::Loading => "loading",
            QueryState::Fetching(_) => "fetching",
            QueryState::Loaded(_) => "loaded",
            QueryState::Invalid(_) => "invalid",
        };
        logging::log!("STATE: {log}")
    });

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-neutral-300">

            <h2 class="text-lg font-bold">"All Server Books from client"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !books.get().is_empty()
                        fallback=|| {
                            view! { <p>"No books"</p> }
                        }
                    >

                        <For
                            each=books
                            key=|book| book.id
                            children=move |book| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{book.id.0}</span>
                                        <span>"Author : "</span>
                                        <span>{book.author}</span>
                                        <span>"Title:  "</span>
                                        <span>{book.title}</span>
                                    </li>
                                }
                            }
                        />

                    </Show>
                </ul>

            </Transition>

        </div>
    }
}
