use crate::{
    __registry__::sidenav_stuff::{
        sidenav_hooks_browser::SIDENAV_HOOKS_BROWSER,
        sidenav_hooks_elements::SIDENAV_HOOKS_ELEMENTS, sidenav_hooks_maths::SIDENAV_HOOKS_MATHS,
        sidenav_hooks_others::SIDENAV_HOOKS_OTHERS,
        sidenav_hooks_reactivity::SIDENAV_HOOKS_REACTIVITY,
        sidenav_hooks_sensors::SIDENAV_HOOKS_SENSORS,
    },
    components::layouts::layout_demos_routes_hooks::LayoutDemosRoutesHooks,
};
use leptos::*;
use std::rc::Rc;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RoutesDemoHooks() -> impl IntoView {
    let demos_hooks_others = Rc::new(SIDENAV_HOOKS_OTHERS.to_vec());
    let demos_hooks_sensors = Rc::new(SIDENAV_HOOKS_SENSORS.to_vec());
    let demos_hooks_elements = Rc::new(SIDENAV_HOOKS_ELEMENTS.to_vec());
    let demos_hooks_browser = Rc::new(SIDENAV_HOOKS_BROWSER.to_vec());
    let demos_hooks_maths = Rc::new(SIDENAV_HOOKS_MATHS.to_vec());
    let demos_hooks_reactivity = Rc::new(SIDENAV_HOOKS_REACTIVITY.to_vec());

    view! {
        <LayoutDemosRoutesHooks
            base_path="/demos-hooks"
            demos_hooks_others=demos_hooks_others
            demos_hooks_sensors=demos_hooks_sensors
            demos_hooks_elements=demos_hooks_elements
            demos_hooks_browser=demos_hooks_browser
            demos_hooks_maths=demos_hooks_maths
            demos_hooks_reactivity=demos_hooks_reactivity
        />
    }
}
