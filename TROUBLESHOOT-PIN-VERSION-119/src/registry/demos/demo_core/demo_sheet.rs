use leptos::html::Div;
use leptos::*;
use leptos_use::on_click_outside;

use crate::registry::{
    hooks::use_lock_body_scroll::use_lock_body_scroll,
    ui::sheet::{
        SheetCancel, SheetContent, SheetDescription, SheetTitle, SheetTrigger, SheetVariant,
    },
};

#[component]
pub fn DemoSheet() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let (_scroll_locked, set_scroll_locked) = use_lock_body_scroll(false);
    let _sheet_ref = create_node_ref::<Div>();

    let toggle_sheet = move |_| {
        let new_state = !is_open.get();
        set_is_open.set(new_state);
        set_scroll_locked.set(new_state);
    };

    create_effect(move |_| {
        if is_open.get() {
            let _ = on_click_outside(_sheet_ref, move |_| {
                set_is_open.set(false);
                set_scroll_locked.set(false);
            });
        }
    });

    view! {
        <>
            <SheetTrigger on:click=toggle_sheet>"Open Sheet"</SheetTrigger>

            <div node_ref=_sheet_ref>
                <SheetContent is_open=is_open class="w-[400px]">
                    <SheetTitle>{"Sheet Title"}</SheetTitle>
                    <SheetDescription>{"This is the content inside the sheet."}</SheetDescription>

                    <SheetCancel on:click=toggle_sheet variant=SheetVariant::Destructive>
                        {"Cancel"}
                    </SheetCancel>
                </SheetContent>
            </div>
        </>
    }
}
