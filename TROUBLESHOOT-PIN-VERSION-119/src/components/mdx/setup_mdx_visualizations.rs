use crate::{
    __registry__::mdx_demos_visualizations::{
        mdx_demo_chart_table::MdxDemoChartTable,
        mdx_demo_chart_pie::MdxDemoChartPie,
        mdx_demo_chart_line::MdxDemoChartLine,
        mdx_demo_chart_radar::MdxDemoChartRadar,
        mdx_demo_chart_candlesticks::MdxDemoChartCandlesticks,
        mdx_demo_chart_scatter::MdxDemoChartScatter,
        mdx_demo_chart_bar_horizontal::MdxDemoChartBarHorizontal,
        mdx_demo_chart_bar_json::MdxDemoChartBarJson,
        mdx_demo_chart_bar::MdxDemoChartBar,
    },
    utils::mdx::mdx_leptos::Components,
};

pub fn setup_mdx_visualizations() -> Components {
    let mut components = Components::new();

    components.add("MdxDemoChartTable".into(), MdxDemoChartTable);
    components.add("MdxDemoChartPie".into(), MdxDemoChartPie);
    components.add("MdxDemoChartLine".into(), MdxDemoChartLine);
    components.add("MdxDemoChartRadar".into(), MdxDemoChartRadar);
    components.add("MdxDemoChartCandlesticks".into(), MdxDemoChartCandlesticks);
    components.add("MdxDemoChartScatter".into(), MdxDemoChartScatter);
    components.add("MdxDemoChartBarHorizontal".into(), MdxDemoChartBarHorizontal);
    components.add("MdxDemoChartBarJson".into(), MdxDemoChartBarJson);
    components.add("MdxDemoChartBar".into(), MdxDemoChartBar);

    components
}
