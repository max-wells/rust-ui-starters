use leptos::*;

use crate::{
    api::api_todos::get_todo,
    models::model_todo::{TodoId, TodoResponse},
    registry::ui::{headings::H2, label::Label},
};

// TODO 🤔 Understand how the input is done and how to use input.rs
// TODO 🤔 Understand how create_resource() works, and the difference with create_resource_with_options()

#[component]
pub fn TodoWithResource() -> impl IntoView {
    let (todo_id, set_todo_id) = create_signal(TodoId(0));

    // todo_id is a Signal<String>, and that is fed into the resource fetcher function.
    // any time todo_id changes, the resource will re-execute.
    let todo_resource: Resource<TodoId, TodoResponse> = create_resource(todo_id, get_todo);

    view! {
        <div class="flex flex-col gap-2 justify-between items-center p-4 rounded-md bg-neutral-200">
            <H2>"Todo with Resource"</H2>
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
                        todo_resource
                            .get()
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
