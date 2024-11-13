use charts_rs::{Color, RadarChart, THEME_DARK};
use leptos::*;

#[component]
pub fn DemoChartRadar() -> impl IntoView {
    let radar_chart = RadarChart::new_with_theme(
        vec![
            (
                "Allocated Budget",
                vec![4200.0, 3000.0, 20000.0, 35000.0, 50000.0, 18000.0],
            )
                .into(),
            (
                "Actual Spending",
                vec![5000.0, 14000.0, 28000.0, 26000.0, 42000.0, 21000.0],
            )
                .into(),
        ],
        vec![
            ("Sales", 6500.0).into(),
            ("Administration", 16000.0).into(),
            ("Information Technology", 30000.0).into(),
            ("Customer Support", 38000.0).into(),
            ("Development", 52000.0).into(),
            ("Marketing", 25000.0).into(),
        ],
        THEME_DARK,
    );

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&radar_chart.svg().unwrap() /> }
        .into_view()
}
