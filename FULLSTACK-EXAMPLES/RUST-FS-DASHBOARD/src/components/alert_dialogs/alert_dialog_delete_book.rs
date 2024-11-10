use leptos::*;

use crate::{
    components::{
        icons::trash2::Trash2,
        ui::alert_dialog::{
            AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
            AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
        },
    },
    models::model_books::BookId,
};

#[component]
pub fn AlertDialogDeleteBook(
    server_book_id: BookId,
    handle_delete: impl Fn(BookId) + 'static,
    set_selected_book_id: impl Fn(Option<BookId>) + 'static,
    selected_book_id: Signal<Option<BookId>>,
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
