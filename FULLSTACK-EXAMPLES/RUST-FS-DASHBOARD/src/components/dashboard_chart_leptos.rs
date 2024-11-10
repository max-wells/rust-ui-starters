use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::*;
use leptos_query::*;

use crate::{
    models::model_persons::{AllPersonsTag, MyPerson},
    utils::hooks::queries::queries_persons::useAllPersons,
};

#[component]
pub fn DashboardChartLeptos() -> impl IntoView {
    let QueryResult {
        data,
        state,
        ..
    } = useAllPersons().use_query(|| AllPersonsTag);

    let all_persons: Signal<Vec<MyPerson>> = Signal::derive(move || data.get().unwrap_or_default());

    create_effect(move |_| {
        let state = state.get();
        let log = match state {
            QueryState::Created => "created",
            QueryState::Loading => "loading",
            QueryState::Fetching(_) => "fetching",
            QueryState::Loaded(_) => "loaded",
            QueryState::Invalid(_) => "invalid",
        };
        logging::log!("STATE: {log}")
    });

    // Prepare data for the chart
    let chart_data = Signal::derive(move || {
        let persons = all_persons.get();
        let mut data_vec = Vec::new();
        let mut count_vec: Vec<f32> = Vec::new();

        for person in &persons {
            if !data_vec.contains(&person.title) {
                data_vec.push(person.title.clone());
                count_vec.push(1.0);
            } else {
                let index = data_vec
                    .iter()
                    .position(|title| title == &person.title)
                    .unwrap();
                count_vec[index] += 1.0;
            }
        }

        (data_vec, count_vec)
    });

    // Create the chart
    let bar_chart = Signal::derive(move || {
        let (data_vec, count_vec) = chart_data.get();
        let mut bar_series = Series::new(String::new(), count_vec);
        bar_series.label_show = true;

        let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
        bar_chart.font_family = String::from("Noto Sans SC");
        bar_chart.background_color = Color::transparent();
        bar_chart.width = 832.0;
        bar_chart.height = 500.0;
        bar_chart.y_axis_hidden = true;

        bar_chart
    });

    view! {
        <div class="flex flex-col gap-4 p-4 rounded-md bg-muted">

            <h2 class="text-lg font-bold">"All Server Persons"</h2>

            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>

                <div
                    class="py-10 px-4 pb-10 w-full h-20 rounded border border-orange-500 max-w-[700px] bg-black-400"
                    inner_html=&bar_chart.get().svg().unwrap()
                />

            </Transition>

        </div>
    }
}
