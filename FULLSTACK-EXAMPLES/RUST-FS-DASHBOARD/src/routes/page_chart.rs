use leptos::*;

use crate::components::dashboard_chart_leptos::DashboardChartLeptos;

#[component]
pub fn PageChart() -> impl IntoView {
    view! {
        <div>
            <DashboardChartLeptos />
        </div>
    }
}
