use leptos::*;
use std::rc::Rc;

use crate::{
    __registry__::sidenav_stuff::{
        sidenav_core_core::SIDENAV_CORE_CORE, sidenav_core_others::SIDENAV_CORE_OTHERS,
        sidenav_core_primitives::SIDENAV_CORE_PRIMITIVES,
        sidenav_core_visualizations::SIDENAV_CORE_VISUALIZATIONS,
    },
    components::layouts::layout_demos_routes_core::LayoutDemosRoutesCore,
};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RoutesDemoCore() -> impl IntoView {
    let demos_core_core = Rc::new(SIDENAV_CORE_CORE.to_vec());
    let demos_core_primitives = Rc::new(SIDENAV_CORE_PRIMITIVES.to_vec());
    let demos_core_visualizations = Rc::new(SIDENAV_CORE_VISUALIZATIONS.to_vec());
    let demos_core_others = Rc::new(SIDENAV_CORE_OTHERS.to_vec());

    view! {
        <LayoutDemosRoutesCore
            base_path="/demos-core"
            demos_core_core=demos_core_core
            demos_core_primitives=demos_core_primitives
            demos_core_visualizations=demos_core_visualizations
            demos_core_others=demos_core_others
        />
    }
}
