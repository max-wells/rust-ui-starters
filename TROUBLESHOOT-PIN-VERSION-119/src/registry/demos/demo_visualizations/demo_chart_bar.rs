use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PersonId(pub u32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyPerson {
    pub id: PersonId,
    pub name: String,
    pub title: String,
}

fn generate_fake_persons_data() -> Vec<MyPerson> {
    vec![
        MyPerson {
            id: PersonId(1),
            name: String::from("Alice Smith"),
            title: String::from("Software Engineer"),
        },
        MyPerson {
            id: PersonId(2),
            name: String::from("Bob Johnson"),
            title: String::from("Software Engineer"),
        },
        MyPerson {
            id: PersonId(3),
            name: String::from("Charlie Brown"),
            title: String::from("Data Scientist"),
        },
        MyPerson {
            id: PersonId(4),
            name: String::from("Diana Prince"),
            title: String::from("UX Designer"),
        },
    ]
}

#[component]
pub fn DemoChartBar() -> impl IntoView {
    let persons_data = generate_fake_persons_data();

    let mut title_count_map = std::collections::HashMap::new();

    for person in persons_data {
        *title_count_map.entry(person.title).or_insert(0.0) += 1.0;
    }

    let data_vec: Vec<String> = title_count_map.keys().cloned().collect();
    let count_vec: Vec<f32> = title_count_map.values().cloned().collect();

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.font_family = String::from("Noto Sans SC");
    bar_chart.background_color = Color::transparent();
    bar_chart.width = 550.0;
    bar_chart.height = 400.0;
    bar_chart.y_axis_hidden = true;

    view! { <div class="p-10 mx-auto h-[500px]" inner_html=&bar_chart.svg().unwrap() /> }
        .into_view()
}
