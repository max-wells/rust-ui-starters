use charts_rs::{Color, PieChart, THEME_DARK};
use leptos::*;

#[component]
pub fn DemoChartPie() -> impl IntoView {
    let pie_chart = PieChart::new_with_theme(
        vec![
            ("rose 1", vec![40.0]).into(),
            ("rose 2", vec![38.0]).into(),
            ("rose 3", vec![32.0]).into(),
            ("rose 4", vec![30.0]).into(),
            ("rose 5", vec![28.0]).into(),
            ("rose 6", vec![26.0]).into(),
            ("rose 7", vec![22.0]).into(),
            ("rose 8", vec![18.0]).into(),
        ],
        THEME_DARK,
    );

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&pie_chart.svg().unwrap() /> }
        .into_view()
}
