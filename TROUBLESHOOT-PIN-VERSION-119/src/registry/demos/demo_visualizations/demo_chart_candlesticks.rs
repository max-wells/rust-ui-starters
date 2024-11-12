use charts_rs::{CandlestickChart, Color, THEME_DARK};
use leptos::*;

#[component]
pub fn DemoChartCandlesticks() -> impl IntoView {
    let data = vec![
        20.0, 34.0, 10.0, 38.0, 40.0, 35.0, 30.0, 50.0, 31.0, 38.0, 33.0, 44.0, 38.0, 15.0, 5.0,
        42.0,
    ];

    let dates = vec![
        "2017-10-24".to_string(),
        "2017-10-25".to_string(),
        "2017-10-26".to_string(),
        "2017-10-27".to_string(),
    ];

    let mut candlestick_chart =
        CandlestickChart::new_with_theme(vec![("", data).into()], dates, THEME_DARK);

    candlestick_chart.font_family = String::from("Noto Sans SC");
    candlestick_chart.background_color = Color::transparent();
    candlestick_chart.width = 550.0;
    candlestick_chart.height = 400.0;

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&candlestick_chart.svg().unwrap() /> }
        .into_view()
}
