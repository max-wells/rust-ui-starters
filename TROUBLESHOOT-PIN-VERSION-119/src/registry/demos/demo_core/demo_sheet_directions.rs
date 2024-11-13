use leptos::*;

use crate::registry::ui::sheet::{
    SheetCancel, SheetContent, SheetDescription, SheetDirection, SheetTitle, SheetTrigger,
    SheetVariant,
};

#[component]
pub fn DemoSheetDirections() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <DemoSheet direction=SheetDirection::Top label="TOP" />
            <div class="flex gap-4">
                <DemoSheet direction=SheetDirection::Left label="LEFT" />
                <DemoSheet direction=SheetDirection::Right label="RIGHT" />
            </div>
            <DemoSheet direction=SheetDirection::Bottom label="BOTTOM" />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoSheet(direction: SheetDirection, label: &'static str) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    let toggle_sheet = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <div>
            <SheetTrigger on:click=toggle_sheet>{label}</SheetTrigger>

            <SheetContent is_open=is_open direction=direction>

                <SheetTitle>{"Sheet Title"}</SheetTitle>
                <SheetDescription>{"This is the content inside the sheet."}</SheetDescription>

                <SheetCancel on:click=toggle_sheet variant=SheetVariant::Destructive>
                    {"Cancel"}
                </SheetCancel>
            </SheetContent>
        </div>
    }
}
