use leptos::*;

use crate::registry::demos::demo_visualizations::demo_chart_bar_horizontal::DemoChartBarHorizontal;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">

            <h1>"Test page"</h1>

            <DemoChartBarHorizontal />

        </div>
    }
}
