use leptos::*;
use leptos_meta::Stylesheet;
use tailwind_fuse::*;

use crate::registry::icons::others::check::Check;

#[component]
pub fn DemoCardsFollowShadow() -> impl IntoView {
    view! {
        <Stylesheet id="cards-follow-shadow" href="/components/cards-follow-shadow.css" />
        <script src="/components/cards-follow-shadow.js" />

        <CardShadowGrid>
            <CardShadowItem />
            <CardShadowItem />
            <CardShadowItem />
            <CardShadowItem />
        </CardShadowGrid>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CardShadowGrid(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "relative grid grid-cols-2 p-0 m-0 list-none", 
        "after:content-[''] after:absolute after:rounded-lg after:bg-[hsl(0,0%,10%)] after:pointer-events-none after:-z-10 after:transition-[inset] after:duration-200 after:-z-10", 
        class())
    });

    view! {
        <ul {..attributes} class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn CardShadowItem() -> impl IntoView {
    view! {
        <li class="p-4 list-none">
            <ArticleCard>
                <Check />
                <h3>"Title"</h3>
                <p class="m-0 text-transparent bg-clip-text bg-gradient-to-b from-[hsl(0,0%,80%)] to-[hsl(0,0%,50%)] font-[80]">
                    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minima alias fuga et ab magnam aliquam commodi ratione vel fugit nesciunt voluptatibus."
                </p>
            </ArticleCard>
        </li>
    }
}

#[component]
pub fn ArticleCard(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "grid relative gap-1 p-4",
            "before:content-[''] before:absolute before:inset-0",
            "before:bg-gradient-to-r before:from-[var(--bg)] before:to-transparent",
            "before:bg-[length:40px_40px] before:bg-[position:-20px_-20px]",
            "before:mask-[linear-gradient(-35deg,var(--bg)_0%,transparent_45%)]",
            "before:z-[-1] before:opacity-[var(--li-active,0)] before:transition-opacity",
            // AFTER
            "after:content-[''] after:absolute after:inset-0",
            "after:z-[-2] after:bg-[hsl(0,0%,10%)]",
            "after:rounded-[1rem]",
            "after:opacity-[var(--li-active,0)] after:transition-opacity",
            class()
        )
    });

    view! {
        <article {..attributes} class=class>
            {children()}
        </article>
    }
}
