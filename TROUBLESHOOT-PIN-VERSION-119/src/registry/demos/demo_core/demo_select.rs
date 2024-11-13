use leptos::*;

// TODO UI.

#[component]
pub fn DemoSelect() -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let (selected, set_selected) = create_signal(String::from("Select an option"));

    view! {
        <div class="relative">
            <button
                on:click=move |_| set_open(!open())
                class="flex justify-between items-center py-2 px-4 rounded border border-gray-300 min-w-[180px] max-w-[180px]"
                class:text-black=move || selected() != "Select an option"
                class:text-gray-500=move || selected() == "Select an option"
            >
                <span class="overflow-hidden max-w-[120px]">{move || selected()}</span>
                <svg
                    class="ml-2 w-4 h-4"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 9l-7 7-7-7"
                    />
                </svg>
            </button>

            <div
                class="absolute z-10 mt-2 w-full bg-white rounded border"
                class:hidden=move || !open()
            >
                <ul class="overflow-auto max-h-[140px]">
                    <li
                        class="py-2 px-4 text-gray-500 cursor-pointer hover:bg-gray-100"
                        on:click=move |_| {
                            set_selected("Option 1".to_string());
                            set_open(false);
                        }
                    >
                        Option 1
                    </li>
                    <li
                        class="py-2 px-4 text-gray-500 cursor-pointer hover:bg-gray-100"
                        on:click=move |_| {
                            set_selected("Option 2".to_string());
                            set_open(false);
                        }
                    >
                        Option 2
                    </li>
                    <li
                        class="py-2 px-4 text-gray-500 cursor-pointer hover:bg-gray-100"
                        on:click=move |_| {
                            set_selected("Option 3".to_string());
                            set_open(false);
                        }
                    >
                        Option 3
                    </li>
                    <li
                        class="py-2 px-4 text-gray-500 cursor-pointer hover:bg-gray-100"
                        on:click=move |_| {
                            set_selected("Option 4".to_string());
                            set_open(false);
                        }
                    >
                        Option 4
                    </li>
                    <li
                        class="py-2 px-4 text-gray-500 cursor-pointer hover:bg-gray-100"
                        on:click=move |_| {
                            set_selected("Option 5".to_string());
                            set_open(false);
                        }
                    >
                        Option 5
                    </li>
                </ul>
            </div>
        </div>
    }
}
