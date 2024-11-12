use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CarouselParallax(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("w-full min-h-[150px]", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn Slides(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("flex overflow-scroll w-full whitespace-nowrap slides touch-pan-x snap-x snap-mandatory before:w-[36vw] before:shrink-0 after:w-[36vw] after:shrink-0 smooth-scroll", class())
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SlideItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("relative flex-shrink-0 mx-2 rounded-3xl slide h-[calc(70vw*1.5)] w-[70vw] snap-center overflow-clip sm:h-[calc(40vw*1.5)] sm:w-[40vw] md:h-[calc(25vw*1.5)] md:w-[25vw]", class())
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SlideImage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: &'static str,
    #[prop(into)] alt: &'static str,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("block object-cover object-center absolute right-0 w-full h-full animate-parallax [animation-timeline:view(x)]", class())
    });

    view! { <img {..attributes} class=class src=src alt=alt /> }
}
