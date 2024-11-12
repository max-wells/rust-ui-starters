use leptos::*;
use serde_json::Value;

// TODO. tabs_custom.rs
use crate::{
    constants::shortfix_json_content_primitives::SHORTFIX_JSON_CONTENT_TABS,
    registry::{
        demos::demo_core::demo_button_reactive::DemoButtonReactive,
        primitives::p_tabs_custom::{
            TabsContent, TabsCustom, TabsList, TabsTrigger, SHORTFIX_TABS_TRIGGER_CLASS,
        },
    },
};

#[component]
pub fn DemoTabsCustom() -> impl IntoView {
    let json_content_tabs = SHORTFIX_JSON_CONTENT_TABS;

    let parsed: Value = serde_json::from_str(json_content_tabs).unwrap();
    let file_content = parsed["files"][0]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let (file_content_signal, _set_file_content_signal) = create_signal(file_content);

    view! {
        <TabsCustom default_value="tab1" class="w-[500px]">
            <TabsList attr:aria-label="Rust UI - Preview and Code" class="border border-sky-500">
                <TabsTrigger value="tab1" attr:class=SHORTFIX_TABS_TRIGGER_CLASS>
                    "Preview"
                </TabsTrigger>
                <TabsTrigger value="tab2" attr:class=SHORTFIX_TABS_TRIGGER_CLASS>
                    "Code"
                </TabsTrigger>
            </TabsList>

            <TabsContent
                value="tab1"
                class="flex justify-center items-center bg-green-500 min-h-[200px]"
            >
                <DemoButtonReactive />
            </TabsContent>
            <TabsContent value="tab2">
                <pre class="overflow-x-auto p-4 text-xs rounded-md border bg-card border-input">
                    {move || file_content_signal.get().clone()}
                </pre>
            </TabsContent>
        </TabsCustom>
    }
}
