use leptos::*;
use leptos_query::*;

use crate::{
    api::api_books::delete_book, components::{
        alert_dialogs::alert_dialog_delete_book::AlertDialogDeleteBook, icons::trash2::Trash2, toaster_custom::{toast::ToastVariant, types::Toasts}, ui::button::{Button, ButtonSize, ButtonVariant}
    },
    models::model_books::{AllBooksTag, BookId, MyBook},
    utils::{hooks::queries::queries_books::useAllBooks, toast_utils::show_toast},
};

#[component]
pub fn AllBooks() -> impl IntoView {
    let QueryResult {
        data,
        state,
        refetch,
        ..
    } = useAllBooks().use_query(|| AllBooksTag);

    let all_books: Signal<Vec<MyBook>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_book = create_action(move |id: &BookId| {
        let id = *id;
        let refetch = refetch.clone();

        let all_books_query = useAllBooks();
        
        async move {
            all_books_query.cancel_query(AllBooksTag);

            all_books_query.update_query_data_mut(AllBooksTag, |all_books| {
                all_books.retain(|book| book.id != id);
            });

            let _ = delete_book(id).await;

            refetch()
        }
    });

    // TODO. Understand 🐛 I was forced to use this signal to make AlertDialog work...
    // TODO. Why ? 🤔
    let (selected_book_id, set_selected_book_id) = create_signal(None::<BookId>);

    let toast_context = expect_context::<Toasts>();

    let handle_delete_book = move |id: BookId| {
        delete_book.dispatch(id);

        show_toast(
            toast_context,
            ToastVariant::Success,
            &format!("🗑️ Deleted book with ID: {}", id.0)
        );
    };

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All Server Books"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !all_books.get().is_empty()
                        fallback=|| {
                            view! { <p>"No books available"</p> }
                        }
                    >

                        <For
                            each=all_books
                            key=|book| book.id
                            children=move |book| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{book.id.0}</span>
                                        <span>"Title : "</span>
                                        <span>{book.title}</span>
                                        <span>"Author : "</span>
                                        <span>{book.author}</span>

                                        <div class="flex gap-2">
                                            <DeleteWithButton
                                                server_book_id=book.id
                                                handle_delete=handle_delete_book
                                            />

                                            <AlertDialogDeleteBook
                                                server_book_id=book.id
                                                handle_delete=handle_delete_book
                                                set_selected_book_id=set_selected_book_id
                                                selected_book_id=selected_book_id.into()
                                            />
                                        </div>
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



/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/



#[component]
fn DeleteWithButton(server_book_id: BookId, handle_delete: impl Fn(BookId) + 'static) -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Destructive
            size=ButtonSize::Sm
            on:click=move |_| { handle_delete(server_book_id) }
        >
            <Trash2 class="size-4" />
        </Button>
    }
}
