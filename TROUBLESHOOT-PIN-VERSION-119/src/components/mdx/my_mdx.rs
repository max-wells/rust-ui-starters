use crate::{
    components::mdx::{
        setup_mdx_core::setup_mdx_core, setup_mdx_extensions::setup_mdx_extensions,
        setup_mdx_hooks::setup_mdx_hooks, setup_mdx_primitives::setup_mdx_primitives,
        setup_mdx_shared::setup_mdx_shared, setup_mdx_visualizations::setup_mdx_visualizations,
    },
    utils::mdx::mdx_leptos::{Components, Mdx},
};
use leptos::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      🦀 ENTRYPOINT 🦀                      */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. Precise `core_primitives`, `core_vizualisations`, etc.

#[component]
pub fn MyMdx(
    source: String,
    #[prop(default = Components::new())] components: Components,
) -> impl IntoView {
    let mut combined_components = setup_mdx_shared();

    combined_components.extend(setup_mdx_core());
    combined_components.extend(setup_mdx_primitives());
    combined_components.extend(setup_mdx_visualizations());
    combined_components.extend(setup_mdx_hooks());
    combined_components.extend(setup_mdx_extensions());

    combined_components.extend(components);

    // combined_components.print_info();

    view! { <Mdx source=source components=combined_components /> }
}
