use leptos::html::Div;
use leptos::*;
use leptos_router::{use_location, A as Link};
use leptos_use::on_click_outside;

use crate::__registry__::sidenav_stuff::sidenav_core_core::SIDENAV_CORE_CORE;
use crate::__registry__::sidenav_stuff::sidenav_core_others::SIDENAV_CORE_OTHERS;
use crate::__registry__::sidenav_stuff::sidenav_core_primitives::SIDENAV_CORE_PRIMITIVES;
use crate::__registry__::sidenav_stuff::sidenav_core_visualizations::SIDENAV_CORE_VISUALIZATIONS;
use crate::components::layouts::logo_home_link::LogoHomeLink;
use crate::registry::hooks::use_lock_body_scroll::use_lock_body_scroll;
use crate::registry::icons::others::menu::Menu;
use crate::registry::ui::headings::H2;
use crate::registry::ui::sheet::{SheetContent, SheetDirection, SheetTrigger, SheetVariant};

#[component]
pub fn NavMobile() -> impl IntoView {
    let location = use_location();
    // println!("location: {:?}", location.pathname.get());
    let is_active = move |path: &str| -> bool { location.pathname.get().starts_with(path) };

    let (is_open, set_is_open) = create_signal(false);
    let (_scroll_locked, set_scroll_locked) = use_lock_body_scroll(false);
    let _sheet_ref = create_node_ref::<Div>();

    let toggle_sheet = move |_| {
        let new_state = !is_open.get();
        set_is_open.set(new_state);
        set_scroll_locked.set(new_state);
    };

    create_effect(move |_| {
        if is_open.get() {
            let _ = on_click_outside(_sheet_ref, move |_| {
                set_is_open.set(false);
                set_scroll_locked.set(false);
            });
        }
    });

    const HIDDEN_FOR_DESKTOP: &str = "md:hidden";

    view! {
        <>
            <SheetTrigger
                on:click=toggle_sheet
                variant=SheetVariant::Ghost
                class=HIDDEN_FOR_DESKTOP
            >
                <Menu class="size-6" />
            </SheetTrigger>

            <div node_ref=_sheet_ref class=HIDDEN_FOR_DESKTOP>
                <SheetContent
                    direction=SheetDirection::Left
                    is_open=is_open
                    on:click=toggle_sheet
                    class="pb-4 w-[350px]"
                >
                    <div class="flex flex-col gap-4">
                        <LogoHomeLink />
                        <GetStartedLinks />
                        <AllDemoComponents />
                        <AllDemoPrimitives />
                        <AllDemoOthers />
                    </div>
                </SheetContent>
            </div>
        </>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//

#[component]
pub fn GetStartedLinks() -> impl IntoView {
    let docs_links = [
        ("/docs/introduction", "Introduction"),
        ("/docs/installation", "Installation"),
        ("/docs/cli", "CLI"),
    ];

    view! {
        <div class="flex flex-col gap-1">
            <H2>Get Started</H2>

            {docs_links
                .iter()
                .map(|(href, name)| {
                    let href_owned = href.to_owned();
                    let name_owned = name.to_owned();
                    view! { <Link href=href_owned>{name_owned}</Link> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn AllDemoComponents() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <H2>Components</H2>

            {SIDENAV_CORE_CORE
                .iter()
                .map(|demo| {
                    let href_owned = format!("demos-core/{}", demo.path_url);
                    let name_owned = demo.name.to_owned();
                    view! { <Link href=href_owned>{name_owned}</Link> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn AllDemoPrimitives() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <H2>Primitives</H2>

            {SIDENAV_CORE_PRIMITIVES
                .iter()
                .map(|demo| {
                    let href_owned = format!("demos-core/{}", demo.path_url);
                    let name_owned = demo.name.to_owned();
                    view! { <Link href=href_owned>{name_owned}</Link> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn AllDemoOthers() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <H2>Others</H2>

            {SIDENAV_CORE_OTHERS
                .iter()
                .map(|demo| {
                    let href_owned = format!("demos-core/{}", demo.path_url);
                    let name_owned = demo.name.to_owned();
                    view! { <Link href=href_owned>{name_owned}</Link> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn AllDemoVizualisations() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <H2>Vizualisations</H2>

            {SIDENAV_CORE_VISUALIZATIONS
                .iter()
                .map(|demo| {
                    let href_owned = format!("demos-core/{}", demo.path_url);
                    let name_owned = demo.name.to_owned();
                    view! { <Link href=href_owned>{name_owned}</Link> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
