use leptos::*;
use leptos_query::*;

use crate::{
    components::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
    models::model_users_url::{AllTodosTagUrl, MyTodo},
    utils::hooks::queries::queries_users_url::useAllTodosQueryUrl,
};

#[component]
pub fn AllTodos() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllTodosQueryUrl().use_query(|| AllTodosTagUrl);

    let todos: Signal<Vec<MyTodo>> = Signal::derive(move || data.get().unwrap_or_default());

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

            <h2 class="text-lg font-bold">"All Todos"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"ID"</TableHead>
                            <TableHead>"Title"</TableHead>
                            <TableHead>"Description"</TableHead>
                            <TableHead>"Completed"</TableHead>
                        </TableRow>
                    </TableHeader>

                    <TableBody>
                        <Show
                            when=move || !todos.get().is_empty()
                            fallback=|| {
                                view! {
                                    <TableRow>
                                        // TODO. colSpan = 4
                                        <TableCell>"No users"</TableCell>
                                    </TableRow>
                                }
                            }
                        >

                            <For
                                each=todos
                                key=|todo| todo.id
                                children=move |todo| {
                                    view! {
                                        <TableRow>
                                            <TableCell>{todo.id.0}</TableCell>
                                            <TableCell>{todo.title}</TableCell>
                                            <TableCell>{todo.description}</TableCell>
                                            <TableCell>{todo.completed.to_string()}</TableCell>
                                        </TableRow>
                                    }
                                }
                            />

                        </Show>
                    </TableBody>
                </Table>

            </Transition>

        </div>
    }
}
