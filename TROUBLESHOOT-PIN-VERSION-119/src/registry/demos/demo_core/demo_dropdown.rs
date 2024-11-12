use ev::MouseEvent;
use leptos::*;

#[component]
pub fn DemoDropdown() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    let toggle_dropdown = move |_| set_is_open.update(|v| *v = !*v);

    let handle_item_click = move |item: &str| {
        set_is_open.set(false);
        // You can add any common logic here
        println!("Clicked item: {}", item);
    };

    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class="flex items-center py-2.5 px-6 text-xs font-medium leading-normal text-white uppercase whitespace-nowrap bg-blue-500 rounded shadow-md transition duration-150 ease-in-out hover:bg-blue-600 hover:shadow-lg focus:bg-blue-600 focus:ring-0 focus:shadow-lg focus:outline-none active:bg-blue-700 active:shadow-lg"
            >
                "Dropdown button"
                <span class="ml-2">
                    <DropdownCaret />
                </span>
            </button>
            <ul class=move || {
                format!(
                    "absolute z-10 float-left mt-1 {} min-w-max list-none overflow-hidden rounded-lg border-none bg-white bg-clip-padding text-left text-base shadow-lg",
                    if is_open() { "" } else { "hidden" },
                )
            }>
                <DropdownItem
                    text="Action 1"
                    on_click=Callback::from(move |_| handle_item_click("Action 1"))
                />
                <DropdownItem
                    text="Action 2"
                    on_click=Callback::from(move |_| handle_item_click("Action 2"))
                />
                <DropdownItem
                    text="Action 3"
                    on_click=Callback::from(move |_| handle_item_click("Action 3"))
                />
            </ul>
        </div>
    }
}

#[component]
fn DropdownItem(text: &'static str, on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <li>
            <button
                class="block py-2 px-4 w-full text-sm font-normal text-left text-gray-700 whitespace-nowrap bg-transparent transition duration-150 ease-in-out hover:bg-gray-100"
                on:click=on_click
            >
                {text}
            </button>
        </li>
    }
}

#[component]
fn DropdownCaret() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            class="w-5 h-5"
        >
            <path
                fill-rule="evenodd"
                d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                clip-rule="evenodd"
            />
        </svg>
    }
}
