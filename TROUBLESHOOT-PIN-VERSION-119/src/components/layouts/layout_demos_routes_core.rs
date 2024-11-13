use leptos::*;
use leptos_router::*;
use std::rc::Rc;

use crate::components::mdx::mdx_header_demo::MdxHeaderDemo;
use crate::components::mdx::mdx_prev_next_demos::MdxPrevNextDemos;
use crate::components::mdx::mdx_shared_demo::SharedDemoMdx;
use crate::components::sidenav_demo_list::SidenavDemoList;
use crate::components::sticky_footer::StickyFooter;
use crate::registry::ui::headings::H4;

pub trait DemoCoreTrait: PartialEq {
    fn name(&self) -> &'static str;
    fn path_url(&self) -> &'static str;
    fn path_mdx(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn tags(&self) -> &'static [&'static str];
}

// TODO 💪 Create the navigation Links between PREVIOUS and NEXT demos.

const PARAMS_KEY: &str = "name";

#[component]
pub fn LayoutDemosRoutesCore<T: DemoCoreTrait + Clone + std::fmt::Debug + 'static>(
    #[prop(into)] base_path: String,
    demos_core_core: Rc<Vec<T>>,
    demos_core_primitives: Rc<Vec<T>>,
    demos_core_visualizations: Rc<Vec<T>>,
    demos_core_others: Rc<Vec<T>>,
) -> impl IntoView {
    let base_path = Rc::new(base_path); // Wrap base_path in Rc
    let params = use_params_map();
    let params_demo_name = move || params().get(PARAMS_KEY).cloned().unwrap_or_default();

    let memo_core_core = create_memo(move |_| demos_core_core.clone());
    let memo_core_primitives = create_memo(move |_| demos_core_primitives.clone());
    let memo_core_vizualisations = create_memo(move |_| demos_core_visualizations.clone());
    let memo_core_others = create_memo(move |_| demos_core_others.clone());

    const CORE_TITLES: &[&str] = &["Core", "Primitives", "Vizualisations", "Others"];

    let demos_core = memo_core_core.get().clone();
    let demos_primitives = memo_core_primitives.get().clone();
    let demos_vizualisations = memo_core_vizualisations.get().clone();
    let demos_others = memo_core_others.get().clone();
    let all_demos: Vec<_> = demos_core
        .iter()
        .chain(demos_primitives.iter())
        .chain(demos_vizualisations.iter())
        .chain(demos_others.iter())
        .cloned() // Add this line to clone the items
        .collect();

    view! {
        <div class="container px-0">
            <div class="flex items-start md:grid md:ml-2 md:grid-cols-[220px_minmax(0,1fr)] lg:grid-cols-[240px_minmax(0,1fr)]">
                <aside class="hidden fixed top-14 z-30 w-full md:block md:sticky md:ml-2 h-[calc(100vh-3.5rem)] shrink-0 shortfix-sidenav-todo-properly">
                    // TODO 🚑 Shortfix for scrollbar of the SIDENAV 👆

                    <div
                        class="overflow-hidden relative pt-2 pb-6 h-full w-[205px]"
                        style="position:relative;--radix-scroll-area-corner-width:0px;--radix-scroll-area-corner-height:0px"
                    >

                        <div class="overflow-hidden overflow-y-scroll w-full h-full rounded-[inherit]">

                            <div class="flex flex-col gap-4 w-full">
                                {CORE_TITLES
                                    .iter()
                                    .zip([
                                        memo_core_core.get(),
                                        memo_core_primitives.get(),
                                        memo_core_vizualisations.get(),
                                        memo_core_others.get(),
                                    ])
                                    .map(|(title, demos)| {
                                        view! {
                                            <div>
                                                // Dereference title
                                                <H4 class="my-1 text-2xl">{*title}</H4>
                                                <SidenavDemoList
                                                    demos=demos
                                                    base_path=Rc::clone(&base_path)
                                                    params_demo_name=Rc::new(params_demo_name)
                                                />
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>

                        </div>

                    </div>

                </aside>

                //
                // ------- RIGHT SIDE -------
                //

                <div class="w-full lg:max-w-[900px]">

                    <div class="px-4" style="box-shadow:0 8px 10px -6px rgba(0, 0, 0, 0.1)">

                        <section class="flex flex-col pt-4 w-full min-h-[840px]">
                            {move || {
                                let current_demo = all_demos
                                    .iter()
                                    .find(|demo| demo.path_url() == params_demo_name());
                                match current_demo {
                                    Some(demo) => {
                                        view! {
                                            <div>
                                                <MdxHeaderDemo
                                                    name=demo.name()
                                                    description=demo.description()
                                                    tags=demo.tags()
                                                />

                                                <SharedDemoMdx mdx_path=demo.path_mdx() />

                                                <MdxPrevNextDemos
                                                    all_demos=Rc::new(all_demos.clone())
                                                    current_demo=demo.clone()
                                                    base_path=Rc::clone(&base_path)
                                                />
                                            </div>
                                        }
                                            .into_view()
                                    }
                                    None => {
                                        view! { <h2>"Select a demo from the sidebar"</h2> }
                                            .into_view()
                                    }
                                }
                            }}

                        </section>

                    </div>

                    //
                    //
                    //

                    <StickyFooter />

                </div>

            </div>

        </div>
    }
}
