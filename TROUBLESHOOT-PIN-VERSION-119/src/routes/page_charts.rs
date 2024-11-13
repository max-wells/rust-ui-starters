use leptos::*;

use crate::registry::demos::demo_core::demo_charts_many::DemoChartsMany;

#[component]
pub fn PageCharts() -> impl IntoView {
    view! {
        <div class="container flex flex-col gap-10 mx-auto">
            <DemoChartsMany />
        </div>
    }
}
