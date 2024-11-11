use leptos::*;
use leptos_meta::Stylesheet;

// TODO. Works better with the HTML version

#[component]
pub fn DemoCardsStacking() -> impl IntoView {
    view! {
        <Stylesheet id="cards-stacking" href="/components-nooo/cards-stacking.css" />

        <div class="flex flex-col gap-4 justify-center items-center p-4 mainBody">

            <div class="grid gap-4 p-2 mx-auto w-full border pt-[200px] border-sky-500 max-w-[600px]">

                <div class="grid relative cards">
                    <StickyArticle i=1 />
                    <StickyArticle i=2 />
                    <StickyArticle i=3 />
                    <StickyArticle i=4 />
                </div>

                <div class="h-[500px] bg-neutral-700">content</div>
                <div class="h-[500px] bg-neutral-700">content</div>
                <div class="h-[500px] bg-neutral-700">content</div>
            </div>

        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•��°.*°.˚:*.´+°.•*/

#[component]
pub fn StickyArticle(i: i32) -> impl IntoView {
    let formatted_style = format!("--i:{}", i);

    const HARDCODED_TOP_OFFSET: &str = "top: 60px;";

    view! {
        <article
            class="sticky p-4 rounded-lg h-[150px] bg-sky-700"
            style=format!("{}; {}", formatted_style, HARDCODED_TOP_OFFSET)
        >
            <h2>"Card {i}"</h2>
            <p>"Lorem ipsum dolor sit amet consectetur adipisicing elit."</p>
        </article>
    }
}
