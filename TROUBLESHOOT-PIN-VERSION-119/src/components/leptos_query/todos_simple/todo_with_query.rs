use leptos::*;
use leptos_query::*;

use crate::{
    models::model_todo::TodoId,
    registry::ui::{headings::H2, label::Label},
    utils::hooks::queries::queries_todos::useTodoQuery,
};

#[component]
pub fn TodoWithQuery() -> impl IntoView {
    let (todo_id, set_todo_id) = create_signal(TodoId(0));

    let QueryResult {
        data, ..
    } = useTodoQuery().use_query(move || todo_id.get());

    view! {
        <div class="flex flex-col gap-2 justify-between items-center p-4 rounded-md bg-neutral-200">
            <H2>"Todo with Query"</H2>
            <Label>"Todo ID"</Label>

            <input
                type="number"
                on:input=move |ev| {
                    if let Ok(todo_id) = event_target_value(&ev).parse() {
                        set_todo_id(TodoId(todo_id));
                    }
                }
                prop:value=move || todo_id.get().0
            />

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                <p>
                    {move || {
                        data.get()
                            .map(|a| {
                                match a.ok().flatten() {
                                    Some(todo) => todo.content,
                                    None => "Not found 😢".into(),
                                }
                            })
                    }}

                </p>
            </Transition>
        </div>
    }
}
