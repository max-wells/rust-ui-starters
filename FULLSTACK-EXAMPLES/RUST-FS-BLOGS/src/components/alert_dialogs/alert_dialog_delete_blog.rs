use leptos::*;

use crate::{
    components::{
        icons::trash2::Trash2,
        ui::alert_dialog::{
            AlertDialogCancel, AlertDialogComponent, AlertDialogDescription, AlertDialogFooter,
            AlertDialogSubmit, AlertDialogTitle, AlertDialogTrigger, AlertDialogVariant,
        },
    },
    models::model_blogs::BlogId,
};

#[component]
pub fn AlertDialogDeleteBlog(
    server_blog_id: BlogId,
    handle_delete: impl Fn(BlogId) + 'static,
    set_selected_blog_id: impl Fn(Option<BlogId>) + 'static,
    selected_blog_id: Signal<Option<BlogId>>,
) -> impl IntoView {
    view! {
        <>
            <AlertDialogTrigger
                variant=AlertDialogVariant::Destructive
                on:click=move |_| {
                    set_selected_blog_id(Some(server_blog_id));
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
                        if let Some(id) = selected_blog_id.get() {
                            handle_delete(id);
                        }
                    }>"Continue"</AlertDialogSubmit>
                </AlertDialogFooter>
            </AlertDialogComponent>
        </>
    }
}
