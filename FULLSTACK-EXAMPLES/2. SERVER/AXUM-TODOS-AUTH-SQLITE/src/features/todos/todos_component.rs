use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::Input;
use crate::error_template::ErrorTemplate;
use crate::features::todos::todos_services::{get_todos, AddTodo, DeleteTodo};

#[component]
pub fn TodosComponent() -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let delete_todo = ServerAction::<DeleteTodo>::new();
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = Resource::new(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    view! {
        <div class="mt-4">
            <MultiActionForm action=add_todo>
                <div class="flex max-w-md gap-4 p-4 border rounded-md bg-accent">
                    <label>"Add a Todo"</label>
                    <Input r#type="text" name="title" />
                    <Button r#type="submit">"Add"</Button>
                </div>
            </MultiActionForm>

            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorTemplate errors=errors /> }
                }>
                    {move || {
                        let existing_todos = {
                            move || {
                                todos
                                    .get()
                                    .map(move |todos| match todos {
                                        Err(e) => {
                                            view! {
                                                <pre class="error">"Server Error: " {e.to_string()}</pre>
                                            }
                                                .into_any()
                                        }
                                        Ok(todos) => {
                                            if todos.is_empty() {
                                                view! { <p>"No tasks were found."</p> }.into_any()
                                            } else {
                                                todos
                                                    .into_iter()
                                                    .map(move |todo| {
                                                        view! {
                                                            <li>
                                                                {todo.title} ": Created at " {todo.created_at} " by "
                                                                {todo.user.unwrap_or_default().username}
                                                                <ActionForm action=delete_todo>
                                                                    <input type="hidden" name="id" value=todo.id />
                                                                    <Button r#type="submit" variant=ButtonVariant::Destructive>
                                                                        "X"
                                                                    </Button>
                                                                </ActionForm>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }
                                        }
                                    })
                                    .unwrap_or(().into_any())
                            }
                        };
                        let pending_todos = move || {
                            submissions
                                .get()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        <li class="pending">
                                            {move || submission.input().get().map(|data| data.title)}
                                        </li>
                                    }
                                })
                                .collect_view()
                        };
                        view! { <ul>{existing_todos} {pending_todos}</ul> }
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
    }
}