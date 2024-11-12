use leptos::*;
use leptos_router::*;
use std::rc::Rc;

use super::layouts::layout_demos_routes_core::DemoCoreTrait;

#[component]
pub fn SidenavDemoList<T: DemoCoreTrait + Clone + 'static>(
    demos: Rc<Vec<T>>,
    base_path: Rc<String>,
    params_demo_name: Rc<impl Fn() -> String + 'static>, // Wrap in Rc for cloning
) -> impl IntoView {
    view! {
        <ul class="ml-1 list-none">
            <For
                each=move || demos.clone().to_vec()
                key=|demo| demo.path_url()
                children={
                    let base_path = Rc::clone(&base_path);
                    let params_demo_name = Rc::clone(&params_demo_name);
                    move |demo| {
                        let demo_clone = demo.clone();
                        let base_path_ref = Rc::clone(&base_path);
                        let is_active = {
                            let params_demo_name = Rc::clone(&params_demo_name);
                            move || demo_clone.path_url() == params_demo_name()
                        };
                        view! {
                            <li class="">
                                <A
                                    href=format!("{}/{}", base_path_ref, demo.path_url())
                                    class=move || {
                                        let base_class = "text-muted-foreground hover:underline";
                                        if is_active() {
                                            format!("{} text-primary font-semibold", base_class)
                                        } else {
                                            base_class.to_string()
                                        }
                                    }
                                    exact=true
                                >
                                    {demo.name()}
                                </A>
                            </li>
                        }
                    }
                }
            />
        </ul>
    }
}
