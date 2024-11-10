use leptos::*;
use tailwind_fuse::*;

// TODO 🐛 tw_merge!() working for all execpt what's inside (text-3xl, text-2xl, etc.)
// TODO. └──> I tried different way of doing this (cf. H1 and H2), but not working

// TODO 💪 Export Motion to _shared.rs (not working yet with #[tw(class = ...)] typically that do not support expanding constants directly)

#[derive(TwClass, Default)]
#[tw(class = "font-bold text-pretty")]
pub struct Heading {
    variant: HeadingVariant,
}

#[component]
pub fn H1(
    #[prop(into, optional)] variant: MaybeSignal<HeadingVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] style: MaybeSignal<String>,
    #[prop(into, optional)] key: MaybeSignal<i32>, // Used to force rerender
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let heading = Heading {
            variant,
        };
        tw_merge!(heading.with_class(class.get()), "text-4xl")
    });

    view! {
        <h1 {..attributes} class=class style=style key=key.get()>
            {children()}
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(into, optional)] variant: MaybeSignal<HeadingVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let heading = Heading {
            variant,
        };
        tw_merge!(heading.with_class(class.get()), "text-3xl")
    });

    view! {
        <h2 {..attributes} class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(into, optional)] variant: MaybeSignal<HeadingVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let heading = Heading {
            variant,
        };
        tw_merge!(heading.with_class(class.get()), "text-2xl")
    });

    view! {
        <h3 {..attributes} class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn H4(
    #[prop(into, optional)] variant: MaybeSignal<HeadingVariant>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let heading = Heading {
            variant,
        };
        tw_merge!(heading.with_class(class.get()), "text-xl")
    });

    view! {
        <h4 {..attributes} class=class>
            {children()}
        </h4>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🧬 STRUCT 🧬                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwVariant)]
pub enum HeadingVariant {
    #[tw(default, class = "text-pretty")]
    Default,
    #[tw(
        class = "tracking-tighter text-transparent bg-gradient-to-r from-white to-gray-500 bg-clip-text"
    )]
    Modern,
    #[tw(
        class = "underline underline-offset-3 decoration-8 decoration-neutral-400 dark:decoration-neutral-600"
    )]
    Underline,
}
