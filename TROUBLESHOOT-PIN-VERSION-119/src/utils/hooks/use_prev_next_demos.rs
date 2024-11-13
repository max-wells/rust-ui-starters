use crate::components::layouts::layout_demos_routes_core::DemoCoreTrait;
use std::rc::Rc;

#[allow(non_snake_case)]
pub fn usePrevNextDemos<T: DemoCoreTrait + Clone + std::fmt::Debug>(
    all_demos: Rc<Vec<T>>,
    current_demo_name: String,
    base_path: Rc<String>,
) -> (String, String, String, String) {
    let vec_demo_names: Vec<String> = all_demos
        .iter()
        .map(|demo| demo.name().to_string())
        .collect();

    let current_index = vec_demo_names
        .iter()
        .position(|name| name == &current_demo_name);

    let (prev_demo_name, next_demo_name) = if let Some(index) = current_index {
        let prev_index = if index == 0 {
            vec_demo_names.len() - 1
        } else {
            index - 1
        };
        let next_index = if index == vec_demo_names.len() - 1 {
            0
        } else {
            index + 1
        };
        (
            vec_demo_names[prev_index].clone(),
            vec_demo_names[next_index].clone(),
        )
    } else {
        ("".to_string(), "".to_string())
    };

    let prev_demo_sluggified = prev_demo_name.replace(" ", "-").to_lowercase();
    let next_demo_sluggified = next_demo_name.replace(" ", "-").to_lowercase();

    let prev_demo_href = format!("{}/{}", base_path, prev_demo_sluggified);
    let next_demo_href = format!("{}/{}", base_path, next_demo_sluggified);

    (
        prev_demo_name,
        next_demo_name,
        prev_demo_href,
        next_demo_href,
    )
}
