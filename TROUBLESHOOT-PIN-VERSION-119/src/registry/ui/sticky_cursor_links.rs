use leptos::*;
use tailwind_fuse::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Clone)]
struct StickyCursorLinksContext;

fn provide_sticky_cursor_links_context() {
    provide_context(StickyCursorLinksContext);

    create_effect(|_| {
        let document = window().unwrap().document().unwrap();
        let cursor = document.query_selector(".cursor").unwrap().unwrap();
        let cursor_clone = cursor.clone();

        let links = document.query_selector_all("nav > .link").unwrap();

        for i in 0..links.length() {
            let link = links.get(i).unwrap();
            let link_clone = link.clone();

            let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let element = link_clone.dyn_ref::<web_sys::Element>().unwrap();
                let span = element.query_selector(".link > span").unwrap().unwrap();
                let html_element = span.dyn_ref::<HtmlElement>().unwrap();
                let x = e.offset_x() as f64;
                let y = e.offset_y() as f64;
                let width = element.client_width() as f64;
                let height = element.client_height() as f64;
                let move_amount = 25.0;
                let x_move = (x / width) * (move_amount * 2.0) - move_amount;
                let y_move = (y / height) * (move_amount * 2.0) - move_amount;

                html_element
                    .style()
                    .set_property(
                        "transform",
                        &format!("translate({}px, {}px)", x_move, y_move),
                    )
                    .unwrap();

                if e.type_() == "mouseleave" {
                    html_element.style().set_property("transform", "").unwrap();
                }
            }) as Box<dyn FnMut(_)>);

            link.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .unwrap();
            link.add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let x = e.client_x();
            let y = e.client_y();
            let cursor_html_element = cursor_clone.dyn_ref::<HtmlElement>().unwrap();
            cursor_html_element
                .style()
                .set_property("left", &format!("{}px", x))
                .unwrap();
            cursor_html_element
                .style()
                .set_property("top", &format!("{}px", y))
                .unwrap();
        }) as Box<dyn FnMut(_)>);

        window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn StickyLinksNav(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    provide_sticky_cursor_links_context();

    let class = create_memo(move |_| {
        tw_merge!(
            "flex flex-col justify-around  mx-auto w-full text-center  lg:flex-row group cursor-none",
            class()
        )
    });

    view! {
        <nav {..attributes} class=class>
            {children()}

            <StickyCursor />
        </nav>
    }
}

#[component]
pub fn StickyLink(text: &'static str) -> impl IntoView {
    view! {
        <a href="#" class="link">
            <span class="inline-block my-2 text-3xl font-bold text-white uppercase">{text}</span>
        </a>
    }
}

#[component]
fn StickyCursor() -> impl IntoView {
    view! {
        <div class="hidden fixed bg-white rounded-full transition-transform duration-300 ease-linear pointer-events-none group-hover:block mix-blend-difference p-[0.3rem] cursor" />
    }
}
