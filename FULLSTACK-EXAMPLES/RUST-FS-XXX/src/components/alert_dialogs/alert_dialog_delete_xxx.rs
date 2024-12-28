use leptos::*;

use crate::{
    components::{
        icons::trash2::Trash2,
        ui::alert_dialog::{
            AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
            AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
        },
    },
    models::model_xxxs::XxxId,
};

#[component]
pub fn AlertDialogDeleteBook(
    server_xxx_id: XxxId,
    handle_delete: impl Fn(XxxId) + 'static,
    set_selected_xxx_id: impl Fn(Option<XxxId>) + 'static,
    selected_xxx_id: Signal<Option<XxxId>>,
) -> impl IntoView {
    view! {
        <>
            <AlertDialogTrigger
                variant=AlertDialogVariant::Destructive
                on:click=move |_| {
                    set_selected_xxx_id(Some(server_xxx_id));
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
                        if let Some(id) = selected_xxx_id.get() {
                            handle_delete(id);
                        }
                    }>"Continue"</AlertDialogSubmit>
                </AlertDialogFooter>
            </AlertDialogComponent>
        </>
    }
}
