use leptos::*;

use crate::{
    components::{
        icons::trash2::Trash2,
        ui::alert_dialog::{
            AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
            AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
        },
    },
    models::model_persons::PersonId,
};

#[component]
pub fn AlertDialogDeletePerson(
    server_person_id: PersonId,
    handle_delete: impl Fn(PersonId) + 'static,
    set_selected_person_id: impl Fn(Option<PersonId>) + 'static,
    selected_person_id: Signal<Option<PersonId>>,
) -> impl IntoView {
    view! {
        <>
            <AlertDialogTrigger
                variant=AlertDialogVariant::Destructive
                on:click=move |_| {
                    set_selected_person_id(Some(server_person_id));
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
                        if let Some(id) = selected_person_id.get() {
                            handle_delete(id);
                        }
                    }>"Continue"</AlertDialogSubmit>
                </AlertDialogFooter>
            </AlertDialogComponent>
        </>
    }
}
