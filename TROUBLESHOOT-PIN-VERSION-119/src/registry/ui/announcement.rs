use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Announcement(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex relative flex-row justify-center items-center py-1.5 px-4 mx-auto text-sm font-medium rounded-2xl transition-shadow duration-500 ease-out group max-w-fit bg-white/40 shadow-[inset_0_-8px_10px_#8fdfff1f] backdrop-blur-sm [--bg-size:300%] dark:bg-black/40 hover:shadow-[inset_0_-5px_10px_#8fdfff3f]",
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
pub fn AnnouncementLineEffect(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "block absolute inset-0 w-full h-full bg-gradient-to-r animate-gradient from-[#ffaa40]/50 via-[#9c40ff]/50 to-[#ffaa40]/50 bg-[length:var(--bg-size)_100%] p-[1px] [border-radius:inherit] ![mask-composite:subtract] [mask:linear-gradient(#fff_0_0)_content-box,linear-gradient(#fff_0_0)]",
            class()
        )
    });

    view! { <div {..attributes} class=class /> }
}

#[component]
pub fn AnnouncementDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "inline text-transparent bg-clip-text bg-gradient-to-r from-[#ffaa40] via-[#9c40ff] to-[#ffaa40] animate-gradient bg-[length:var(--bg-size)_100%]",
            class()
        )
    });

    view! {
        <span {..attributes} class=class>
            {children()}
        </span>
    }
}
