use leptos::*;
use leptos_meta::Stylesheet;
use tailwind_fuse::*;

use crate::components::icons::check::Check;

#[component]
pub fn DemoCardsRepushing() -> impl IntoView {
    view! {
        <Stylesheet id="cards-repushing" href="/components/cards-repushing.css" />

        <CardsRepushing class="container mainContainer hover:[&>.glass]:rotate-0 hover:[&>.glass]:m-[0_10px]">
            <CardsRepushing class="container mainContainer">
                <CardsRepushingItem data_text="Github" rotation=-15 />
                <CardsRepushingItem data_text="Code" rotation=5 />
                <CardsRepushingItem data_text="Earn" rotation=25 />
            </CardsRepushing>
        </CardsRepushing>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CardsRepushing(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| tw_merge!("flex relative justify-center items-center", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardsRepushingItem(
    #[prop(into)] data_text: String,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into)] rotation: i32,
    #[prop(into, optional)] check_class: Option<String>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "glass",
            "bg-gradient-to-b from-[#fff2] to-transparent shadow-[0_25px_25px_rgba(0,0,0,0.25)] transition duration-500 m-[0_-45px] [backdrop-filter:blur(10px)] [transform:rotate(calc(var(--r)*1deg))]",
            "flex relative justify-center items-center rounded-md border border-gray-200 w-[180px] h-[200px]",
            "before:content-[''] before:absolute before:h-[40px] before:bottom-0 before:w-full before:flex before:justify-center before:items-center before:content-attr(data-text) before:text-white",
            class()
        )
    });

    let check_class = check_class.unwrap_or_else(|| "text-4xl size-10".to_string());

    view! {
        <div data-text=data_text style=format!("--r:{};", rotation) class=class>
            <Check class=check_class />
        </div>
    }
}
