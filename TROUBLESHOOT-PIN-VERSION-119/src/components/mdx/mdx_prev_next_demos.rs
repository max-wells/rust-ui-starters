use leptos::*;
use leptos_router::*;
use std::rc::Rc;

use crate::{
    components::layouts::layout_demos_routes_core::DemoCoreTrait,
    registry::{
        icons::chevrons::{chevron_left::ChevronLeft, chevron_right::ChevronRight},
        ui::button::{Button, ButtonVariant},
    },
    utils::hooks::use_prev_next_demos::usePrevNextDemos,
};

#[component]
pub fn MdxPrevNextDemos<T: DemoCoreTrait + Clone + std::fmt::Debug + 'static>(
    all_demos: Rc<Vec<T>>,
    current_demo: T,
    base_path: Rc<String>,
) -> impl IntoView {
    let current_demo_name = current_demo.name().to_string();

    // 🪝
    let (prev_demo_name, next_demo_name, prev_demo_href, next_demo_href) =
        usePrevNextDemos(all_demos, current_demo_name.clone(), base_path.clone());

    view! {
        <div class="flex justify-between items-center my-8">

            // TODO. Button should be as_child
            <Button variant=ButtonVariant::Outline>
                <A href=prev_demo_href class="flex items-center">
                    <ChevronLeft class="mr-2 size-4" />
                    {&prev_demo_name}
                </A>
            </Button>

            <Button variant=ButtonVariant::Outline>
                <A href=next_demo_href class="flex items-center">
                    {&next_demo_name}
                    <ChevronRight class="ml-2 size-4" />
                </A>
            </Button>
        </div>
    }
}
