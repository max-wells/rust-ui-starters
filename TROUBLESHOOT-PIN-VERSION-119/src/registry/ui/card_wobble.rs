use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use tailwind_fuse::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement};

#[derive(Clone)]
struct CardWobbleContext;

pub fn provide_card_wobble_context() {
    provide_context(CardWobbleContext);

    create_effect(move |_| {
        let document = window().unwrap().document().unwrap();

        if let Some(slide) = document.get_element_by_id("example") {
            let slide_clone = Rc::new(RefCell::new(slide.unchecked_into::<HtmlElement>()));

            let slide_clone_for_mousemove = Rc::clone(&slide_clone);
            let closure_mousemove = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let slide = slide_clone_for_mousemove.borrow();
                let rect = slide.get_bounding_client_rect();
                let x = e.client_x() as f64 - (rect.left() + rect.width() / 2.0);
                let y = e.client_y() as f64 - (rect.top() + rect.height() / 2.0);
                slide
                    .style()
                    .set_property("--x", &format!("{}px", x))
                    .unwrap();
                slide
                    .style()
                    .set_property("--y", &format!("{}px", y))
                    .unwrap();

                // TODO. Debug
                web_sys::console::log_2(&"Mousemove: --x:".into(), &x.into());
                web_sys::console::log_2(&"Mousemove: --y:".into(), &y.into());
            }) as Box<dyn FnMut(_)>);

            slide_clone
                .borrow()
                .add_event_listener_with_callback(
                    "mousemove",
                    closure_mousemove.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure_mousemove.forget();

            let slide_clone_for_mouseleave = Rc::clone(&slide_clone);
            let closure_mouseleave = Closure::wrap(Box::new(move || {
                let slide = slide_clone_for_mouseleave.borrow();
                slide.style().set_property("--x", "0px").unwrap();
                slide.style().set_property("--y", "0px").unwrap();
                web_sys::console::log_1(&"Mouseleave: --x and --y reset to 0px".into());
            }) as Box<dyn FnMut()>);

            slide_clone
                .borrow()
                .add_event_listener_with_callback(
                    "mouseleave",
                    closure_mouseleave.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure_mouseleave.forget();
        } else {
            web_sys::console::error_1(&"Element with ID 'example' not found.".into());
        }
    });
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CardWobble(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "relative flex flex-col items-center justify-center text-center text-white slide",
            class()
        )
    });

    view! {
        <div {..attributes} class=class id="example">
            {children()}
        </div>
    }
}

#[component]
pub fn CardWobbleImageWrapper(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "slide__image-wrapper",
            "overflow-hidden absolute h-full left-[0%] top-[0%] rounded-[1%]",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardWobbleImage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: String,
    #[prop(into)] alt: MaybeSignal<String>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("slide__image", "object-cover absolute pointer-events-none select-none w-[110%] h-[110%] left-[-5%] top-[-5%]", class())
    });

    view! { <img {..attributes} class=class src=src alt=alt /> }
}

#[component]
pub fn CardWobbleContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("slide__content", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
