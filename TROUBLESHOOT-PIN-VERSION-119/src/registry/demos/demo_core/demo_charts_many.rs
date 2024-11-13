use charts_rs::*;
use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoChartsMany() -> impl IntoView {
    let mut charts = MultiChart::new();
    charts.margin = (10.0).into();
    charts.background_color = Some((31, 29, 29, 150).into());

    let theme = "shadcn";
    let bar_chart = BarChart::new_with_theme(
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
        theme,
    );
    charts.add(ChildChart::Bar(bar_chart, None));

    let candlestick_chart = CandlestickChart::new_with_theme(
        vec![(
            "",
            vec![
                20.0, 34.0, 10.0, 38.0, 40.0, 35.0, 30.0, 50.0, 31.0, 38.0, 33.0, 44.0, 38.0, 15.0,
                5.0, 42.0,
            ],
        )
            .into()],
        vec![
            "2017-10-24".to_string(),
            "2017-10-25".to_string(),
            "2017-10-26".to_string(),
            "2017-10-27".to_string(),
        ],
        theme,
    );
    charts.add(ChildChart::Candlestick(candlestick_chart, None));

    let horizontal_bar_chart = HorizontalBarChart::new_with_theme(
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
        theme,
    );
    charts.add(ChildChart::HorizontalBar(horizontal_bar_chart, None));

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
    charts.add(ChildChart::Line(line_chart, None));

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
        theme,
    );

    charts.add(ChildChart::Pie(pie_chart, None));

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
        theme,
    );
    charts.add(ChildChart::Radar(radar_chart, None));

    let scatter_chart = ScatterChart::new_with_theme(
        vec![
            (
                "Female",
                vec![
                    161.2, 51.6, 167.5, 59.0, 159.5, 49.2, 157.0, 63.0, 155.8, 53.6, 170.0, 59.0,
                    159.1, 47.6, 166.0, 69.8, 176.2, 66.8, 160.2, 75.2, 172.5, 55.2, 170.9, 54.2,
                    172.9, 62.5, 153.4, 42.0, 160.0, 50.0, 147.2, 49.8, 168.2, 49.2, 175.0, 73.2,
                    157.0, 47.8, 167.6, 68.8, 159.5, 50.6, 175.0, 82.5, 166.8, 57.2, 176.5, 87.8,
                    170.2, 72.8,
                ],
            )
                .into(),
            (
                "Male",
                vec![
                    174.0, 65.6, 175.3, 71.8, 193.5, 80.7, 186.5, 72.6, 187.2, 78.8, 181.5, 74.8,
                    184.0, 86.4, 184.5, 78.4, 175.0, 62.0, 184.0, 81.6, 180.0, 76.6, 177.8, 83.6,
                    192.0, 90.0, 176.0, 74.6, 174.0, 71.0, 184.0, 79.6, 192.7, 93.8, 171.5, 70.0,
                    173.0, 72.4, 176.0, 85.9, 176.0, 78.8, 180.5, 77.8, 172.7, 66.2, 176.0, 86.4,
                    173.5, 81.8,
                ],
            )
                .into(),
        ],
        theme,
    );
    charts.add(ChildChart::Scatter(scatter_chart, None));

    let table_chart = TableChart::new_with_theme(
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
        theme,
    );
    charts.add(ChildChart::Table(table_chart, None));

    let svg_content = charts.svg().unwrap();

    view! {
        <Stylesheet id="radar-mini" href="/components-nooo/radwar-mini.css" />

        <div class="mx-auto w-full max-w-2xl">
            <p>"Hello"</p>
            <div inner_html=svg_content></div>
        </div>
    }
}
