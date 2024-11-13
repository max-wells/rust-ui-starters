use leptos::*;

use crate::registry::{
    icons::others::trash2::Trash2,
    ui::alert_dialog::{
        AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
        AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
    },
};

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    view! {
        <AlertDialogTrigger variant=AlertDialogVariant::Destructive>
            "Delete" <Trash2 class="ml-2 size-4" />
        </AlertDialogTrigger>

        <AlertDialogComponent>

            <AlertDialogTitle>This is a headline</AlertDialogTitle>
            <AlertDialogDescription>
                Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.
            </AlertDialogDescription>

            <AlertDialogFooter>
                <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                <AlertDialogSubmit>"Continue"</AlertDialogSubmit>
            </AlertDialogFooter>
        </AlertDialogComponent>
    }
}
