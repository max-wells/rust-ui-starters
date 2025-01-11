use leptos::*;
use leptos_query::*;

use crate::{
    components::ui::badge::Badge,
    models::model_tags::{AllTagsQKey, MyTag},
    utils::hooks::queries::queries_tags::useAllTags,
};

#[component]
pub fn AllTags() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllTags().use_query(|| AllTagsQKey);

    let all_tags: Signal<Vec<MyTag>> = Signal::derive(move || data.get().unwrap_or_default());

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

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All TAGS"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <div class="flex flex-wrap gap-2">
                    <Show
                        when=move || !all_tags.get().is_empty()
                        fallback=|| {
                            view! { <p>"No tags available"</p> }
                        }
                    >
                        <For
                            each=all_tags
                            key=|tag| tag.id
                            children=move |tag| {
                                view! { <Badge>{tag.name}</Badge> }
                            }
                        />
                    </Show>
                </div>
            </Transition>
        </div>
    }
}
