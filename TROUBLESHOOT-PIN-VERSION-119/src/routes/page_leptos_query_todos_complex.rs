use leptos::*;

use crate::{
    components::leptos_query::todos_complex::{
        all_todos_complex::AllTodosComplex, form_todo_complex::FormTodoComplex,
        todo_with_query_complex::TodoWithQueryComplex,
    },
    models::model_todo::Todo,
};

#[component]
pub fn PageLeptosQueryTodosComplex() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            <div class="flex gap-10">
                <TodoWithQueryComplex />
            </div>
            <FormTodoComplex />
            <AllTodosComplex />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// * 💁 When using this, you get a ton of hydration errors.
#[component]
fn TodoBody(todo: Signal<Option<Option<Todo>>>) -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            <p>
                {move || {
                    todo.get()
                        .map(|a| {
                            match a {
                                Some(todo) => todo.content,
                                None => "Not found".into(),
                            }
                        })
                }}

            </p>
        </Suspense>
    }
}
