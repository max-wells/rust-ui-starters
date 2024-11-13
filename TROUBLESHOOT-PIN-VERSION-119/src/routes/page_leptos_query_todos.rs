use leptos::*;

use crate::{
    components::leptos_query::todos_simple::{
        all_todos::AllTodos, form_todo::FormTodo, todo_with_query::TodoWithQuery,
        todo_with_resource::TodoWithResource,
    },
    models::model_todo::Todo,
};

#[component]
pub fn PageLeptosQueryTodos() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            <div class="flex gap-10">
                <TodoWithResource />
                <TodoWithQuery />
            </div>
            <FormTodo />
            <AllTodos />
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
