use leptos::*;
use leptos_query::*;

use crate::{
    api::api_server_books::delete_server_book_from_client,
    models::model_server_books::{AllServerBooksTag, ServerBook, ServerBookId},
    registry::{
        icons::others::trash2::Trash2,
        ui::button::{Button, ButtonSize, ButtonVariant},
        ui::alert_dialog::{
            AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
            AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
        },
    },
    utils::hooks::queries::queries_server_books::{useAllServerBooksQuery, useServerBookQuery},
};

#[component]
pub fn AllServerBooks() -> impl IntoView {
    let QueryResult {
        data,
        state,
        refetch,
        ..
    } = useAllServerBooksQuery().use_query(|| AllServerBooksTag);

    let all_server_books: Signal<Vec<ServerBook>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_server_book = create_action(move |id: &ServerBookId| {
        let id = *id;
        let refetch = refetch.clone();

        let server_book_query = useServerBookQuery();
        let all_server_books_query = useAllServerBooksQuery();
        
        async move {
            all_server_books_query.cancel_query(AllServerBooksTag);

            all_server_books_query.update_query_data_mut(AllServerBooksTag, |server_books| {
                server_books.retain(|server_book| server_book.id != id);
            });

            server_book_query.set_query_data(id, Ok(None));

            let _ = delete_server_book_from_client(id).await;

            let _ = server_book_query.invalidate_query(id);

            refetch()
        }
    });

    // TODO. Understand 🐛 I was forced to use this signal to make AlertDialog work...
    // TODO. Why ? 🤔
    let (selected_book_id, set_selected_book_id) = create_signal(None::<ServerBookId>);

    let handle_delete_server_book = move |id: ServerBookId| {
        logging::log!("Deleting server book with ID: {:?}", id);
        delete_server_book.dispatch(id);
    };

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-neutral-300">

            <h2 class="text-lg font-bold">"All Server Books"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !all_server_books.get().is_empty()
                        fallback=|| {
                            view! { <p>"No server books"</p> }
                        }
                    >

                        <For
                            each=all_server_books
                            key=|server_book| server_book.id
                            children=move |server_book| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{server_book.id.0}</span>
                                        <span>"Title : "</span>
                                        <span>{server_book.title}</span>
                                        <span>"Author : "</span>
                                        <span>{server_book.author}</span>

                                        <div class="flex gap-2">
                                            <DeleteWithButton
                                                server_book_id=server_book.id
                                                handle_delete=handle_delete_server_book
                                            />

                                            <DeleteWithAlertDialog
                                                server_book_id=server_book.id
                                                handle_delete=handle_delete_server_book
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
fn DeleteWithButton(server_book_id: ServerBookId, handle_delete: impl Fn(ServerBookId) + 'static) -> impl IntoView {
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

#[component]
fn DeleteWithAlertDialog(
    server_book_id: ServerBookId,
    handle_delete: impl Fn(ServerBookId) + 'static,
    set_selected_book_id: impl Fn(Option<ServerBookId>) + 'static,
    selected_book_id: Signal<Option<ServerBookId>>,
) -> impl IntoView {
    view! {
        <>
            <AlertDialogTrigger
                variant=AlertDialogVariant::Destructive
                on:click=move |_| {
                    set_selected_book_id(Some(server_book_id));
                }
            >
                <Trash2 class="size-4" />
            </AlertDialogTrigger>
            <AlertDialogComponent>
                <AlertDialogTitle>This is a headline</AlertDialogTitle>
                <AlertDialogDescription>
                    Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.
                </AlertDialogDescription>

                <AlertDialogFooter>
                    <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                    <AlertDialogSubmit on:click=move |_| {
                        if let Some(id) = selected_book_id.get() {
                            handle_delete(id);
                        }
                    }>"Continue"</AlertDialogSubmit>
                </AlertDialogFooter>
            </AlertDialogComponent>
        </>
    }
}