use leptos::*;
use leptos_router::*;
use std::rc::Rc;

use crate::components::mdx::mdx_header_demo::MdxHeaderDemo;
use crate::components::mdx::mdx_shared_demo::SharedDemoMdx;
use crate::components::sticky_footer::StickyFooter;
use crate::registry::ui::headings::H4;

pub trait DemoHooksTrait: PartialEq {
    fn name(&self) -> &'static str;
    fn path_url(&self) -> &'static str;
    fn path_mdx(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn tags(&self) -> &'static [&'static str];
}

// TODO 💪 Create the navigation Links between PREVIOUS and NEXT demos.

const PARAMS_KEY: &str = "name";

#[component]
pub fn LayoutDemosRoutesHooks<T: DemoHooksTrait + Clone + 'static>(
    #[prop(into)] base_path: String,
    demos_hooks_others: Rc<Vec<T>>,
    demos_hooks_sensors: Rc<Vec<T>>,
    demos_hooks_elements: Rc<Vec<T>>,
    demos_hooks_browser: Rc<Vec<T>>,
    demos_hooks_maths: Rc<Vec<T>>,
    demos_hooks_reactivity: Rc<Vec<T>>,
) -> impl IntoView {
    let base_path = Rc::new(base_path); // Wrap base_path in Rc
    let params = use_params_map();
    let params_demo_name = move || params().get(PARAMS_KEY).cloned().unwrap_or_default();

    let memo_hooks_others = create_memo(move |_| demos_hooks_others.clone());
    let memo_hooks_sensors = create_memo(move |_| demos_hooks_sensors.clone());
    let memo_hooks_elements = create_memo(move |_| demos_hooks_elements.clone());
    let memo_hooks_browser = create_memo(move |_| demos_hooks_browser.clone());
    let memo_hooks_maths = create_memo(move |_| demos_hooks_maths.clone());
    let memo_hooks_reactivity = create_memo(move |_| demos_hooks_reactivity.clone());

    const HOOK_TITLES: &[&str] = &[
        "Hooks Elements",
        "Hooks Sensors",
        "Hooks Browser",
        "Hooks Maths",
        "Hooks Reactivity",
        "Hooks Others",
    ];

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
                                {HOOK_TITLES
                                    .iter()
                                    .zip([
                                        memo_hooks_elements.get(),
                                        memo_hooks_sensors.get(),
                                        memo_hooks_browser.get(),
                                        memo_hooks_maths.get(),
                                        memo_hooks_reactivity.get(),
                                        memo_hooks_others.get(),
                                    ])
                                    .map(|(title, demos)| {
                                        view! {
                                            <div>
                                                // Dereference title
                                                <H4 class="my-1 text-2xl">{*title}</H4>
                                                <DemoList
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

                            <div>
                                {move || {
                                    let demos_others = memo_hooks_others.get();
                                    let demos_sensors = memo_hooks_sensors.get();
                                    let demos_elements = memo_hooks_elements.get();
                                    let demos_browser = memo_hooks_browser.get();
                                    let demos_maths = memo_hooks_maths.get();
                                    let demos_reactivity = memo_hooks_reactivity.get();
                                    let all_demos: Vec<_> = demos_others
                                        .iter()
                                        .chain(demos_sensors.iter())
                                        .chain(demos_elements.iter())
                                        .chain(demos_browser.iter())
                                        .chain(demos_maths.iter())
                                        .chain(demos_reactivity.iter())
                                        .collect();
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
                            </div>

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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn DemoList<T: DemoHooksTrait + Clone + 'static>(
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
