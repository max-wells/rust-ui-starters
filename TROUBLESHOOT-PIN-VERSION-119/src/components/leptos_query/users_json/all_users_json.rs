use leptos::*;
use leptos_query::*;

use crate::{
    models::model_users_json::{AllUsersTagJson, UserJson},
    utils::hooks::queries::queries_users_json::useAllUsersQueryJson,
};

#[component]
pub fn AllUsersJson() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllUsersQueryJson().use_query(|| AllUsersTagJson);

    let users: Signal<Vec<UserJson>> = Signal::derive(move || data.get().unwrap_or_default());

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

            <h2 class="text-lg font-bold">"All Users (Complex)"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !users.get().is_empty()
                        fallback=|| {
                            view! { <p>"No users"</p> }
                        }
                    >

                        <For
                            each=users
                            key=|user| user.id
                            children=move |user| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{user.id.0}</span>
                                        <span>"User ID : "</span>
                                        <span>{user.userId.0}</span>
                                        <span>"Title : "</span>
                                        <span>{user.title}</span>
                                        <span>"Body:  "</span>
                                        <span>{user.body}</span>
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
