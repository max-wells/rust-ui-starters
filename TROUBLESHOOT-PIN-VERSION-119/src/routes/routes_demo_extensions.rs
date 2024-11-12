use crate::{
    __registry__::sidenav_stuff::{
        sidenav_extensions_css::SIDENAV_EXTENSIONS_CSS,
        sidenav_extensions_javascript::SIDENAV_EXTENSIONS_JAVASCRIPT,
        sidenav_extensions_others::SIDENAV_EXTENSIONS_OTHERS,
    },
    components::layouts::layout_demos_routes_extensions::LayoutDemosRoutesExtensions,
};
use leptos::*;
use std::rc::Rc;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RoutesDemoExtensions() -> impl IntoView {
    let demos_extensions_css = Rc::new(SIDENAV_EXTENSIONS_CSS.to_vec());
    let demos_extensions_javascript = Rc::new(SIDENAV_EXTENSIONS_JAVASCRIPT.to_vec());
    let demos_extensions_others = Rc::new(SIDENAV_EXTENSIONS_OTHERS.to_vec());

    view! {
        <LayoutDemosRoutesExtensions
            base_path="/demos-extensions"
            demos_extensions_css=demos_extensions_css
            demos_extensions_javascript=demos_extensions_javascript
            demos_extensions_others=demos_extensions_others
        />
    }
}
