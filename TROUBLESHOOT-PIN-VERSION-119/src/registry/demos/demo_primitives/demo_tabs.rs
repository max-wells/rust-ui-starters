use leptos::*;
use serde_json::Value;

use crate::{
    constants::shortfix_json_content_primitives::SHORTFIX_JSON_CONTENT_TABS,
    registry::{
        demos::demo_core::demo_button_reactive::DemoButtonReactive,
        primitives::p_tabs::{
            PrimitiveTabsContent, PrimitiveTabsList, PrimitiveTabsRoot, PrimitiveTabsTrigger,
        },
    },
};

#[component]
pub fn DemoTabs() -> impl IntoView {
    let json_content_tabs = SHORTFIX_JSON_CONTENT_TABS;

    let parsed: Value = serde_json::from_str(json_content_tabs).unwrap();
    let file_content = parsed["files"][0]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let (file_content_signal, _set_file_content_signal) = create_signal(file_content);

    view! {
        <PrimitiveTabsRoot attr:class="w-[500px]" default_value="tab1">
            <PrimitiveTabsList attr:class=CLASS_TABS_LIST attr:aria-label="Manage your account">
                <PrimitiveTabsTrigger attr:class=CLASS_TABS_TRIGGER value="tab1">
                    "Preview"
                </PrimitiveTabsTrigger>
                <PrimitiveTabsTrigger attr:class=CLASS_TABS_TRIGGER value="tab2">
                    "Code"
                </PrimitiveTabsTrigger>
            </PrimitiveTabsList>

            <PrimitiveTabsContent attr:class=CLASS_TABS_CONTENT value="tab1">
                <div class="flex justify-center items-center bg-green-500 min-h-[200px]">
                    <DemoButtonReactive />
                </div>

            </PrimitiveTabsContent>
            <PrimitiveTabsContent attr:class=CLASS_TABS_CONTENT value="tab2">
                <pre class="overflow-x-auto p-4 text-xs rounded-md border bg-card border-input">
                    {move || file_content_signal.get().clone()}
                </pre>
            </PrimitiveTabsContent>
        </PrimitiveTabsRoot>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const CLASS_TABS_LIST: &str =
    "flex items-center justify-center w-full text-muted-foreground bg-muted  h-10 rounded-md p-1";

const CLASS_TABS_TRIGGER: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm   w-full";

const CLASS_TABS_CONTENT: &str = "mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background";
