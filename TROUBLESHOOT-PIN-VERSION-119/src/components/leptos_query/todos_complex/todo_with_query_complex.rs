use leptos::*;
use leptos_query::*;

use crate::{
    models::model_todo_complex::TodoIdComplex,
    registry::ui::{headings::H2, label::Label},
    utils::hooks::queries::queries_todos_complex::useTodoQueryComplex,
};

#[component]
pub fn TodoWithQueryComplex() -> impl IntoView {
    let (todo_id, set_todo_id) = create_signal(TodoIdComplex(0));

    let QueryResult {
        data, ..
    } = useTodoQueryComplex().use_query(move || todo_id.get());

    view! {
        <div class="flex flex-col gap-2 justify-between items-center p-4 rounded-md bg-neutral-200 w-[400px]">
            <H2>"Todo with Query"</H2>
            <Label>"Todo ID"</Label>

            <input
                type="number"
                on:input=move |ev| {
                    if let Ok(todo_id) = event_target_value(&ev).parse() {
                        set_todo_id(TodoIdComplex(todo_id));
                    }
                }
                prop:value=move || todo_id.get().0
            />

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                <div>
                    {move || {
                        data.get()
                            .map(|a| {
                                match a.ok().flatten() {
                                    Some(todo) => {
                                        view! {
                                            <div>
                                                <p>{format!("Content: {}", todo.content)}</p>
                                                <p>{format!("Description: {}", todo.description)}</p>
                                            </div>
                                        }
                                            .into_view()
                                    }
                                    None => view! { <p>"Not found 😢"</p> }.into_view(),
                                }
                            })
                    }}
                </div>
            </Transition>
        </div>
    }
}
