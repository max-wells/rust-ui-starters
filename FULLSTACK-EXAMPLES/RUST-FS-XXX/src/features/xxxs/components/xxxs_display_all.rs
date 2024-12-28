use leptos::*;
use leptos_query::*;

use crate::{
    api::api_xxxs::delete_xxx, components::{
        icons::trash2::Trash2, toaster_custom::{toast::ToastVariant, types::Toasts}, ui::button::{Button, ButtonSize, ButtonVariant}
    }, features::xxxs::{components::xxxs_alert_dialog_delete::XxxsAlertDialogDelete, models::{xxxs_models::{MyXxx, TagAllXxxs, XxxId}}, hooks::xxxs_queries::useAllXxxs}, utils::toast_utils::show_toast
};

#[component]
pub fn XxxsDisplayAll() -> impl IntoView {
    let QueryResult {
        data,
        state,
        refetch,
        ..
    } = useAllXxxs().use_query(|| TagAllXxxs);

    let all_xxxs: Signal<Vec<MyXxx>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_xxx = create_action(move |id: &XxxId| {
        let id = *id;
        let refetch = refetch.clone();

        let all_xxxs_query = useAllXxxs();
        
        async move {
            all_xxxs_query.cancel_query(TagAllXxxs);

            all_xxxs_query.update_query_data_mut(TagAllXxxs, |all_xxxs| {
                all_xxxs.retain(|xxx| xxx.id != id);
            });

            let _ = delete_xxx(id).await;

            refetch()
        }
    });

    // TODO. Understand 🐛 I was forced to use this signal to make AlertDialog work...
    // TODO. Why ? 🤔
    let (selected_xxx_id, set_selected_xxx_id) = create_signal(None::<XxxId>);

    let toast_context = expect_context::<Toasts>();

    let handle_delete_xxx = move |id: XxxId| {
        delete_xxx.dispatch(id);

        show_toast(
            toast_context,
            ToastVariant::Success,
            &format!("🗑️ Deleted xxx with ID: {}", id.0)
        );
    };

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All Server Xxxs"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !all_xxxs.get().is_empty()
                        fallback=|| {
                            view! { <p>"No xxxs available"</p> }
                        }
                    >

                        <For
                            each=all_xxxs
                            key=|xxx| xxx.id
                            children=move |xxx| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{xxx.id.0}</span>
                                        <span>"Field One : "</span>
                                        <span>{xxx.field_one}</span>
                                        <span>"Field Two : "</span>
                                        <span>{xxx.field_two}</span>

                                        <div class="flex gap-2">
                                            <DeleteWithButton
                                                server_xxx_id=xxx.id
                                                handle_delete=handle_delete_xxx
                                            />

                                            <XxxsAlertDialogDelete
                                                server_xxx_id=xxx.id
                                                handle_delete=handle_delete_xxx
                                                set_selected_xxx_id=set_selected_xxx_id
                                                selected_xxx_id=selected_xxx_id.into()
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
fn DeleteWithButton(server_xxx_id: XxxId, handle_delete: impl Fn(XxxId) + 'static) -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Destructive
            size=ButtonSize::Sm
            on:click=move |_| { handle_delete(server_xxx_id) }
        >
            <Trash2 class="size-4" />
        </Button>
    }
}
