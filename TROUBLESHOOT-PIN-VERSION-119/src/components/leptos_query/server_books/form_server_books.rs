use leptos::*;
use leptos_router::ActionForm;
use wasm_bindgen::JsCast; // Import the JsCast trait

use crate::{
    api::api_server_books::AddServerBook,
    models::model_server_books::AllServerBooksTag,
    registry::ui::{button::Button, headings::H2, input::Input, label::Label},
    utils::hooks::queries::queries_server_books::{useAllServerBooksQuery, useServerBookQuery},
};

// TODO. Display loading state when adding book.

#[component]
pub fn FormServerBooks() -> impl IntoView {
    let add_server_book = create_server_action::<AddServerBook>();
    let response = add_server_book.value();

    let server_book_query = useServerBookQuery();
    let all_server_books_query = useAllServerBooksQuery();

    create_effect(move |_| {
        // If action is successful.
        if let Some(Ok(book)) = response.get() {
            all_server_books_query.cancel_query(AllServerBooksTag);

            // Optimistic update for all todos.
            all_server_books_query.update_query_data_mut(AllServerBooksTag, {
                let book = book.clone();
                |books| {
                    books.push(book);
                }
            });

            // Optimistic update for individual TodoResponse.
            let id = book.id;
            server_book_query.set_query_data(id, Ok(Some(book)));

            // Invalidate individual TodoResponse.
            server_book_query.invalidate_query(id);

            // Invalidate AllTodos.
            all_server_books_query.invalidate_query(AllServerBooksTag);
        }
    });

    // Function to handle form submission and alert the content
    let handle_submit = move |event: web_sys::SubmitEvent| {
        event.prevent_default();
        let form_data = web_sys::FormData::new_with_form(
            &event
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlFormElement>(),
        )
        .unwrap();
        let title = form_data.get("title").as_string().unwrap_or_default();
        let author = form_data.get("author").as_string().unwrap_or_default();

        web_sys::window()
            .unwrap()
            .alert_with_message(&format!(
                "{{ \"title\": \"{}\", \"author\": \"{}\" }}",
                title, author
            ))
            .unwrap();

        // Create an instance of AddTodoComplex with the form data
        let new_server_book = AddServerBook {
            title: title.clone(),
            author: author.clone(),
        };

        add_server_book.dispatch(new_server_book);
    };

    view! {
        <div class="flex flex-col gap-2">

            <H2>"FormServerBooks"</H2>

            <ActionForm action=add_server_book on:submit=handle_submit>
                <div class="flex flex-col gap-2 p-1 border max-w-[370px]">
                    <div class="flex gap-2 items-center">
                        <Label r#for="book-title">"Title"</Label>
                        <Input id="book-title" name="title" />
                    </div>
                    <div class="flex gap-2 items-center">
                        <Label r#for="book-author">"Author"</Label>
                        <Input id="book-author" name="author" />
                    </div>
                    <Button r#type="submit">"Add Book"</Button>
                </div>
            </ActionForm>
        </div>
    }
}
