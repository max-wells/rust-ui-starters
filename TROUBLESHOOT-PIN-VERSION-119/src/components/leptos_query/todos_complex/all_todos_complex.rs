use leptos::*;
use leptos_query::*;

use crate::{
    api::api_todos_complex::delete_todo_complex,
    models::model_todo_complex::{AllTodosTagComplex, TodoComplex, TodoIdComplex},
    registry::{icons::others::trash2::Trash2, ui::button::{ButtonVariant, Button, ButtonSize}},
    utils::hooks::queries::queries_todos_complex::{useAllTodosQueryComplex, useTodoQueryComplex},
};

// TODO. Split delete_todo into a separate component.

#[component]
pub fn AllTodosComplex() -> impl IntoView {
    let QueryResult {   
        data,
        state,
        refetch,
        ..
    } = useAllTodosQueryComplex().use_query(|| AllTodosTagComplex);

    let todos: Signal<Vec<TodoComplex>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_todo = create_action(move |id: &TodoIdComplex| {
        let id = *id;
        let refetch = refetch.clone();

        let todo_query = useTodoQueryComplex();
        let all_todos_query = useAllTodosQueryComplex();
        
        async move {
            all_todos_query.cancel_query(AllTodosTagComplex);

            all_todos_query.update_query_data_mut(AllTodosTagComplex, |todos| {
                todos.retain(|t| t.id != id);
            });

            todo_query.set_query_data(id, Ok(None));

            let _ = delete_todo_complex(id).await;

            let _ = todo_query.invalidate_query(id);

            refetch()
        }
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
                                        <span>{todo.id.0}</span>
                                        <span>": "</span>
                                        <span>{todo.content}</span>
                                        <span>" "</span>
                                        <Button
                                            variant=ButtonVariant::Destructive
                                            size=ButtonSize::Sm
                                            on:click=move |_| { delete_todo.dispatch(todo.id) }
                                        >
                                            <Trash2 class="size-4" />
                                        </Button>
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
