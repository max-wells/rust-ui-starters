use html::Div;
use leptos::*;
use leptos_use::on_click_outside;
// use leptos_router::A;

use crate::registry::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

// TODO. Shortfix 🚑 using window because A is not working inside DropdownMenu...

#[component]
pub fn TestToggleTemporary() -> impl IntoView {
    let dropdown_open = create_rw_signal(false);
    let dropdown_ref = create_node_ref::<Div>();

    let toggle_dropdown = move |_| dropdown_open.update(|open| *open = !*open);

    // Close the dropdown when clicking outside
    let _ = on_click_outside(dropdown_ref, move |_| dropdown_open.set(false));

    let MENU_ITEMS = [
        ("Test Page", "/test"),
        ("Json", "/json"),
        ("Forms", "/forms"),
        ("Charts", "/charts"),
        ("Css Tricks", "/css-tricks"),
        ("Parallax", "/parallax"),
        ("Parallax 1", "/parallax1"),
        ("Bento Grids", "/bento-grids"),
        ("Animated For", "/animated-for"),
        ("Leptos Query Todos", "/leptos-query-todos"),
        ("Leptos Query Todos Complex", "/leptos-query-todos-complex"),
        ("Leptos Query Todos Json", "/leptos-query-todos-json"),
        ("Leptos Query Users Json", "/leptos-query-users-json"),
        ("Leptos Query Users Url", "/leptos-query-users-url"),
        ("Leptos Query Server Books", "/leptos-query-server-books"),
    ];

    view! {
        <DropdownMenu node_ref=dropdown_ref>
            <DropdownMenuTrigger on:click=toggle_dropdown>"Test"</DropdownMenuTrigger>

            <DropdownMenuContent is_open=dropdown_open>
                <div class="py-1" role="none">
                    {MENU_ITEMS
                        .iter()
                        .map(|(label, href)| {
                            let label = label.to_string();
                            let href = href.to_string();
                            view! {
                                <DropdownMenuItem on_click=move |_| {
                                    web_sys::window().unwrap().location().set_href(&href).unwrap();
                                }>{label}</DropdownMenuItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
