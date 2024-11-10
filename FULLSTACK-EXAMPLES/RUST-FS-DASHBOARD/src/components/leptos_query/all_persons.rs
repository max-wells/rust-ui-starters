use leptos::*;
use leptos_query::*;

use crate::{
    api::api_persons::delete_person, components::{
        alert_dialogs::alert_dialog_delete_person::AlertDialogDeletePerson, icons::trash2::Trash2, toaster_custom::{toast::ToastVariant, types::Toasts}, ui::button::{Button, ButtonSize, ButtonVariant}
    },
    models::model_persons::{AllPersonsTag, PersonId, MyPerson},
    utils::{hooks::queries::queries_persons::useAllPersons, toast_utils::show_toast},
};

#[component]
pub fn AllPersons() -> impl IntoView {
    let QueryResult {
        data,
        state,
        refetch,
        ..
    } = useAllPersons().use_query(|| AllPersonsTag);

    let all_persons: Signal<Vec<MyPerson>> = Signal::derive(move || data.get().unwrap_or_default());

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

    let delete_person = create_action(move |id: &PersonId| {
        let id = *id;
        let refetch = refetch.clone();

        let all_persons_query = useAllPersons();
        
        async move {
            all_persons_query.cancel_query(AllPersonsTag);

            all_persons_query.update_query_data_mut(AllPersonsTag, |all_persons| {
                all_persons.retain(|person| person.id != id);
            });

            let _ = delete_person(id).await;

            refetch()
        }
    });

    // TODO. Understand 🐛 I was forced to use this signal to make AlertDialog work...
    // TODO. Why ? 🤔
    let (selected_person_id, set_selected_person_id) = create_signal(None::<PersonId>);

    let toast_context = expect_context::<Toasts>();

    let handle_delete_person = move |id: PersonId| {
        delete_person.dispatch(id);

        show_toast(
            toast_context,
            ToastVariant::Success,
            &format!("🗑️ Deleted person with ID: {}", id.0)
        );
    };

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All Server Persons"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <ul>
                    <Show
                        when=move || !all_persons.get().is_empty()
                        fallback=|| {
                            view! { <p>"No persons available"</p> }
                        }
                    >

                        <For
                            each=all_persons
                            key=|person| person.id
                            children=move |person| {
                                view! {
                                    <li>
                                        <span>"ID : "</span>
                                        <span>{person.id.0}</span>
                                        <span>"Title : "</span>
                                        <span>{person.title}</span>
                                        <span>"Name : "</span>
                                        <span>{person.name}</span>
                                        <span>"Level : "</span>
                                        <span>{person.level}</span>
                                        <span>"Compensation : "</span>
                                        <span>{person.compensation}</span>
                                        <span>"Joined Date : "</span>
                                        <span>
                                            {person.joined_date.format("%Y-%m-%d").to_string()}
                                        </span>

                                        <div class="flex gap-2">
                                            <DeleteWithButton
                                                server_person_id=person.id
                                                handle_delete=handle_delete_person
                                            />

                                            <AlertDialogDeletePerson
                                                server_person_id=person.id
                                                handle_delete=handle_delete_person
                                                set_selected_person_id=set_selected_person_id
                                                selected_person_id=selected_person_id.into()
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
fn DeleteWithButton(server_person_id: PersonId, handle_delete: impl Fn(PersonId) + 'static) -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Destructive
            size=ButtonSize::Sm
            on:click=move |_| { handle_delete(server_person_id) }
        >
            <Trash2 class="size-4" />
        </Button>
    }
}
