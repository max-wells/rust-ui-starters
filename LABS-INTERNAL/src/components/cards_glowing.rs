use leptos::*;
use tailwind_fuse::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement, MouseEvent};

const HARDCODED_CARDS_ID: &str = "cards";
const HARDCODED_CARD_CLASS: &str = "card";


/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


#[derive(Clone)]
struct MouseEffectContext;

fn provide_mouse_effect_context() {
    provide_context(MouseEffectContext);

    create_effect(move |_| {
        let document = window().unwrap().document().unwrap();

        if let Some(cards) = document.get_element_by_id(HARDCODED_CARDS_ID) {
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let cards = document.get_elements_by_class_name(HARDCODED_CARD_CLASS);

                for i in 0..cards.length() {
                    if let Some(card) = cards.item(i) {
                        let card: HtmlElement = card.dyn_into().unwrap();
                        let rect = card.get_bounding_client_rect();
                        let x = event.client_x() as f64 - rect.left();
                        let y = event.client_y() as f64 - rect.top();

                        card.style()
                            .set_property("--mouse-x", &format!("{}px", x))
                            .unwrap();
                        card.style()
                            .set_property("--mouse-y", &format!("{}px", y))
                            .unwrap();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            cards
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    });
}



#[component]
pub fn CardsGlowing(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    provide_mouse_effect_context();

    let class = create_memo(move |_| tw_merge!("flex flex-wrap gap-2 max-w-[916px]", class()));

    view! {
        <div {..attributes} class=class id=HARDCODED_CARDS_ID>
            {children()}
        </div>
    }
}

#[component]
pub fn CardGlowingItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            HARDCODED_CARD_CLASS, 
            "w-[calc(50%-4px)] md:w-[300px]",
            "flex relative flex-col rounded-md cursor-pointer w-[300px] h-[260px] bg-[rgba(255,255,255,0.1)]",
            "before:content-[''] before:absolute before:inset-0 before:opacity-0 before:transition-opacity before:duration-500 before:h-[100%] before:w-[100%] before:left-0 before:top-0 ",
            "after:content-[''] after:absolute after:inset-0 after:opacity-0 after:transition-opacity after:duration-500 after:h-[100%] after:w-[100%] after:left-0 after:top-0",
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
pub fn CardGlowingContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("flex flex-col p-2 absolute", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
