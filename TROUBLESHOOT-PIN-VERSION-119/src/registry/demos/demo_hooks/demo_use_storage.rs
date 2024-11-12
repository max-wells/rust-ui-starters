use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_use::docs::Note;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::registry::ui::{button::Button, input::Input};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BananaState {
    pub name: String,
    pub wearing: String,
    pub descending: String,
    pub count: u32,
}

impl Default for BananaState {
    fn default() -> Self {
        Self {
            name: "Bananas".to_string(),
            wearing: "pyjamas".to_string(),
            descending: "stairs".to_string(),
            count: 2,
        }
    }
}

// TODO UI. Inputs for all the fields.

#[component]
pub fn DemoUseStorage() -> impl IntoView {
    let (state, set_state, reset) =
        use_local_storage::<BananaState, JsonSerdeCodec>("banana-state");
    let (state2, _, _) = use_local_storage::<BananaState, JsonSerdeCodec>("banana-state");

    let name_signal = create_signal(state.get().name.clone());
    let wearing_signal = create_signal(state.get().wearing.clone());
    let descending_signal = create_signal(state.get().descending.clone());
    let count_signal = create_signal(state.get().count.to_string());

    view! {
        <div class="flex flex-col gap-2">
            <Input
                class="block"
                r#type="text"
                value=Some(name_signal.0)
                on:input=move |e| set_state.update(|s| s.name = event_target_value(&e))
            />
            <Input
                class="block"
                r#type="text"
                value=Some(wearing_signal.0)
                on:input=move |e| set_state.update(|s| s.wearing = event_target_value(&e))
            />
            <Input
                class="block"
                r#type="text"
                value=Some(descending_signal.0)
                on:input=move |e| set_state.update(|s| s.descending = event_target_value(&e))
            />
            <Input
                class="block"
                r#type="number"
                value=Some(count_signal.0)
                on:input=move |e| {
                    set_state
                        .update(|s| s.count = event_target_value(&e).parse::<f64>().unwrap() as u32)
                }
                min="0".to_string()
                step="1".to_string()
                max="1000".to_string()
            />
            <Button on:click=move |_| reset()>"Delete from storage"</Button>

            <p>
                "Second " <b>
                    <code>"use_storage"</code>
                </b> ":"
            </p>

            <pre>{move || format!("{:#?}", state2.get())}</pre>

            <Note>
                "The values are persistent. When you reload the page or "
                <a href="#" target="_blank">
                    "open a second window"
                </a> ", the values will be the same."
            </Note>
        </div>
    }
}
