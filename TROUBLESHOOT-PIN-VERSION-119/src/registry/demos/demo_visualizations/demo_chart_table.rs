use charts_rs::{TableChart, THEME_DARK};
use leptos::*;

#[component]
pub fn DemoChartTable() -> impl IntoView {
    let mut table_chart = TableChart::new_with_theme(
        vec![
            vec![
                "Name".to_string(),
                "Price".to_string(),
                "Change".to_string(),
            ],
            vec![
                "Datadog Inc".to_string(),
                "97.32".to_string(),
                "-7.49%".to_string(),
            ],
            vec![
                "Hashicorp Inc".to_string(),
                "28.66".to_string(),
                "-9.25%".to_string(),
            ],
            vec![
                "Gitlab Inc".to_string(),
                "51.63".to_string(),
                "+4.32%".to_string(),
            ],
        ],
        THEME_DARK,
    );

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&table_chart.svg().unwrap() /> }
        .into_view()
}
