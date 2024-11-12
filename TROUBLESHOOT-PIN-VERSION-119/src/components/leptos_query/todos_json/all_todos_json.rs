use leptos::*;
use leptos_query::*;

use crate::{
    models::model_todo_json::{AllTodosTagJson, TodoJson},
    utils::hooks::queries::queries_todos_json::useAllTodosQueryJson,
};

#[component]
pub fn AllTodosJson() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllTodosQueryJson().use_query(|| AllTodosTagJson);

    let todos: Signal<Vec<TodoJson>> = Signal::derive(move || data.get().unwrap_or_default());

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

            <h2 class="text-lg font-bold">"All Todos (Complex)"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !todos.get().is_empty()
                        fallback=|| {
                            view! { <p>"No todos"</p> }
                        }
                    >

                        <For
                            each=todos
                            key=|todo| todo.id
                            children=move |todo| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{todo.id.0}</span>
                                        <span>"Content : "</span>
                                        <span>{todo.content}</span>
                                        <span>"Description:  "</span>
                                        <span>{todo.description}</span>
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
