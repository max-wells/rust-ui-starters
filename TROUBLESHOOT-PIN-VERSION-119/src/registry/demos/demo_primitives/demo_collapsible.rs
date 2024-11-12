use leptos::*;

use crate::registry::{
    icons::chevrons::chevron_up_down::ChevronUpDown,
    primitives::p_collapsible::{
        PrimitiveCollapsibleContent, PrimitiveCollapsibleRoot, PrimitiveCollapsibleTrigger,
    },
    ui::button::{Button, ButtonSize, ButtonVariant},
};

#[component]
pub fn DemoCollapsible() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <PrimitiveCollapsibleRoot
            attr:class="w-[300px]"
            open=open
            on_open_change=move |open: bool| set_open.set(open)
        >
            <div class="flex justify-between items-center px-2">
                <span class="text-sm font-semibold">"@peduarte starred 3 repositories"</span>
                <PrimitiveCollapsibleTrigger as_child=true>
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                        <ChevronUpDown class="size-4" />
                    </Button>
                </PrimitiveCollapsibleTrigger>
            </div>

            <div class="py-3 px-4 text-sm rounded-md border">
                <span>"leptix/primitives"</span>
            </div>

            <PrimitiveCollapsibleContent>
                <div class="py-3 px-4 text-sm rounded-md border">
                    <span>"@radix-ui/colors"</span>
                </div>
                <div class="py-3 px-4 text-sm rounded-md border">
                    <span>"@stitches/react"</span>
                </div>
            </PrimitiveCollapsibleContent>
        </PrimitiveCollapsibleRoot>
    }
}
