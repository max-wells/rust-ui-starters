use leptos::html::Div;
use leptos::*;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};

// TODO. Improve scrollbar

#[component]
pub fn DemoUseInfiniteScroll() -> impl IntoView {
    let el = create_node_ref::<Div>();

    let (data, set_data) = create_signal(vec![1, 2, 3, 4, 5, 6]);

    let _ = use_infinite_scroll_with_options(
        el,
        move |_| async move {
            let len = data.with_untracked(|d| d.len());
            set_data.update(|data| *data = (1..len + 6).collect());
        },
        UseInfiniteScrollOptions::default().distance(10.0),
    );

    view! {
        <div
            node_ref=el
            class="flex overflow-y-scroll flex-col gap-2 p-4 m-auto rounded w-[300px] h-[300px] bg-gray-500/5"
        >
            <For each=move || data.get() key=|i| *i let:item>
                <div class="p-3 rounded h-15 bg-gray-500/5">{item}</div>
            </For>
        </div>
    }
}
