use charts_rs::{HorizontalBarChart, THEME_DARK};
use leptos::*;

#[component]
pub fn DemoChartBarHorizontal() -> impl IntoView {
    let bar_chart_horizontal = HorizontalBarChart::new_with_theme(
        vec![
            (
                "2011",
                vec![18203.0, 23489.0, 29034.0, 104970.0, 131744.0, 630230.0],
            )
                .into(),
            (
                "2012",
                vec![19325.0, 23438.0, 31000.0, 121594.0, 134141.0, 681807.0],
            )
                .into(),
        ],
        vec![
            "Brazil".to_string(),
            "Indonesia".to_string(),
            "USA".to_string(),
            "India".to_string(),
            "China".to_string(),
            "World".to_string(),
        ],
        THEME_DARK,
    );

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&bar_chart_horizontal.svg().unwrap() /> }
        .into_view()
}
