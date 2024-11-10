use leptos::*;
use leptos_query::*;

use crate::{
    api::api_blogs::delete_blog, components::{
        alert_dialogs::alert_dialog_delete_blog::AlertDialogDeleteBlog, icons::trash2::Trash2, toaster_custom::{toast::ToastVariant, types::Toasts}, ui::{button::{Button, ButtonSize, ButtonVariant}, card::{Card, CardDescription, CardTitle}}
    },
    models::model_blogs::{AllBlogsQKey, BlogId, MyBlog},
    utils::{hooks::queries::queries_blogs::useAllBlogs, toast_utils::show_toast},
};

#[component]
pub fn AllBlogs() -> impl IntoView {
    let QueryResult {
        data,
        state,
        refetch,
        ..
    } = useAllBlogs().use_query(|| AllBlogsQKey);

    let all_blogs: Signal<Vec<MyBlog>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_blog = create_action(move |id: &BlogId| {
        let id = *id;
        let refetch = refetch.clone();

        let all_blogs_query = useAllBlogs();
        
        async move {
            all_blogs_query.cancel_query(AllBlogsQKey);

            all_blogs_query.update_query_data_mut(AllBlogsQKey, |all_blogs| {
                all_blogs.retain(|blog| blog.id != id);
            });

            let _ = delete_blog(id).await;

            refetch()
        }
    });

    // TODO. Understand 🐛 I was forced to use this signal to make AlertDialog work...
    // TODO. Why ? 🤔
    let (selected_blog_id, set_selected_blog_id) = create_signal(None::<BlogId>);

    let toast_context = expect_context::<Toasts>();

    let handle_delete_blog = move |id: BlogId| {
        delete_blog.dispatch(id);

        show_toast(
            toast_context,
            ToastVariant::Success,
            &format!("🗑️ Deleted blog with ID: {}", id.0)
        );
    };

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All Server Blogs"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <div class="flex flex-wrap gap-4">
                    <Show
                        when=move || !all_blogs.get().is_empty()
                        fallback=|| {
                            view! { <p>"No blogs available"</p> }
                        }
                    >

                        <For
                            each=all_blogs
                            key=|blog| blog.id
                            children=move |blog| {
                                view! {
                                    <Card class="flex relative flex-col gap-2 w-[400px]">
                                        <CardTitle>{blog.title}</CardTitle>
                                        <CardDescription>{blog.author}</CardDescription>
                                        <img src=blog.image_url class="w-full h-auto" />

                                        <div class="flex absolute right-2 top-4 gap-2">
                                            <DeleteWithButton
                                                server_blog_id=blog.id
                                                handle_delete=handle_delete_blog
                                            />

                                            <AlertDialogDeleteBlog
                                                server_blog_id=blog.id
                                                handle_delete=handle_delete_blog
                                                set_selected_blog_id=set_selected_blog_id
                                                selected_blog_id=selected_blog_id.into()
                                            />
                                        </div>
                                    </Card>
                                }
                            }
                        />

                    </Show>
                </div>

            </Transition>

        </div>
    }
}



/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/



#[component]
fn DeleteWithButton(server_blog_id: BlogId, handle_delete: impl Fn(BlogId) + 'static) -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Destructive
            size=ButtonSize::Sm
            on:click=move |_| { handle_delete(server_blog_id) }
        >
            <Trash2 class="size-4" />
        </Button>
    }
}
