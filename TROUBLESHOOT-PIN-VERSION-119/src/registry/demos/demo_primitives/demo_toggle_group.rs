use leptos::*;

use crate::registry::icons::others::text_align_center::TextAlignCenter;
use crate::registry::icons::others::text_align_left::TextAlignLeft;
use crate::registry::icons::others::text_align_right::TextAlignRight;
use crate::registry::primitives::p_toggle_group::{
    ToggleGroupItem, ToggleGroupKind, ToggleGroupRoot, ToggleGroupSingle,
};

#[component]
pub fn DemoToggleGroup() -> impl IntoView {
    let toggle_group_item_classes = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground bg-transparent h-10 px-3";

    view! {
        <ToggleGroupRoot
            attr:class="flex items-center justify-center gap-1 w-fit"
            kind=ToggleGroupKind::Single {
                value: ToggleGroupSingle::none().into(),
                default_value: "center".into(),
                on_value_change: None,
            }
            // kind=AccordionKind::Multiple {
            // value: ToggleGroupMultiple::none(),
            // default_value: ToggleGroupMultiple::none(),
            // on_value_change: None
            // }
            attr:aria-label="Text alignment"
        >
            <ToggleGroupItem
                value="left"
                attr:class=toggle_group_item_classes
                attr:aria-label="Left aligned"
            >
                <TextAlignLeft />
            </ToggleGroupItem>
            <ToggleGroupItem
                value="center"
                attr:class=toggle_group_item_classes
                attr:aria-label="Center aligned"
            >
                <TextAlignCenter />
            </ToggleGroupItem>
            <ToggleGroupItem
                value="right"
                attr:class=toggle_group_item_classes
                attr:aria-label="Right aligned"
            >
                <TextAlignRight />
            </ToggleGroupItem>
        </ToggleGroupRoot>
    }
}
