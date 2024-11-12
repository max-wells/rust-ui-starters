use leptos::*;
use leptos_use::use_window_scroll;

// TODO 🪝 Improve this 💪 (🐛 when changing to other page with window size)

#[component]
pub fn DemoUseWindowScroll() -> impl IntoView {
    let (x, y) = use_window_scroll();

    let div = document().create_element("div").unwrap();
    div.set_attribute("class", "absolute top-full left-full w-full h-full")
        .unwrap();

    document().body().unwrap().append_child(&div).unwrap();

    view! {
        <div>See scroll values in the lower right corner of the screen.</div>
        <div class="m-5 shadow-lg float area">
            <p class="mb-2">Scroll value</p>
            <div>x: {move || format!("{:.1}", x())} <br />y: {move || format!("{:.1}", y())}</div>
        </div>
    }
}
