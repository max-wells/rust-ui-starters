use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Card3DHover(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex relative justify-center items-end px-[36px] mx-[50px] card3dHover",
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
pub fn Card3DHoverImage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: String,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute w-full transition-all duration-500 z-[-1] card3dHover-wrapper",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            <img src=src class="object-cover w-full h-full" />
        </div>
    }
}

#[component]
pub fn Card3DHoverImageTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: String,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "w-full transition-transform duration-500 card3dHover-title",
            class()
        )
    });

    view! { <img {..attributes} class=class src=src /> }
}

#[component]
pub fn Card3DHoverImageOnHover(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] src: String,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute w-full opacity-0 transition-all duration-500  z-[-1] card3dHover-onHover",
            class()
        )
    });

    view! { <img {..attributes} class=class src=src /> }
}
