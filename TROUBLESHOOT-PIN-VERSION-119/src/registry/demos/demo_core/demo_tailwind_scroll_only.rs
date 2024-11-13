use leptos::*;

#[component]
pub fn DemoTailwindScrollOnly() -> impl IntoView {
    view! {
        <div
            dir="ltr"
            class="overflow-hidden relative px-2 h-[3800px] max-w-[1000px]"
            style="position:relative;--radix-scroll-area-corner-width:0px;--radix-scroll-area-corner-height:0px"
        >
            <div
                data-radix-scroll-area-viewport=""
                class="w-full h-full rounded-[inherit]"
                style="overflow-x:hidden;overflow-y:hidden"
            >
                <div style="min-width:100%;display:table">
                    <header class="px-12 h-[80vh] overflow-clip bg-neutral-400">
                        <div class="flex items-center mx-auto max-w-7xl h-full">
                            <h2 class="text-5xl font-bold text-pretty animate-fadeOutDown [animation-range:0px_300px] [animation-timeline:scroll()] supports-no-scroll-driven-animations:animate-none max-w-[14ch] leading-[1] md:text-[120px]">
                                Scroll driven animations are the future.
                            </h2>
                        </div>
                    </header>
                    <main class="px-12 mt-8 overflow-x-clip">
                        <div class="flex flex-col items-center mx-auto max-w-7xl text-2xl">
                            <p class="mb-4 w-full max-w-3xl text-4xl font-bold">
                                I need this to work in internet explorer!
                            </p>
                            <p class="mb-12 max-w-3xl">
                                We do not need a contract, do we what is a hamburger menu. I need this to work in internet explorer! I know somebody who can do this for a reasonable cost, but we try your eye, but can you change everything?
                            </p>
                            <h2 class="pt-32 pb-24 text-5xl font-bold text-center md:pt-60 md:pb-32 text-pretty animate-makeItBigger [animation-range:0%_60%] [animation-timeline:--quote] [view-timeline-name:--quote] supports-no-scroll-driven-animations:animate-none leading-[1] md:text-[160px]">
                                Can you make the font bigger?
                            </h2>
                            <p class="mt-72 mb-8 max-w-3xl">
                                Can you make it look like this clipart i found what you have given us is texty, we want sexy, yet we do not need a backup, it never goes down! make it sexy. Other agencies charge much lesser can you use a high definition screenshot can you make it look more designed i cant pay you . That is going to be a chunk of change the website does not have the theme i was going for can you make pink a little more pinkish, nor theres all this spanish text on my site.
                            </p>
                        </div>
                    </main>
                </div>
            </div>
        </div>
    }
}
