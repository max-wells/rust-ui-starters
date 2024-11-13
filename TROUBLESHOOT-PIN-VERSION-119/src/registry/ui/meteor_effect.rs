use leptos::*;
use rand::Rng;
use tailwind_fuse::*;

#[component]
pub fn MeteorEffect(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("relative w-full max-w-[320px]", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn Meteors(#[prop(optional)] number: Option<usize>) -> impl IntoView {
    const DEFAULT_NUMBER_METEORS: usize = 20;

    let number_of_meteors = number.unwrap_or(DEFAULT_NUMBER_METEORS);
    let mut rng = rand::thread_rng();

    let meteors = (0..number_of_meteors).map(|_| {
        let left = rng.gen_range(-400..=400);
        let animation_delay = rng.gen_range(0.2..=0.8);
        let animation_duration = rng.gen_range(2..=10);

        view! {
            <Meteor style=format!(
                "top:0;left:{}px;animation-delay:{}s;animation-duration:{}s",
                left,
                animation_delay,
                animation_duration,
            ) />
        }
    });

    view! { <>{meteors.collect::<Vec<_>>()}</> }
}

#[component]
pub fn Meteor(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] style: MaybeSignal<String>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "absolute top-1/2 left-1/2 w-0.5 h-0.5 animate-meteor-effect rotate-[215deg] rounded-[9999px] bg-slate-500 shadow-[0_0_0_1px_#ffffff10] before:absolute before:top-1/2 before:h-[1px] before:w-[50px] before:-translate-y-[50%] before:transform before:bg-gradient-to-r before:from-[#64748b] before:to-transparent before:content-[&#x27;&#x27;]",
            class()
        )
    });

    view! { <span {..attributes} class=class style=style /> }
}

#[component]
pub fn MeteorCard(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex overflow-hidden relative flex-col p-6 bg-gray-900 rounded-2xl border",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
