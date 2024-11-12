use charts_rs::LineChart;
use leptos::*;

#[component]
pub fn DemoChartLine() -> impl IntoView {
    let line_chart = LineChart::new_with_theme(
        vec![
            (
                "Email",
                vec![120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0],
            )
                .into(),
            (
                "Union Ads",
                vec![220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0],
            )
                .into(),
            (
                "Direct",
                vec![320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0],
            )
                .into(),
            (
                "Search Engine",
                vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0],
            )
                .into(),
        ],
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        "shadcn",
    );

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&line_chart.svg().unwrap() /> }
        .into_view()
}
