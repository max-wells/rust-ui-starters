use leptos::*;

use crate::{
    components::css_tricks::{
        css_trick_card_blog::CssTrickCardBlog,
        css_trick_nth_child_selector::CssTrickNthChildSelector,
        css_trick_stick_bottom_cards::CssTrickStickBottomCards,
    },
    registry::icons::others::dot::Dot,
};

#[component]
pub fn PageCssTricks() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">

            <CssTrickStickBottomCards />

            <CssTrickCardBlog />

            <CssTrickNthChildSelector />

            <TypingIndicator />
            <SidenavTailwind />
            <SlideTextEffectFail />
            <ImageTapeEffect />
            <CustomChecboxes />
            <RibbonShapeEffect />
            <TransitionInTailwind />
            <StepsProgressIndicator />
            <AccordionPureHtml />
            <FeedNestedUsers />
            <ActivityFeed />
            <TailwindDropdownMenu />
            <TailwindGlassmorphism />
            <TailwindOnlyTabs />
            <TestimonialSectionGrid />
            <AnimatedCardsTailwind />
            <CarouselWithFlowbiteScript />
            <FooterCool />
            <DropdownMenuList />
            <FooterNumberTwo />
            <BlogImageHeaderWithBackgroundEffect />
            <StickySections />
            <HoverEffectCard />
            <FancyUnderline />
            <TypingEffect />
            <CardRibbon />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SlideTextEffectFail() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-900">
                <div class="py-24 px-4 mx-auto w-full max-w-6xl md:px-6">
                    <div class="text-center">

                        <div class="text-3xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r md:text-4xl [text-wrap:balance] from-slate-200/60 to-50% to-slate-200">
                            Trusted by the most innovative minds in
                            <span class="inline-flex overflow-hidden flex-col text-indigo-500 h-[calc(theme(fontSize.3xl)*theme(lineHeight.tight))] md:h-[calc(theme(fontSize.4xl)*theme(lineHeight.tight))]">
                                <ul class="block leading-tight text-left animate-textSlide [&_li]:block">
                                    <li>Finance</li>
                                    <li>Tech</li>
                                    <li>AI</li>
                                    <li>Crypto</li>
                                    <li>eCommerce</li>
                                    <li aria-hidden="true">Finance</li>
                                </ul>
                            </span>
                        </div>

                    </div>

                </div>
            </main>

        </div>
    }
}

#[component]
pub fn CustomChecboxes() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative justify-center items-center min-h-screen bg-gradient-to-br from-emerald-500 via-lime-300 to-green-500">
            <div class="px-24 max-w-2xl">
                <div class="relative h-80 shadow-xl w-[32rem] shadow-black/60">
                    <div class="absolute -top-6 -left-10 w-20 h-12 bg-white -rotate-45 top-left"></div>
                    <div class="absolute -top-6 -left-10 w-20 h-12 bg-white border border-gray-100 top-left -rotate-[52deg]"></div>
                    <div class="absolute -top-6 -right-10 w-20 h-12 bg-white rotate-45 top-right"></div>
                    <div class="absolute -bottom-6 -right-10 w-20 h-12 bg-white -rotate-45 bottom-right"></div>
                    <div class="absolute -bottom-6 -left-10 w-20 h-12 bg-white rotate-45 bottom-left"></div>
                    <img
                        class="object-cover w-full h-full"
                        src="https://images.unsplash.com/photo-1533468659570-9cc9354310e4?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1317&q=80"
                        alt=""
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ImageTapeEffect() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden justify-center items-center min-h-screen bg-gradient-to-br from-emerald-500 via-lime-300 to-green-500">
            <div class="mx-auto max-w-2xl">
                <div class="flex flex-wrap gap-4 justify-center">
                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1605276374104-dee2a0ed3cd6?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1170&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category One
                                </p>
                                <p class="text-sm text-gray-400">Front View</p>
                            </header>
                        </div>
                    </label>

                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1630650231815-a567e2ed26cc?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1170&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category Two
                                </p>
                                <p class="text-sm text-gray-400">Night Lights</p>
                            </header>
                        </div>
                    </label>
                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1598228723793-52759bba239c?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1074&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category Three
                                </p>
                                <p class="text-sm text-gray-400">Bright Day</p>
                            </header>
                        </div>
                    </label>
                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1560518883-ce09059eeffa?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1073&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category Four
                                </p>
                                <p class="text-sm text-gray-400">Key Holder</p>
                            </header>
                        </div>
                    </label>
                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1628624747186-a941c476b7ef?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1170&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category Five
                                </p>
                                <p class="text-sm text-gray-400">Stand Tall</p>
                            </header>
                        </div>
                    </label>
                    <label class="relative cursor-pointer">
                        <input type="checkbox" class="sr-only peer" name="size-choice" />
                        <span class="absolute top-2 right-2 z-10 opacity-0 transition-all peer-checked:opacity-100">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="fill-blue-500 stroke-white"
                                width="32"
                                height="32"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="#2c3e50"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <circle cx="12" cy="12" r="9" />
                                <path d="M9 12l2 2l4 -4" />
                            </svg>
                        </span>
                        <div class="overflow-hidden bg-white rounded-lg ring ring-transparent shadow-md transition-all active:scale-95 grayscale peer-checked:ring-blue-500 peer-checked:grayscale-0">
                            <div>
                                <img
                                    src="https://images.unsplash.com/photo-1518780664697-55e3ad937233?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=765&q=80"
                                    alt="Sofa 1"
                                    class="object-cover w-48 h-28"
                                />
                            </div>
                            <header class="py-2.5 px-2.5">
                                <p class="text-lg font-bold tracking-wide text-gray-700">
                                    Category One
                                </p>
                                <p class="text-sm text-gray-400">Summer Vibe</p>
                            </header>
                        </div>
                    </label>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RibbonShapeEffect() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10 justify-center items-center min-h-screen bg-gradient-to-br from-lime-500 to-lime-500 via-lime-300/70">
            <div class="relative h-80 bg-white rounded-md shadow-xl z-[1] w-[520px]">
                <div class="overflow-hidden absolute -top-2 -left-2 w-40 h-40 ribbon before:absolute before:top-0 before:right-0 before:-z-[1] before:border-4 before:border-blue-500 after:absolute after:left-0 after:bottom-0 after:-z-[1] after:border-4 after:border-blue-500">
                    <div class="absolute -left-14 py-2.5 w-60 text-center text-white bg-gradient-to-br from-blue-600 via-blue-400 to-blue-500 shadow-md -rotate-45 top-[43px]">
                        Beyond Builder
                    </div>
                </div>
                <div class="flex justify-center items-center p-2 min-h-full">
                    <p class="text-xl font-bold text-gray-600">Your Content Goes Here</p>
                </div>
            </div>
            <div class="flex gap-5">
                <div class="relative w-40 h-40 bg-white rounded-md shadow-xl z-[1]">
                    <div class="absolute -top-2 right-4 ribbon drop-shadow-[2px_3px_2px_rgba(0,0,0,0.4)]">
                        <div class="flex justify-center py-2 px-1 w-9 h-12 text-lg font-bold text-center text-white bg-green-500 clip-path-polygon-[0_0,_100%_0,_100%_calc(100%_-_16px),_40%_100%,_0_calc(100%_-_12px)]">
                            <svg
                                width="24px"
                                height="24px"
                                aria-hidden="true"
                                focusable="false"
                                data-prefix="fas"
                                data-icon="check"
                                class="svg-inline--fa fa-check fa-w-16"
                                role="img"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 512 512"
                            >
                                <path
                                    fill="currentColor"
                                    d="M173.898 439.404l-166.4-166.4c-9.997-9.997-9.997-26.206 0-36.204l36.203-36.204c9.997-9.998 26.207-9.998 36.204 0L192 312.69 432.095 72.596c9.997-9.997 26.207-9.997 36.204 0l36.203 36.204c9.997 9.997 9.997 26.206 0 36.204l-294.4 294.401c-9.998 9.997-26.207 9.997-36.204-.001z"
                                ></path>
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="relative w-40 h-40 bg-white rounded-md shadow-xl z-[1]">
                    <div class="absolute -top-2 right-4 ribbon drop-shadow-[2px_3px_2px_rgba(0,0,0,0.4)]">
                        <div class="flex justify-center py-2 px-1 w-9 h-12 text-lg font-bold text-center text-white bg-rose-500 clip-path-polygon-[0_0,_100%_0,_100%_100%,_50%_calc(100%_-_8px),_0_100%]">
                            <svg
                                width="24px"
                                height="24px"
                                aria-hidden="true"
                                focusable="false"
                                data-prefix="fas"
                                data-icon="times"
                                class="svg-inline--fa fa-times fa-w-11"
                                role="img"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 352 512"
                            >
                                <path
                                    fill="currentColor"
                                    d="M242.72 256l100.07-100.07c12.28-12.28 12.28-32.19 0-44.48l-22.24-22.24c-12.28-12.28-32.19-12.28-44.48 0L176 189.28 75.93 89.21c-12.28-12.28-32.19-12.28-44.48 0L9.21 111.45c-12.28 12.28-12.28 32.19 0 44.48L109.28 256 9.21 356.07c-12.28 12.28-12.28 32.19 0 44.48l22.24 22.24c12.28 12.28 32.2 12.28 44.48 0L176 322.72l100.07 100.07c12.28 12.28 32.2 12.28 44.48 0l22.24-22.24c12.28-12.28 12.28-32.19 0-44.48L242.72 256z"
                                ></path>
                            </svg>
                        </div>
                    </div>
                </div>
                <div class="relative w-40 h-40 bg-white rounded-md shadow-xl z-[1]">
                    <div class="absolute -top-2 right-4 ribbon drop-shadow-[2px_3px_2px_rgba(0,0,0,0.4)]">
                        <div class="flex justify-center py-2 px-1 w-9 h-12 text-lg font-bold text-center text-white bg-amber-500 clip-path-polygon-[0_0,_100%_0,_100%_calc(100%_-_8px),_50%_100%,_0_calc(100%_-_8px)]">
                            <svg
                                width="24px"
                                height="24px"
                                aria-hidden="true"
                                focusable="false"
                                data-prefix="far"
                                data-icon="star"
                                class="svg-inline--fa fa-star fa-w-18"
                                role="img"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 576 512"
                            >
                                <path
                                    fill="currentColor"
                                    d="M528.1 171.5L382 150.2 316.7 17.8c-11.7-23.6-45.6-23.9-57.4 0L194 150.2 47.9 171.5c-26.2 3.8-36.7 36.1-17.7 54.6l105.7 103-25 145.5c-4.5 26.3 23.2 46 46.4 33.7L288 439.6l130.7 68.7c23.2 12.2 50.9-7.4 46.4-33.7l-25-145.5 105.7-103c19-18.5 8.5-50.8-17.7-54.6zM388.6 312.3l23.7 138.4L288 385.4l-124.3 65.3 23.7-138.4-100.6-98 139-20.2 62.2-126 62.2 126 139 20.2-100.6 98z"
                                ></path>
                            </svg>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidenavTailwind() -> impl IntoView {
    // @layer components {
    //     .hamburger div:first-child {
    //         @apply rotate-45 translate-y-1.5;
    //     }
    //     .hamburger div:last-child {
    //         @apply -rotate-45 -translate-y-1;
    //     }
    // }
    view! {
        <div class="h-screen bg-gray-50">
            <header>
                <div class="relative z-20 bg-white border-b">
                    <div class="px-6 md:px-12 lg:container lg:py-4 lg:px-6 lg:mx-auto">
                        <div class="flex justify-between items-center">
                            <div class="relative z-20">
                                <a href="#">
                                    <img
                                        src="https://tailus.io/sources/blocks/navigation-layout/preview/images/logo.svg"
                                        alt="logo-tailus"
                                        class="w-32"
                                    />
                                </a>
                            </div>

                            <div class="flex justify-end items-center border-l lg:border-l-0">
                                <input
                                    type="checkbox"
                                    name="hamburger"
                                    id="hamburger"
                                    class="peer"
                                    hidden
                                />
                                <label
                                    for="hamburger"
                                    class="block relative z-20 p-6 -mr-6 cursor-pointer lg:hidden peer-checked:hamburger"
                                >
                                    <div
                                        aria-hidden="true"
                                        class="m-auto w-6 h-0.5 rounded transition duration-300 bg-sky-900"
                                    ></div>
                                    <div
                                        aria-hidden="true"
                                        class="m-auto mt-2 w-6 h-0.5 rounded transition duration-300 bg-sky-900"
                                    ></div>
                                </label>

                                <div class="fixed inset-0 bg-white border-r shadow-xl transition duration-300 lg:static lg:w-auto lg:border-r-0 lg:shadow-none lg:translate-x-0 peer-checked:translate-x-0 w-[calc(100%-4.5rem)] translate-x-[-100%]">
                                    <div class="flex flex-col justify-between h-full lg:flex-row lg:items-center">
                                        <ul class="px-6 pt-32 space-y-8 text-gray-700 md:px-12 lg:flex lg:pt-0 lg:space-y-0 lg:space-x-12">
                                            <li>
                                                <a
                                                    href="#"
                                                    class="relative group before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:bg-cyan-100"
                                                >
                                                    <span class="relative text-cyan-800">Home</span>
                                                </a>
                                            </li>
                                            <li>
                                                <a
                                                    href="#"
                                                    class="relative group before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-cyan-100 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
                                                >
                                                    <span class="relative group-hover:text-cyan-800">
                                                        Services
                                                    </span>
                                                </a>
                                            </li>
                                            <li>
                                                <a
                                                    href="#"
                                                    class="relative group before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-cyan-100 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
                                                >
                                                    <span class="relative group-hover:text-cyan-800">
                                                        Portfolio
                                                    </span>
                                                </a>
                                            </li>
                                            <li>
                                                <a
                                                    href="#"
                                                    class="relative group before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-cyan-100 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
                                                >
                                                    <span class="relative group-hover:text-cyan-800">
                                                        About us
                                                    </span>
                                                </a>
                                            </li>
                                        </ul>

                                        <div class="py-8 px-6 border-t md:py-16 md:px-12 lg:py-0 lg:pr-0 lg:pl-6 lg:border-t-0 lg:border-l">
                                            <a
                                                href="#"
                                                class="block py-3 px-6 text-center text-white bg-gradient-to-r to-cyan-400 rounded-full from-sky-600"
                                            >
                                                Get started
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}

#[component]
pub fn TransitionInTailwind() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center min-h-screen bg-indigo-500">
            <div class="box-content px-4 mx-auto space-y-4 border-r-8 border-l-8 border-indigo-400 border-opacity-50 group w-18">
                <div class="w-16 h-16 bg-white rounded-lg shadow-xl opacity-25 transition duration-200 transform group-hover:opacity-100"></div>
                <div class="w-16 h-16 bg-white rounded-lg shadow-xl opacity-25 transition duration-300 transform group-hover:opacity-100"></div>
                <div class="w-16 h-16 bg-white rounded-lg shadow-xl opacity-25 transition duration-500 transform group-hover:opacity-100"></div>
                <div class="w-16 h-16 bg-white rounded-lg shadow-xl opacity-25 transition duration-1000 transform group-hover:opacity-100"></div>
            </div>

        </div>
    }
}

#[component]
pub fn StepsProgressIndicator() -> impl IntoView {
    // .hover\:ring-opacity-100:hover {
    //     --tw-ring-opacity: 1;
    // }
    view! {
        <div class="flex flex-col justify-center items-center p-10 w-screen min-h-screen text-gray-700">
            <div class="flex relative flex-col w-full max-w-screen-md">
                <div class="flex absolute top-0 justify-between px-1 mt-4 w-full">
                    <div class="flex-grow border-t-2 border-green-500"></div>
                    <div class="flex-grow border-t-2 border-green-500"></div>
                    <div class="flex-grow border-t-2 border-gray-300"></div>
                    <div class="flex-grow border-t-2 border-gray-300"></div>
                </div>
                <div class="flex relative justify-between w-full">

                    <div class="flex flex-col items-center">
                        <button class="flex justify-center items-center w-8 h-8 bg-green-500 rounded-full ring-4 ring-green-200 ring-opacity-0 transition duration-200 hover:ring-opacity-100">
                            <svg
                                class="w-4 h-4 text-green-50 stroke-current"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </button>
                        <span class="absolute mt-12 w-36 text-xs font-semibold tracking-wide text-center uppercase">
                            First Step
                        </span>
                    </div>

                    <div class="flex flex-col items-center">
                        <button class="flex justify-center items-center w-8 h-8 bg-green-500 rounded-full ring-4 ring-green-200 ring-opacity-0 transition duration-200 hover:ring-opacity-100">
                            <svg
                                class="w-4 h-4 text-green-50 stroke-current"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </button>
                        <span class="absolute mt-12 w-36 text-xs font-semibold tracking-wide text-center uppercase">
                            Second Step
                        </span>
                    </div>

                    <div class="flex flex-col items-center">
                        <div class="flex justify-center items-center w-8 h-8 bg-white rounded-full border-2 border-green-500 ring-4 ring-green-200">
                            <span class="text-sm font-semibold text-green-500">3</span>
                        </div>
                        <span class="absolute mt-12 w-36 text-xs font-semibold tracking-wide text-center text-green-500 uppercase">
                            Third Step
                        </span>
                    </div>

                    <div class="flex flex-col items-center">
                        <div class="flex justify-center items-center w-8 h-8 bg-white rounded-full border-2 border-gray-300">
                            <span class="text-sm font-semibold text-gray-400">4</span>
                        </div>
                        <span class="absolute mt-12 w-36 text-xs font-semibold tracking-wide text-center text-gray-400 uppercase">
                            Fourth Step
                        </span>
                    </div>

                    <div class="flex flex-col items-center">
                        <div class="flex justify-center items-center w-8 h-8 bg-white rounded-full border-2 border-gray-300">
                            <span class="text-sm font-semibold text-gray-400">5</span>
                        </div>
                        <span class="absolute mt-12 w-36 text-xs font-semibold tracking-wide text-center text-gray-400 uppercase">
                            Fifth Step
                        </span>
                    </div>

                </div>
            </div>

        </div>
    }
}

#[component]
pub fn AccordionPureHtml() -> impl IntoView {
    // Source : https://wind-ui.com/components/accordions/

    view! {
        <section class="w-full rounded divide-y divide-slate-200">
            <details class="p-4 group" open>
                <summary class="relative pr-8 font-medium list-none transition-colors duration-300 cursor-pointer focus-visible:outline-none [&::-webkit-details-marker]:hidden text-slate-700 group-hover:text-slate-900">
                    How does TailwindCSS works?
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="absolute right-0 top-1 w-4 h-4 transition duration-300 stroke-slate-700 shrink-0 group-open:rotate-45"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="1.5"
                        aria-labelledby="title-ac01 desc-ac01"
                    >
                        <title id="title-ac01">Open icon</title>
                        <desc id="desc-ac01">icon that represents the state of the summary</desc>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                    </svg>
                </summary>
                <p class="mt-4 text-slate-500">
                    Tailwind CSS works by scanning all of your HTML files, JavaScript components, and any other templates for class names, generating the corresponding styles and then writing them to a static CSS file.
                </p>
            </details>
            <details class="p-4 group">
                <summary class="relative pr-8 font-medium list-none transition-colors duration-300 cursor-pointer focus-visible:outline-none [&::-webkit-details-marker]:hidden text-slate-700 group-hover:text-slate-900">
                    How do I install TailwindCSS?
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="absolute right-0 top-1 w-4 h-4 transition duration-300 stroke-slate-700 shrink-0 group-open:rotate-45"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="1.5"
                        aria-labelledby="title-ac02 desc-ac02"
                    >
                        <title id="title-ac02">Open icon</title>
                        <desc id="desc-ac02">icon that represents the state of the summary</desc>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                    </svg>
                </summary>
                <p class="mt-4 text-slate-500">
                    The simplest and fastest way to get up and running with Tailwind CSS from scratch is with the Tailwind CLI tool. The CLI is also available as a standalone executable if you want to use it without installing Node.js. Install tailwindcss via npm, and create your tailwind.config.js file.
                </p>
            </details>
            <details class="p-4 group">
                <summary class="relative pr-8 font-medium list-none transition-colors duration-300 cursor-pointer focus-visible:outline-none [&::-webkit-details-marker]:hidden text-slate-700 group-hover:text-slate-900">
                    What is Wind UI about?
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="absolute right-0 top-1 w-4 h-4 transition duration-300 stroke-slate-700 shrink-0 group-open:rotate-45"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="1.5"
                        aria-labelledby="title-ac03 desc-ac03"
                    >
                        <title id="title-ac03">Open icon</title>
                        <desc id="desc-ac03">icon that represents the state of the summary</desc>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                    </svg>
                </summary>
                <p class="mt-4 text-slate-500">
                    Expertly made, responsive, accessible components in React and HTML ready to be used on your website or app. Just copy and paste them on your Tailwind CSS project.
                </p>
            </details>
            <details class="p-4 group">
                <summary class="relative pr-8 font-medium list-none transition-colors duration-300 cursor-pointer focus-visible:outline-none [&::-webkit-details-marker]:hidden text-slate-700 group-hover:text-slate-900">
                    How do I use Wind UI components?
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="absolute right-0 top-1 w-4 h-4 transition duration-300 stroke-slate-700 shrink-0 group-open:rotate-45"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="1.5"
                        aria-labelledby="title-ac04 desc-ac04"
                    >
                        <title id="title-ac04">Open icon</title>
                        <desc id="desc-ac04">icon that represents the state of the summary</desc>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                    </svg>
                </summary>
                <p class="mt-4 text-slate-500">
                    All components can be copied and pasted and easily implemented in your tailwind css projects. You can choose which language you want to copy the desired component and just hover and click on the component you need and paste it on your project.
                </p>
            </details>
        </section>
    }
}

#[component]
pub fn FeedNestedUsers() -> impl IntoView {
    view! {
        <ul
            aria-label="Nested user feed"
            role="feed"
            class="flex relative flex-col gap-12 py-12 pl-6 before:absolute before:top-0 before:left-6 before:h-full before:border before:-translate-x-1/2 before:border-slate-200 before:border-dashed after:absolute after:top-6 after:left-6 after:bottom-6 after:border after:-translate-x-1/2 after:border-slate-200"
        >
            <li role="article" class="relative pl-6">
                <div class="flex flex-col flex-1 gap-2">
                    <a
                        href="#"
                        class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                    >
                        <img
                            src="https://i.pravatar.cc/48?img=1"
                            alt="user name"
                            title="user name"
                            width="48"
                            height="48"
                            class="max-w-full rounded-full"
                        />
                    </a>
                    <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                        <span class="flex-1">
                            Mary Jane
                            <span class="text-sm font-normal text-slate-500">
                                created a new thread
                            </span>
                        </span>
                        <span class="text-xs font-normal text-slate-400">3 hours ago</span>
                    </h4>
                    <p class="text-sm text-slate-500">
                        We just released windUI v1.5, which includes a brand new component. An activity feed is a chronological record of system events or user actions. Have a look at the feed page and let me know what you think. Feedback is highly appreciated.
                    </p>
                </div>
            </li>
            <li role="article" class="relative pl-6">
                <div class="flex flex-col flex-1 gap-2">
                    <a
                        href="#"
                        class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                    >
                        <img
                            src="https://i.pravatar.cc/48?img=12"
                            alt="user name"
                            title="user name"
                            width="48"
                            height="48"
                            class="max-w-full rounded-full"
                        />
                    </a>
                    <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                        <span class="flex-1">
                            John langan
                            <span class="text-sm font-normal text-slate-500">liked the thread</span>
                        </span>
                        <span class="text-xs font-normal text-slate-400">2 hours ago</span>
                    </h4>
                </div>
                <ul
                    role="group"
                    class="flex relative flex-col gap-12 py-12 pl-6 after:absolute after:top-12 after:left-6 after:bottom-12 after:border after:-translate-x-1/2 after:border-slate-200 before:absolute before:top-6 before:left-6 before:bottom-6 before:border before:-translate-x-1/2 before:border-slate-200 before:border-dashed"
                >
                    <li role="article" class="relative pl-6">
                        <div class="flex flex-col flex-1 gap-2">
                            <a
                                href="#"
                                class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                            >
                                <img
                                    src="https://i.pravatar.cc/48?img=1"
                                    alt="user name"
                                    title="user name"
                                    width="48"
                                    height="48"
                                    class="max-w-full rounded-full"
                                />
                            </a>
                            <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                                <span class="flex-1">
                                    Mary Jane
                                    <span class="text-sm font-normal text-slate-500">replied</span>
                                </span>
                                <span class="text-xs font-normal text-slate-400">2 hours ago</span>
                            </h4>
                            <p class="text-sm text-slate-500">
                                Hey john! Did you had a look at the new component?
                            </p>
                        </div>
                    </li>
                    <li role="article" class="relative pl-6">
                        <div class="flex flex-col flex-1 gap-2">
                            <a
                                href="#"
                                class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                            >
                                <img
                                    src="https://i.pravatar.cc/48?img=12"
                                    alt="user name"
                                    title="user name"
                                    width="48"
                                    height="48"
                                    class="max-w-full rounded-full"
                                />
                            </a>
                            <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                                <span class="flex-1">
                                    John langan
                                    <span class="text-sm font-normal text-slate-500">replied</span>
                                </span>
                                <span class="text-xs font-normal text-slate-400">2 hours ago</span>
                            </h4>
                            <p class="text-sm text-slate-500">
                                It looks awesome! Good one! The only thing that I would tweak is the spacing in smaller screens. Other than that it looks superb! Really loved the color as well.
                            </p>
                        </div>
                    </li>
                    <li role="article" class="relative pl-6">
                        <div class="flex flex-col flex-1 gap-2">
                            <a
                                href="#"
                                class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                            >
                                <img
                                    src="https://i.pravatar.cc/48?img=1"
                                    alt="user name"
                                    title="user name"
                                    width="48"
                                    height="48"
                                    class="max-w-full rounded-full"
                                />
                            </a>
                            <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                                <span class="flex-1">
                                    Mary Jane
                                    <span class="text-sm font-normal text-slate-500">replied</span>
                                </span>
                                <span class="text-xs font-normal text-slate-400">3 hours ago</span>
                            </h4>
                            <p class="text-sm text-slate-500">
                                Thanks so much . Your feedback is highly appreciated.
                            </p>
                        </div>
                    </li>
                </ul>
            </li>
            <li role="article" class="relative pl-6">
                <div class="flex flex-col flex-1 gap-2">
                    <a
                        href="#"
                        class="inline-flex absolute -left-3 z-10 justify-center items-center w-6 h-6 text-white rounded-full ring-2 ring-white"
                    >
                        <img
                            src="https://i.pravatar.cc/48?img=13"
                            alt="user name"
                            title="user name"
                            width="48"
                            height="48"
                            class="max-w-full rounded-full"
                        />
                    </a>
                    <h4 class="flex flex-col items-start text-base font-medium leading-6 md:flex-row lg:items-center text-slate-700">
                        <span class="flex-1">
                            Manos Gaitanakis
                            <span class="text-sm font-normal text-slate-500">commented</span>
                        </span>
                        <span class="text-xs font-normal text-slate-400">3 hours ago</span>
                    </h4>
                    <p class="text-sm text-slate-500">
                        "Love it! I really like how the nested feeds are working as well. Is that going to be multi-nested? Or maybe stay in just one level. Also any ides on how I can remove the time stamp from the feeds?"
                    </p>
                </div>
            </li>
        </ul>
    }
}

#[component]
pub fn ActivityFeed() -> impl IntoView {
    view! {
        <ul
            aria-label="Activity feed"
            role="feed"
            class="flex relative flex-col gap-12 py-12 pl-8 before:absolute before:top-0 before:left-8 before:h-full before:border before:-translate-x-1/2 before:border-slate-200 before:border-dashed after:absolute after:top-6 after:left-8 after:bottom-6 after:border after:-translate-x-1/2 after:border-slate-200"
        >
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M6 6.878V6a2.25 2.25 0 012.25-2.25h7.5A2.25 2.25 0 0118 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0A2.25 2.25 0 004.5 9v.878m13.5-3A2.25 2.25 0 0119.5 9v.878m0 0a2.246 2.246 0 00-.75-.128H5.25c-.263 0-.515.045-.75.128m15 0A2.25 2.25 0 0121 12v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6c0-.98.626-1.813 1.5-2.122"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">UI/UX project created</h4>
                    <p class="text-sm text-slate-500">13:12pm</p>
                </div>
            </li>
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">
                        Task list created for project
                    </h4>
                    <p class="text-sm text-slate-500">13:31pm</p>
                </div>
            </li>
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">
                        Warning! Project name cannot be empty
                    </h4>
                    <p class="text-sm text-slate-500">13:32pm</p>
                </div>
            </li>
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">New user added</h4>
                    <p class="text-sm text-slate-500">13:56pm</p>
                </div>
            </li>
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">
                        Warning! Project is going to be expired
                    </h4>
                    <p class="text-sm text-slate-500">13:32pm</p>
                </div>
            </li>
            <li role="article" class="relative pl-8">
                <span class="flex absolute left-0 z-10 justify-center items-center w-10 h-10 rounded-full ring-2 ring-white -translate-x-1/2 text-slate-700 bg-slate-200">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-5 h-5"
                        role="presentation"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
                        />
                    </svg>
                </span>
                <div class="flex flex-col flex-1 gap-0">
                    <h4 class="text-base font-medium text-slate-700">Project deleted</h4>
                    <p class="text-sm text-slate-500">13:32pm</p>
                </div>
            </li>
        </ul>
    }
}

#[component]
pub fn TailwindDropdownMenu() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-12 justify-center items-center p-8 min-h-screen text-white bg-gradient-to-b from-rose-950 to-teal-950 [&:has(input[name=radio-panel]:not(#option-0):checked)_#label-0]:opacity-100 [&:has(input[name=radio-panel]:not(#option-0):checked)_#options]:opacity-100 [&:has(input[name=radio-panel]:not(#option-0):checked)_#options]:translate-y-0">
            <input type="radio" id="option-0" name="radio-panel" class="sr-only peer/option-0" />
            <input type="radio" id="option-1" name="radio-panel" class="sr-only peer/option-1" />
            <input type="radio" id="option-2" name="radio-panel" class="sr-only peer/option-2" />
            <input type="radio" id="option-3" name="radio-panel" class="sr-only peer/option-3" />

            <div class="peer-checked/option-1:[&_#panel-1]:translate-x-0 peer-checked/option-1:[&_#panel-1]:opacity-100 peer-checked/option-1:[&_#label-1]:border-rose-400/90 peer-checked/option-1:[&_#label-1>span]:rotate-0 peer-checked/option-1:[&_#panel-1_li]:opacity-100 peer-checked/option-1:[&_#panel-1_li]:translate-y-0 peer-checked/option-1:[&_#panel-1_li]:delay-[--d] peer-checked/option-2:[&_#panel-2]:translate-x-0 peer-checked/option-2:[&_#panel-2]:opacity-100 peer-checked/option-2:[&_#label-2]:border-rose-400/90 peer-checked/option-2:[&_#label-2>span]:rotate-0 peer-checked/option-2:[&_#panel-2_li]:opacity-100 peer-checked/option-2:[&_#panel-2_li]:translate-y-0 peer-checked/option-2:[&_#panel-2_li]:delay-[--d] peer-checked/option-3:[&_#panel-3]:translate-x-0 peer-checked/option-3:[&_#panel-3]:opacity-100 peer-checked/option-3:[&_#label-3]:border-rose-400/90 peer-checked/option-3:[&_#label-3>span]:rotate-0 peer-checked/option-3:[&_#panel-3_li]:opacity-100 peer-checked/option-3:[&_#panel-3_li]:translate-y-0 peer-checked/option-3:[&_#panel-3_li]:delay-[--d]">

                <div
                    id="wrapper"
                    class="flex gap-3 *:text-sm *:w-fit *:flex max-[400px]:flex-col *:items-center *:gap-2 *:border *:border-rose-300/50 *:cursor-pointer *:rounded-full *:py-1 *:px-3 *:transition-all *:duration-300 *:bg-black/50 *:text-white [&_label>span]:rotate-180 [&_label>span]:transition-transform [&_label>span]:duration-300 hover:*:border-rose-400/90 focus:*:border-rose-400/90"
                >

                    <label id="label-1" for="option-1">
                        Artwork
                        <span class="material-symbols-outlined">expand_more</span>
                    </label>

                    <label id="label-2" for="option-2">
                        All
                        <span class="material-symbols-outlined">expand_more</span>
                    </label>

                    <label id="label-3" for="option-3">
                        All tags
                        <span class="material-symbols-outlined">expand_more</span>
                    </label>
                    <label id="label-0" for="option-0" class="opacity-0 !px-2">
                        <span class="text-sm material-symbols-outlined">close</span>
                    </label>
                </div>

                <div
                    id="options"
                    class="grid overflow-hidden place-content-center p-4 mt-4 rounded-xl border opacity-0 transition-all duration-500 translate-y-12 focus-within:opacity-100 focus-within:translate-y-0 bg-black/50 border-rose-400/50 [grid-template-area:'stack'] *:[grid-area:stack] *:transition-all *:duration-500 *:-translate-x-96 *:opacity-0 *:w-full [&_h2]:text-sm [&_h2]:font-medium [&_h2]:uppercase [&_h2]:my-4 min-[401px]:[&_ul]:columns-2 [&_ul]:text-sm [&_ul]:space-y-3 [&_li]:relative [&_li]:flex [&_li]:items-center [&_li]:pl-4 [&_li]:isolate [&_li]:transition-all [&_li]:duration-300 [&_li]:break-inside-avoid // not working [&_li]:opacity-0 [&_li]:translate-y-12 [&_li:hover>button]:text-white before:[&_li]:absolute before:[&_li]:top-0 before:[&_li]:left-0 before:[&_li]:w-2 before:[&_li]:h-full before:[&_li]:absolute before:[&_li]:break-inside-avoid-column // not working before:[&_li]:rounded-[2px] before:[&_li]:transition-all before:[&_li]:duration-300 before:[&_li]:-z-10 [&_li>button]:outline-none [&_li>button]:border-none [&_li>button]:ring-0 before:[&_li:has(button:focus-visible)]:w-full before:[&_li:has(button:focus-visible)]:h-full [&_.search]:w-full [&_.search]:flex [&_.search]:items-center [&_.search]:gap-2 [&_.search]:transtion [&_.search]:duration-300 [&_.search]:border-b [&_.search]:p-2 [&_.search>input]:w-32 [&_.search>*]:border-none [&_.search>*]:outline-none [&_.search>*]:ring-0 [&_.search>input]:bg-transparent focus-within:[&_.search]:bg-white/20 hover:before:[&_li]:w-full hover:before:[&_li]:h-full"
                >

                    <div id="panel-1">
                        <h2>Artwork</h2>
                        <ul>
                            <li style="--d:300ms" class="before:bg-rose-500">
                                <button type="button">Artwork</button>
                            </li>
                            <li style="--d:325ms" class="before:bg-sky-500">
                                <button type="button">Support</button>
                            </li>
                            <li style="--d:350ms" class="before:bg-green-500">
                                <button type="button">Coding</button>
                            </li>
                            <li style="--d:375ms" class="before:bg-amber-500">
                                <button type="button">Contests</button>
                            </li>
                            <li style="--d:400ms" class="before:bg-cyan-500">
                                <button type="button">Game Engine</button>
                            </li>
                            <li style="--d:425ms" class="before:bg-red-500">
                                <button type="button">Jobs</button>
                            </li>
                            <li style="--d:450ms" class="before:bg-yellow-500">
                                <button type="button">General Formums</button>
                            </li>
                            <li style="--d:475ms" class="before:bg-lime-500">
                                <button type="button">Promotions</button>
                            </li>
                        </ul>
                    </div>

                    <div id="panel-2">
                        <h2>All categories</h2>
                        <ul>
                            <li style="--d:300ms" class="before:bg-fuchsia-500">
                                <button type="button">Something 1</button>
                            </li>
                            <li style="--d:325ms" class="before:bg-teal-500">
                                <button type="button">Something 2</button>
                            </li>
                            <li style="--d:350ms" class="before:bg-rose-500">
                                <button type="button">Something 3</button>
                            </li>
                            <li style="--d:375ms" class="before:bg-lime-500">
                                <button type="button">Something 4</button>
                            </li>
                            <li style="--d:400ms" class="before:bg-orange-500">
                                <button type="button">Something 5</button>
                            </li>
                            <li style="--d:425ms" class="before:bg-purple-500">
                                <button type="button">Something 6</button>
                            </li>
                        </ul>
                    </div>
                    <div id="panel-3">
                        <h2>All tags</h2>
                        <ul>
                            <li style="--d:300ms" class="before:bg-red-500">
                                <button type="button">UI</button>
                            </li>
                            <li style="--d:325ms" class="before:bg-blue-500">
                                <button type="button">UX</button>
                            </li>
                            <li style="--d:350ms" class="before:bg-slate-500">
                                <button type="button">Front End</button>
                            </li>
                            <li style="--d:375ms" class="before:bg-indigo-500">
                                <button type="button">Back End</button>
                            </li>
                            <li style="--d:400ms" class="before:bg-yellow-500">
                                <button type="button">Developer</button>
                            </li>
                            <li style="--d:425ms" class="before:bg-emerald-500">
                                <button type="button">Designer</button>
                            </li>
                            <li style="--d:450ms" class="before:bg-pink-500">
                                <button type="button">Systems</button>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TailwindGlassmorphism() -> impl IntoView {
    view! {
        <section class="grid place-items-center min-h-screen text-white bg-slate-800 isolate">
            <div class="relative max-w-xs">
                <div class="flex flex-col gap-6 items-center p-12 text-center rounded-2xl border bg-white/5 border-slate-600 backdrop-blur">
                    <img
                        class="object-cover mx-auto w-24 h-24 rounded-full border-4 border-slate-600"
                        src="https://images.pexels.com/photos/1239291/pexels-photo-1239291.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
                        alt=""
                    />
                    <div>
                        <h3 class="text-2xl font-bold">Emily Johnson</h3>
                        <p class="mt-1 text-cyan-300">Marketing Manager</p>
                    </div>
                    <p>
                        Creative marketing guru who strategizes and executes impactful campaigns that resonate with the audiences.
                    </p>
                    <button class="py-2 px-6 rounded-full border transition border-slate-500 hover:bg-indigo-200/50">
                        Book a call
                    </button>
                </div>

                <div class="absolute -top-10 -left-14 w-32 h-32 bg-gradient-to-r from-indigo-500 to-indigo-300 rounded-full -z-10"></div>
                <div class="absolute -bottom-8 -right-14 w-32 h-32 bg-gradient-to-r from-cyan-600 to-cyan-300 rounded-full -z-10"></div>

            </div>
        </section>
    }
}

#[component]
pub fn TailwindOnlyTabs() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 justify-center items-center min-h-screen bg-zinc-900 text-zinc-200">
            <h1>Tailwind Tabs</h1>
            <div class="w-full max-w-md [&_label]:cursor-pointer">

                <input
                    type="radio"
                    name="sub-tab-toggñe"
                    id="radio-tab-1"
                    class="sr-only peer/tab-1"
                    checked
                />
                <input
                    type="radio"
                    name="sub-tab-toggñe"
                    id="radio-tab-2"
                    class="sr-only peer/tab-2"
                />
                <input
                    type="radio"
                    name="sub-tab-toggñe"
                    id="radio-tab-3"
                    class="sr-only peer/tab-3"
                />

                <div class="flex px-4 gap-[1px] peer-checked/tab-1:[&>label:nth-child(1)]:bg-blue-800 peer-checked/tab-2:[&>label:nth-child(2)]:bg-green-800 peer-checked/tab-3:[&>label:nth-child(3)]:bg-rose-800 *:rounded-t-xl *:py-1 *:px-4 *:transition *:duration-300 hover:*:bg-zinc-800">
                    <label for="radio-tab-1">Tab 1</label>
                    <label for="radio-tab-2">Tab 2</label>
                    <label for="radio-tab-3">Tab 3</label>
                </div>

                <div class="p-6 rounded-xl bg-zinc-800 [&_label]:rounded-md [&_label]:text-white [&_label]:py-1 [&_label]:px-4 [&_label]:w-full [&_label]:transition [&_label]:duration-300 [&_label]:border-b [&_label]:border-zinc-500 peer-checked/tab-1:[&_#sub-tab-1]:opacity-100 peer-checked/tab-1:[&_#sub-tab-1]:pointer-events-auto peer-checked/tab-1:[&_#panels-contents-1]:opacity-100 peer-checked/tab-2:[&_#sub-tab-2]:opacity-100 peer-checked/tab-2:[&_#sub-tab-2]:pointer-events-auto peer-checked/tab-2:[&_#panels-contents-2]:opacity-100 peer-checked/tab-3:[&_#sub-tab-3]:opacity-100 peer-checked/tab-3:[&_#sub-tab-3]:pointer-events-auto peer-checked/tab-3:[&_#panels-contents-3]:opacity-100">

                    <input
                        type="radio"
                        name="panel-toggle-1"
                        id="radio-panel-1.1"
                        class="sr-only peer/panel-1.1"
                        checked
                    />
                    <input
                        type="radio"
                        name="panel-toggle-1"
                        id="radio-panel-1.2"
                        class="sr-only peer/panel-1.2"
                    />
                    <input
                        type="radio"
                        name="panel-toggle-1"
                        id="radio-panel-1.3"
                        class="sr-only peer/panel-1.3"
                    />
                    <input
                        type="radio"
                        name="panel-toggle-2"
                        id="radio-panel-2.1"
                        class="sr-only peer/panel-2.1"
                        checked
                    />
                    <input
                        type="radio"
                        name="panel-toggle-2"
                        id="radio-panel-2.2"
                        class="sr-only peer/panel-2.2"
                    />
                    <input
                        type="radio"
                        name="panel-toggle-2"
                        id="radio-panel-2.3"
                        class="sr-only peer/panel-2.3"
                    />
                    <input
                        type="radio"
                        name="panel-toggle-3"
                        id="radio-panel-3.1"
                        class="sr-only peer/panel-3.1"
                        checked
                    />
                    <input
                        type="radio"
                        name="panel-toggle-3"
                        id="radio-panel-3.2"
                        class="sr-only peer/panel-3.2"
                    />
                    <input
                        type="radio"
                        name="panel-toggle-3"
                        id="radio-panel-3.3"
                        class="sr-only peer/panel-3.3"
                    />

                    <div class="grid [template-grid-areas:'stack'] *:[grid-area:stack] *:transition-opacity *:duration-300 *:opacity-0 *:pointer-events-none *:flex *:gap-[1px] peer-checked/panel-1.1:[&_#sub-tab-1>label:nth-child(1)]:bg-blue-800 peer-checked/panel-1.2:[&_#sub-tab-1>label:nth-child(2)]:bg-blue-800 peer-checked/panel-1.3:[&_#sub-tab-1>label:nth-child(3)]:bg-blue-800 peer-checked/panel-2.1:[&_#sub-tab-2>label:nth-child(1)]:bg-green-800 peer-checked/panel-2.2:[&_#sub-tab-2>label:nth-child(2)]:bg-green-800 peer-checked/panel-2.3:[&_#sub-tab-2>label:nth-child(3)]:bg-green-800 peer-checked/panel-3.1:[&_#sub-tab-3>label:nth-child(1)]:bg-rose-800 peer-checked/panel-3.2:[&_#sub-tab-3>label:nth-child(2)]:bg-rose-800 peer-checked/panel-3.3:[&_#sub-tab-3>label:nth-child(3)]:bg-rose-800">

                        <div id="sub-tab-1">
                            <label for="radio-panel-1.1">SubTab 1.1</label>
                            <label for="radio-panel-1.2">SubTab 1.2</label>
                            <label for="radio-panel-1.3">SubTab 1.3</label>
                        </div>

                        <div id="sub-tab-2">
                            <label for="radio-panel-2.1">SubTab 2.1</label>
                            <label for="radio-panel-2.2">SubTab 2.2</label>
                            <label for="radio-panel-2.3">SubTab 2.3</label>
                        </div>

                        <div id="sub-tab-3">
                            <label for="radio-panel-3.1">SubTab 3.1</label>
                            <label for="radio-panel-3.2">SubTab 3.2</label>
                            <label for="radio-panel-3.3">SubTab 3.3</label>
                        </div>
                    </div>

                    <div class="grid mt-2 [grid-template-areas:'contents'] *:[grid-area:contents] *:transition-opacity *:duration-300 *:rounded-md *:grid *:[grid-template-areas:'panels'] *:opacity-0 [&>div>div]:[grid-area:panels] [&>div>div]:transition-opacity [&>div>div]:duration-300 [&>div>div]:opacity-0 [&>div>div]:p-8 [&>div>div]:grid [&>div>div]:place-content-center peer-checked/panel-1.1:[&_#panel-11]:opacity-100 peer-checked/panel-1.2:[&_#panel-12]:opacity-100 peer-checked/panel-1.3:[&_#panel-13]:opacity-100 peer-checked/panel-2.1:[&_#panel-21]:opacity-100 peer-checked/panel-2.2:[&_#panel-22]:opacity-100 peer-checked/panel-2.3:[&_#panel-23]:opacity-100 peer-checked/panel-3.1:[&_#panel-31]:opacity-100 peer-checked/panel-3.2:[&_#panel-32]:opacity-100 peer-checked/panel-3.3:[&_#panel-33]:opacity-100">
                        <div id="panels-contents-1" class="bg-blue-800">
                            <div id="panel-11">panel 1.1</div>
                            <div id="panel-12">panel 1.2</div>
                            <div id="panel-13">panel 1.3</div>
                        </div>

                        <div id="panels-contents-2" class="bg-green-800">
                            <div id="panel-21">panel 2.1</div>
                            <div id="panel-22">panel 2.2</div>
                            <div id="panel-23">panel 2.3</div>
                        </div>

                        <div id="panels-contents-3" class="bg-rose-800">
                            <div id="panel-31">panel 3.1</div>
                            <div id="panel-32">panel 3.2</div>
                            <div id="panel-33">panel 3.3</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TestimonialSectionGrid() -> impl IntoView {
    view! {
        <section id="testimonies" class="py-20 bg-slate-900">
            <div class="mx-8 max-w-6xl md:mx-10 lg:mx-20 xl:mx-auto">

                <div class="opacity-100 transition duration-500 ease-in-out transform scale-100 translate-x-0 translate-y-0">
                    <div class="mb-12 space-y-5 md:mb-16 md:text-center">
                        <div class="inline-block py-1 px-3 text-sm font-semibold text-indigo-100 bg-opacity-60 rounded-lg md:text-center hover:bg-opacity-40 hover:cursor-pointer text-cn bg-[#202c47]">
                            Words from Others
                        </div>
                        <h1 class="mb-5 text-3xl font-semibold text-white md:text-5xl md:text-center">
                            It is not just us.
                        </h1>
                        <p class="text-xl text-gray-100 md:text-2xl md:text-center">
                            Here is what others have to say about us.
                        </p>
                    </div>
                </div>

                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 lg:gap-8">

                    <ul class="space-y-8">
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/kanyewest" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1276461929934942210/cqNhNk6v_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Kanye West"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Kanye West</h3>
                                                <p class="text-gray-500 text-md">
                                                    Rapper &amp; Entrepreneur
                                                </p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Find God.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/tim_cook" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1535420431766671360/Pwq-1eJc_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Tim Cook"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Tim Cook</h3>
                                                <p class="text-gray-500 text-md">CEO of Apple</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Diam quis enim lobortis scelerisque
                                            fermentum dui faucibus in ornare. Donec pretium vulputate sapien nec sagittis
                                            aliquam malesuada bibendum.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/kanyewest" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1276461929934942210/cqNhNk6v_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Kanye West"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Kanye West</h3>
                                                <p class="text-gray-500 text-md">
                                                    Rapper &amp; Entrepreneur
                                                </p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Find God.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/tim_cook" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1535420431766671360/Pwq-1eJc_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Tim Cook"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Tim Cook</h3>
                                                <p class="text-gray-500 text-md">CEO of Apple</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Diam quis enim lobortis scelerisque
                                            fermentum dui faucibus in ornare. Donec pretium vulputate sapien nec sagittis
                                            aliquam malesuada bibendum.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                    </ul>

                    <ul class="hidden space-y-8 sm:block">
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/paraga" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1375285353146327052/y6jeByyD_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Parag Agrawal"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Parag Agrawal
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of Twitter</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Enim neque volutpat ac tincidunt vitae
                                            semper. Mattis aliquam faucibus purus in massa tempor. Neque vitae tempus quam
                                            pellentesque nec. Turpis cursus in hac habitasse platea dictumst.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/tim_cook" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1535420431766671360/Pwq-1eJc_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Tim Cook"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Tim Cook</h3>
                                                <p class="text-gray-500 text-md">CEO of Apple</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Diam quis enim lobortis scelerisque
                                            fermentum dui faucibus in ornare. Donec pretium vulputate sapien nec sagittis
                                            aliquam malesuada bibendum.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/paraga" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1375285353146327052/y6jeByyD_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Parag Agrawal"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Parag Agrawal
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of Twitter</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Enim neque volutpat ac tincidunt vitae
                                            semper. Mattis aliquam faucibus purus in massa tempor. Neque vitae tempus quam
                                            pellentesque nec. Turpis cursus in hac habitasse platea dictumst.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/tim_cook" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1535420431766671360/Pwq-1eJc_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Tim Cook"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">Tim Cook</h3>
                                                <p class="text-gray-500 text-md">CEO of Apple</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Diam quis enim lobortis scelerisque
                                            fermentum dui faucibus in ornare. Donec pretium vulputate sapien nec sagittis
                                            aliquam malesuada bibendum.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                    </ul>

                    <ul class="hidden space-y-8 lg:block">
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/satyanadella" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1221837516816306177/_Ld4un5A_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Satya Nadella"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Satya Nadella
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of Microsoft</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Tortor dignissim convallis aenean et
                                            tortor at. At ultrices mi tempus imperdiet nulla malesuada. Id cursus metus aliquam
                                            eleifend mi. Quis ipsum suspendisse ultrices gravida dictum fusce ut.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/dan_schulman" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/516916920482672641/3jCeLgFb_400x400.jpeg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Dan Schulman"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Dan Schulman
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of PayPal</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Quam pellentesque nec nam aliquam sem
                                            et tortor consequat id. Enim sit amet venenatis urna cursus.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/satyanadella" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/1221837516816306177/_Ld4un5A_400x400.jpg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Satya Nadella"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Satya Nadella
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of Microsoft</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Tortor dignissim convallis aenean et
                                            tortor at. At ultrices mi tempus imperdiet nulla malesuada. Id cursus metus aliquam
                                            eleifend mi. Quis ipsum suspendisse ultrices gravida dictum fusce ut.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                        <li class="text-sm leading-6">
                            <div class="relative group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg opacity-25 transition group-hover:opacity-100 group-hover:duration-200 blur duration-400"></div>
                                <a href="https://twitter.com/dan_schulman" class="cursor-pointer">
                                    <div class="relative p-6 space-y-6 leading-none rounded-lg ring-1 bg-slate-800 ring-gray-900/5">
                                        <div class="flex items-center space-x-4">
                                            <img
                                                src="https://pbs.twimg.com/profile_images/516916920482672641/3jCeLgFb_400x400.jpeg"
                                                class="w-12 h-12 bg-center bg-cover rounded-full border"
                                                alt="Dan Schulman"
                                            />
                                            <div>
                                                <h3 class="text-lg font-semibold text-white">
                                                    Dan Schulman
                                                </h3>
                                                <p class="text-gray-500 text-md">CEO of PayPal</p>
                                            </div>
                                        </div>
                                        <p class="leading-normal text-gray-300 text-md">
                                            Quam pellentesque nec nam aliquam sem
                                            et tortor consequat id. Enim sit amet venenatis urna cursus.
                                        </p>
                                    </div>
                                </a>
                            </div>
                        </li>
                    </ul>

                </div>
            </div>
        </section>
    }
}

#[component]
pub fn AnimatedCardsTailwind() -> impl IntoView {
    view! {
        <section class="overflow-hidden bg-zinc-50">
            <div class="flex flex-col justify-center py-12 px-8 mx-auto space-y-24 max-w-screen-xl md:px-12 lg:py-24 h-svh 2xl:max-w-screen-3xl">
                <div class="flex flex-col mx-auto sm:flex-row">
                    <a href="#_">
                        <img
                            src="https://images.unsplash.com/photo-1530035415911-95194de4ebcc?q=80&amp;w=2670&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            class="object-cover w-full h-full rounded-xl duration-500 transform origin-bottom rotate-6 hover:scale-150 hover:rotate-0 hover:-translate-y-12"
                            alt="#_"
                        />
                    </a>
                    <a href="#_">
                        <img
                            src="https://images.unsplash.com/photo-1487180144351-b8472da7d491?q=80&amp;w=2672&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D "
                            class="object-cover w-full h-full rounded-xl duration-500 transform origin-bottom -rotate-12 hover:scale-150 hover:rotate-0 hover:-translate-y-12"
                            alt="#_"
                        />
                    </a>
                    <a href="#_">
                        <img
                            src="https://images.unsplash.com/photo-1586996292898-71f4036c4e07?q=80&amp;w=2670&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            class="object-cover w-full h-full rounded-xl duration-500 transform origin-bottom rotate-6 hover:scale-150 hover:rotate-0 hover:-translate-y-12"
                            alt="#_"
                        />
                    </a>
                    <a href="#_">
                        <img
                            src="https://images.unsplash.com/photo-1522775417749-29284fb89f43?q=80&amp;w=2574&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            class="object-cover w-full h-full rounded-xl duration-500 transform origin-bottom -rotate-12 hover:scale-150 hover:rotate-0 hover:-translate-y-12"
                            alt="#_"
                        />
                    </a>

                </div>
            </div>
        </section>
    }
}

#[component]
pub fn CarouselWithFlowbiteScript() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-2xl">

            <div
                id="default-carousel"
                class="overflow-hidden relative rounded-lg shadow-lg"
                data-carousel="static"
            >

                <div class="relative h-80 md:h-96" data-carousel-inner>

                    <div class="hidden duration-700 ease-in-out" data-carousel-item>
                        <img
                            src="https://flowbite.com/docs/images/carousel/carousel-1.svg"
                            class="object-cover w-full h-full"
                            alt="Slide 1"
                        />
                        <span class="absolute top-1/2 left-1/2 text-xl font-semibold text-white transform -translate-x-1/2 -translate-y-1/2 md:text-2xl dark:text-gray-800">
                            First Slide
                        </span>
                    </div>

                    <div class="hidden duration-700 ease-in-out" data-carousel-item>
                        <img
                            src="https://flowbite.com/docs/images/carousel/carousel-2.svg"
                            class="object-cover w-full h-full"
                            alt="Slide 2"
                        />
                    </div>

                    <div class="hidden duration-700 ease-in-out" data-carousel-item>
                        <img
                            src="https://flowbite.com/docs/images/carousel/carousel-3.svg"
                            class="object-cover w-full h-full"
                            alt="Slide 3"
                        />
                    </div>
                </div>

                <div
                    class="flex absolute bottom-5 left-1/2 z-30 space-x-2 -translate-x-1/2"
                    data-carousel-indicators
                >
                    <button
                        type="button"
                        class="w-3 h-3 bg-gray-300 rounded-full transition hover:bg-gray-400 focus:bg-gray-400 focus:outline-none"
                    ></button>
                    <button
                        type="button"
                        class="w-3 h-3 bg-gray-300 rounded-full transition hover:bg-gray-400 focus:bg-gray-400 focus:outline-none"
                    ></button>
                    <button
                        type="button"
                        class="w-3 h-3 bg-gray-300 rounded-full transition hover:bg-gray-400 focus:bg-gray-400 focus:outline-none"
                    ></button>
                </div>

                <button
                    type="button"
                    class="flex absolute left-3 top-1/2 z-40 justify-center items-center w-10 h-10 rounded-full transition hover:bg-gray-300 focus:outline-none bg-gray-200/50"
                    data-carousel-prev
                >
                    <svg
                        class="w-5 h-5 text-gray-600"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15 19l-7-7 7-7"
                        ></path>
                    </svg>
                </button>
                <button
                    type="button"
                    class="flex absolute right-3 top-1/2 z-40 justify-center items-center w-10 h-10 rounded-full transition hover:bg-gray-300 focus:outline-none bg-gray-200/50"
                    data-carousel-next
                >
                    <svg
                        class="w-5 h-5 text-gray-600"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 5l7 7-7 7"
                        ></path>
                    </svg>
                </button>
            </div>

            <p class="mt-5 text-center text-gray-700 dark:text-gray-300">
                This carousel slider component is part of a larger, open-source library of Tailwind CSS components. Learn more
                by going to the official
                <a
                    class="text-blue-600 hover:underline"
                    href="https://flowbite.com/docs/getting-started/introduction/"
                    target="_blank"
                >
                    Flowbite Documentation
                </a>.
            </p>
            <script src="https://unpkg.com/flowbite@1.4.0/dist/flowbite.js"></script>

        </div>
    }
}

#[component]
pub fn FooterCool() -> impl IntoView {
    view! {
        <footer class="bg-[#1A1D2B]">
            <div class="container p-0 mx-auto md:p-8 xl:px-0">
                <div class="px-6 pt-16 pb-10 mx-auto max-w-7xl">
                    <div class="xl:grid xl:grid-cols-3 xl:gap-8">
                        <div class="space-y-4">
                            <div>
                                <a href="/">
                                    <div class="flex items-center space-x-2 text-2xl font-medium">
                                        <span>
                                            <img
                                                src="https://www.svgrepo.com/show/452102/slack.svg"
                                                alt="AI Logo"
                                                width="64"
                                                height="64"
                                                class="w-16"
                                            />
                                        </span>
                                        <span class="text-white">AIOps</span>
                                    </div>

                                </a>
                            </div>
                            <div class="pr-16 max-w-md text-gray-200 text-md">
                                Enhance productivity and
                                efficiency with cutting-edge artificial intelligence solutions for your business operations.
                            </div>
                            <div class="flex space-x-2">
                                <a
                                    href=""
                                    target="_blank"
                                    class="text-gray-200 hover:text-gray-200"
                                >
                                    <span class="sr-only">Linkedin</span>
                                    <svg
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                        class="w-6 h-6"
                                        aria-hidden="true"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M16.338 16.338H13.67V12.16c0-.995-.017-2.277-1.387-2.277-1.39 0-1.601 1.086-1.601 2.207v4.248H8.014v-8.59h2.559v1.174h.037c.356-.675 1.227-1.387 2.526-1.387 2.703 0 3.203 1.778 3.203 4.092v4.711zM5.005 6.575a1.548 1.548 0 11-.003-3.096 1.548 1.548 0 01.003 3.096zm-1.337 9.763H6.34v-8.59H3.667v8.59zM17.668 1H2.328C1.595 1 1 1.581 1 2.298v15.403C1 18.418 1.595 19 2.328 19h15.34c.734 0 1.332-.582 1.332-1.299V2.298C19 1.581 18.402 1 17.668 1z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </a>
                                <a
                                    href=""
                                    target="_blank"
                                    class="text-gray-200 hover:text-gray-200"
                                >
                                    <span class="sr-only">Twitter</span>
                                    <svg
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                        class="w-6 h-6"
                                        aria-hidden="true"
                                    >
                                        <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"></path>
                                    </svg>
                                </a>
                            </div>
                        </div>
                        <div class="grid grid-cols-2 gap-8 mt-16 xl:col-span-2 xl:mt-0">
                            <div class="md:grid md:grid-cols-2 md:gap-8">
                                <div>
                                    <h3 class="font-semibold leading-6 text-white text-md">
                                        Our Solutions
                                    </h3>
                                    <ul role="list" class="mt-6 space-y-4">
                                        <li>
                                            <a
                                                href="/aiplatform"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                AI Platform
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/aialgorithms"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                AI Algorithms
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/industryapplications"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Industry
                                                Applications
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                                <div class="mt-10 md:mt-0">
                                    <h3 class="font-semibold leading-6 text-white text-md">
                                        Use Cases
                                    </h3>
                                    <ul role="list" class="mt-6 space-y-4">
                                        <li>
                                            <a
                                                href="/predictiveanalysis"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Predictive
                                                Analysis
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/customerexperience"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Customer
                                                Experience
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/automation"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Automation
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                            <div class="md:grid md:grid-cols-2 md:gap-8">
                                <div>
                                    <h3 class="font-semibold leading-6 text-white text-md">
                                        Resources
                                    </h3>
                                    <ul role="list" class="mt-6 space-y-4">
                                        <li>
                                            <a
                                                href="/pricing"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Pricing
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/blog"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Blog
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/casestudies"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Case Studies
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/terms"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Terms
                                                of Service
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/privacy"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Privacy Policy
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                                <div class="mt-10 md:mt-0">
                                    <h3 class="font-semibold leading-6 text-white text-md">
                                        Company
                                    </h3>
                                    <ul role="list" class="mt-6 space-y-4">
                                        <li>
                                            <a
                                                href="/aboutus"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                About Us
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/careers"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Careers
                                            </a>
                                        </li>
                                        <li>
                                            <a
                                                href="/contactus"
                                                class="leading-6 text-gray-300 hover:text-gray-50 text-md"
                                            >
                                                Contact Us
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="pt-8 mt-16 border-t sm:mt-20 lg:mt-24 border-gray-400/30">
                        <div class="text-center text-white text-md">
                            Copyright  2024 . Crafted with <span class="text-gray-50">"♥"</span>
                            by AI enthusiasts at <a rel="noopener" href="/">
                                AIOps.
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn DropdownMenuList() -> impl IntoView {
    view! {
        <ul class="flex flex-col gap-2 mx-auto mt-24 max-w-[280px]">

            <li>
                <details class="group">

                    <summary class="flex gap-2 justify-between items-center p-2 font-medium hover:cursor-pointer marker:content-none">

                        <span class="flex gap-2">

                            <img
                                class="w-6 h-6 rounded-lg"
                                src="https://lh3.googleusercontent.com/a/AGNmyxbSlMgTRzE3_SMIxpDAhpNad-_CN5_tmph1NQ1KhA=s96-c"
                                alt=""
                            />

                            <span>Prajwal Hallale</span>
                        </span>
                        <svg
                            class="w-5 h-5 text-gray-500 transition group-open:rotate-90"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            viewBox="0 0 16 16"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z"
                            ></path>
                        </svg>
                    </summary>

                    <article class="px-4 pb-4">

                        <ul class="flex flex-col gap-4 pl-2 mt-4">

                            <li class="flex gap-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z"
                                    ></path>
                                </svg>

                                <a href="http://127.0.0.1:8000/user/dashboard">Dashboard</a>
                            </li>

                            <li class="flex gap-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0111.186 0z"
                                    ></path>
                                </svg>

                                <a href="http://127.0.0.1:8000/user/study-lists">Study Lists</a>
                            </li>

                            <li class="flex gap-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m6.75 12l-3-3m0 0l-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
                                    ></path>
                                </svg>

                                <a href="http://127.0.0.1:8000/user/contribution">
                                    Your contribution
                                </a>
                            </li>

                            <li class="flex gap-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z"
                                    ></path>
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    ></path>
                                </svg>

                                <a href="http://127.0.0.1:8000/user/settings">Settings</a>
                            </li>

                            <form action="http://127.0.0.1:8000/auth/logout" method="POST">
                                <input
                                    type="hidden"
                                    name="_token"
                                    value="ymEkCLBFpgkdaSbidUArRsdHbER5DkT6ByS3eJYb"
                                />
                                <button
                                    type="submit"
                                    class="py-1 px-2 text-sm text-red-500 rounded-md hover:bg-red-200"
                                >
                                    Log Out
                                </button>
                            </form>

                        </ul>

                    </article>

                </details>
            </li>

            <li>
                <details class="group">

                    <summary class="flex gap-2 justify-between items-center p-2 font-medium hover:cursor-pointer marker:content-none">

                        <span class="flex gap-2">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"
                                ></path>
                            </svg>

                            <span>Recent Documents</span>
                        </span>
                        <svg
                            class="w-5 h-5 text-gray-500 transition group-open:rotate-90"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            viewBox="0 0 16 16"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z"
                            ></path>
                        </svg>
                    </summary>

                    <article class="px-4 pb-4">
                        <ul class="flex flex-col gap-1 pl-2">
                            <li>
                                <a href="">Document title</a>
                            </li>
                            <li>
                                <a href="">Document title</a>
                            </li>
                            <li>
                                <a href="">Document title</a>
                            </li>
                        </ul>
                    </article>

                </details>
            </li>

            <li>
                <details class="group">

                    <summary class="flex gap-2 justify-between items-center p-2 font-medium hover:cursor-pointer marker:content-none">

                        <span class="flex gap-2">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
                                />
                            </svg>
                            <span>Popular courses</span>
                        </span>
                        <svg
                            class="w-5 h-5 text-gray-500 transition group-open:rotate-90"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            viewBox="0 0 16 16"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z"
                            ></path>
                        </svg>

                    </summary>

                    <article class="px-4 pb-4">
                        <ul class="flex flex-col gap-1 pl-2">
                            <li>
                                <a href="">Course title</a>
                            </li>
                            <li>
                                <a href="">Course title</a>
                            </li>
                            <li>
                                <a href="">Course title</a>
                            </li>
                        </ul>
                    </article>

                </details>
            </li>

        </ul>
    }
}

#[component]
pub fn FooterNumberTwo() -> impl IntoView {
    view! {
        <footer class="w-full bg-white">
            <div class="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">

                <div class="grid grid-cols-2 gap-3 gap-y-8 py-10 sm:grid-cols-4 md:gap-8 lg:grid-cols-6 max-sm:max-w-sm max-sm:mx-auto">
                    <div class="col-span-full mb-10 lg:col-span-2 lg:mb-0">
                        <a href="https://pagedone.io/" class="flex justify-center lg:justify-start">
                            <svg
                                class="w-40 h-8"
                                viewBox="0 0 164 33"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M47 24.7231V7H54.4171C54.5916 7 54.816 7.00821 55.0903 7.02462C55.3645 7.03282 55.618 7.05744 55.8507 7.09846C56.8895 7.25436 57.7455 7.59487 58.4186 8.12C59.1001 8.64513 59.6029 9.30974 59.927 10.1138C60.2594 10.9097 60.4256 11.7959 60.4256 12.7723C60.4256 13.7405 60.2594 14.6267 59.927 15.4308C59.5945 16.2267 59.0876 16.8872 58.4061 17.4123C57.733 17.9374 56.8812 18.2779 55.8507 18.4338C55.618 18.4667 55.3604 18.4913 55.0778 18.5077C54.8035 18.5241 54.5833 18.5323 54.4171 18.5323H50.0042V24.7231H47ZM50.0042 15.7631H54.2925C54.4587 15.7631 54.6457 15.7549 54.8534 15.7385C55.0612 15.7221 55.2523 15.6892 55.4268 15.64C55.9255 15.5169 56.3161 15.2995 56.5986 14.9877C56.8895 14.6759 57.0931 14.3231 57.2094 13.9292C57.3341 13.5354 57.3964 13.1497 57.3964 12.7723C57.3964 12.3949 57.3341 12.0092 57.2094 11.6154C57.0931 11.2133 56.8895 10.8564 56.5986 10.5446C56.3161 10.2328 55.9255 10.0154 55.4268 9.89231C55.2523 9.84308 55.0612 9.81436 54.8534 9.80615C54.6457 9.78974 54.4587 9.78154 54.2925 9.78154H50.0042V15.7631Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M66.0975 25.0923C65.1252 25.0923 64.3024 24.9118 63.6293 24.5508C62.9561 24.1815 62.445 23.6933 62.096 23.0862C61.7553 22.479 61.5849 21.8103 61.5849 21.08C61.5849 20.44 61.6929 19.8656 61.909 19.3569C62.1251 18.84 62.4575 18.3969 62.9063 18.0277C63.355 17.6503 63.9368 17.3426 64.6515 17.1046C65.1917 16.9323 65.8233 16.7764 66.5463 16.6369C67.2776 16.4974 68.0671 16.3703 68.9148 16.2554C69.7707 16.1323 70.6641 16.001 71.5949 15.8615L70.5228 16.4646C70.5311 15.5456 70.3234 14.8687 69.8995 14.4338C69.4757 13.999 68.761 13.7815 67.7554 13.7815C67.1488 13.7815 66.5629 13.921 65.9978 14.2C65.4327 14.479 65.0379 14.959 64.8135 15.64L62.0711 14.7908C62.4035 13.6667 63.0351 12.7641 63.9659 12.0831C64.9049 11.4021 66.1681 11.0615 67.7554 11.0615C68.9522 11.0615 70.0034 11.2544 70.9093 11.64C71.8234 12.0256 72.5007 12.6574 72.9412 13.5354C73.1822 14.0031 73.3276 14.4831 73.3775 14.9754C73.4274 15.4595 73.4523 15.9887 73.4523 16.5631V24.7231H70.822V21.8431L71.2583 22.3108C70.6517 23.2708 69.9411 23.9764 69.1267 24.4277C68.3206 24.8708 67.3108 25.0923 66.0975 25.0923ZM66.6959 22.7292C67.3773 22.7292 67.9591 22.6103 68.4411 22.3723C68.9231 22.1344 69.3054 21.8431 69.5879 21.4985C69.8788 21.1538 70.0741 20.8297 70.1738 20.5262C70.3317 20.1487 70.419 19.7179 70.4356 19.2338C70.4605 18.7415 70.473 18.3436 70.473 18.04L71.3954 18.3108C70.4896 18.4503 69.7126 18.5733 69.0643 18.68C68.4161 18.7867 67.8593 18.8892 67.3939 18.9877C66.9286 19.0779 66.5172 19.1805 66.1598 19.2954C65.8108 19.4185 65.5158 19.5621 65.2748 19.7262C65.0338 19.8903 64.8468 20.079 64.7138 20.2923C64.5891 20.5056 64.5268 20.7559 64.5268 21.0431C64.5268 21.3713 64.6099 21.6626 64.7761 21.9169C64.9423 22.1631 65.1833 22.36 65.4991 22.5077C65.8233 22.6554 66.2222 22.7292 66.6959 22.7292Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M82.1078 31C81.3598 31 80.641 30.8851 79.9512 30.6554C79.2698 30.4256 78.6548 30.0933 78.1063 29.6585C77.5578 29.2318 77.109 28.7149 76.76 28.1077L79.5274 26.7538C79.785 27.2379 80.1465 27.5949 80.6119 27.8246C81.0856 28.0626 81.5884 28.1815 82.1203 28.1815C82.7435 28.1815 83.3003 28.0708 83.7907 27.8492C84.281 27.6359 84.6591 27.3159 84.925 26.8892C85.1993 26.4708 85.3281 25.9456 85.3115 25.3138V21.5354H85.6855V11.4308H88.3157V25.3631C88.3157 25.6995 88.2991 26.0195 88.2659 26.3231C88.2409 26.6349 88.1952 26.9385 88.1287 27.2338C87.9293 28.0954 87.547 28.801 86.9819 29.3508C86.4168 29.9087 85.7145 30.3231 84.8752 30.5938C84.0441 30.8646 83.1217 31 82.1078 31ZM81.846 25.0923C80.6077 25.0923 79.5274 24.7846 78.6049 24.1692C77.6825 23.5538 76.9678 22.7169 76.4608 21.6585C75.9539 20.6 75.7004 19.4062 75.7004 18.0769C75.7004 16.7313 75.9539 15.5333 76.4608 14.4831C76.9761 13.4246 77.7032 12.5918 78.6423 11.9846C79.5814 11.3692 80.6867 11.0615 81.9582 11.0615C83.238 11.0615 84.3101 11.3692 85.1744 11.9846C86.047 12.5918 86.7076 13.4246 87.1564 14.4831C87.6052 15.5415 87.8296 16.7395 87.8296 18.0769C87.8296 19.3979 87.6052 20.5918 87.1564 21.6585C86.7076 22.7169 86.0387 23.5538 85.1494 24.1692C84.2602 24.7846 83.1591 25.0923 81.846 25.0923ZM82.3072 22.4338C83.1134 22.4338 83.7616 22.2533 84.2519 21.8923C84.7505 21.5231 85.112 21.0103 85.3364 20.3538C85.5691 19.6974 85.6855 18.9385 85.6855 18.0769C85.6855 17.2072 85.5691 16.4482 85.3364 15.8C85.112 15.1436 84.7588 14.6349 84.2768 14.2738C83.7948 13.9046 83.1715 13.72 82.407 13.72C81.6008 13.72 80.936 13.9169 80.4124 14.3108C79.8889 14.6964 79.5024 15.2215 79.2531 15.8862C79.0038 16.5426 78.8792 17.2728 78.8792 18.0769C78.8792 18.8892 78.9997 19.6277 79.2407 20.2923C79.49 20.9487 79.8681 21.4697 80.375 21.8554C80.882 22.241 81.5261 22.4338 82.3072 22.4338Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M97.6827 25.0923C96.3198 25.0923 95.1231 24.801 94.0926 24.2185C93.0621 23.6359 92.256 22.8277 91.6743 21.7938C91.1008 20.76 90.8141 19.5703 90.8141 18.2246C90.8141 16.7723 91.0967 15.5128 91.6618 14.4462C92.2269 13.3713 93.0122 12.5385 94.0178 11.9477C95.0234 11.3569 96.1869 11.0615 97.5082 11.0615C98.9044 11.0615 100.089 11.3856 101.061 12.0338C102.042 12.6738 102.769 13.5805 103.242 14.7538C103.716 15.9272 103.895 17.3097 103.778 18.9015H100.799V17.8185C100.791 16.3744 100.533 15.32 100.026 14.6554C99.5194 13.9908 98.7216 13.6585 97.6329 13.6585C96.4029 13.6585 95.4888 14.0359 94.8904 14.7908C94.2921 15.5374 93.9929 16.6328 93.9929 18.0769C93.9929 19.4226 94.2921 20.4646 94.8904 21.2031C95.4888 21.9415 96.3614 22.3108 97.5082 22.3108C98.2479 22.3108 98.8836 22.1508 99.4155 21.8308C99.9557 21.5026 100.371 21.0308 100.662 20.4154L103.629 21.3015C103.114 22.4995 102.316 23.4308 101.235 24.0954C100.163 24.76 98.9792 25.0923 97.6827 25.0923ZM93.0455 18.9015V16.6615H102.308V18.9015H93.0455Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M111.708 25.0923C110.47 25.0923 109.39 24.7846 108.467 24.1692C107.545 23.5538 106.83 22.7169 106.323 21.6585C105.816 20.6 105.563 19.4062 105.563 18.0769C105.563 16.7313 105.816 15.5333 106.323 14.4831C106.838 13.4246 107.565 12.5918 108.505 11.9846C109.444 11.3692 110.549 11.0615 111.82 11.0615C113.1 11.0615 114.172 11.3692 115.037 11.9846C115.909 12.5918 116.57 13.4246 117.019 14.4831C117.467 15.5415 117.692 16.7395 117.692 18.0769C117.692 19.3979 117.467 20.5918 117.019 21.6585C116.57 22.7169 115.901 23.5538 115.012 24.1692C114.122 24.7846 113.021 25.0923 111.708 25.0923ZM112.169 22.4338C112.976 22.4338 113.624 22.2533 114.114 21.8923C114.613 21.5231 114.974 21.0103 115.199 20.3538C115.431 19.6974 115.548 18.9385 115.548 18.0769C115.548 17.2072 115.431 16.4482 115.199 15.8C114.974 15.1436 114.621 14.6349 114.139 14.2738C113.657 13.9046 113.034 13.72 112.269 13.72C111.463 13.72 110.798 13.9169 110.275 14.3108C109.751 14.6964 109.365 15.2215 109.115 15.8862C108.866 16.5426 108.741 17.2728 108.741 18.0769C108.741 18.8892 108.862 19.6277 109.103 20.2923C109.352 20.9487 109.73 21.4697 110.237 21.8554C110.744 22.241 111.388 22.4338 112.169 22.4338ZM115.548 24.7231V15.3938H115.174V7H118.203V24.7231H115.548Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M127.395 25.0923C126.049 25.0923 124.873 24.7928 123.867 24.1938C122.861 23.5949 122.08 22.7703 121.523 21.72C120.975 20.6615 120.701 19.4472 120.701 18.0769C120.701 16.6821 120.983 15.4595 121.548 14.4092C122.113 13.359 122.899 12.5385 123.904 11.9477C124.91 11.3569 126.073 11.0615 127.395 11.0615C128.749 11.0615 129.93 11.361 130.935 11.96C131.941 12.559 132.722 13.3877 133.279 14.4462C133.835 15.4964 134.114 16.7067 134.114 18.0769C134.114 19.4554 133.831 20.6738 133.266 21.7323C132.709 22.7826 131.928 23.6072 130.923 24.2062C129.917 24.7969 128.741 25.0923 127.395 25.0923ZM127.395 22.3108C128.592 22.3108 129.481 21.9169 130.062 21.1292C130.644 20.3415 130.935 19.3241 130.935 18.0769C130.935 16.7887 130.64 15.7631 130.05 15C129.46 14.2287 128.575 13.8431 127.395 13.8431C126.589 13.8431 125.924 14.0236 125.4 14.3846C124.885 14.7374 124.503 15.2338 124.253 15.8738C124.004 16.5056 123.879 17.24 123.879 18.0769C123.879 19.3651 124.174 20.3949 124.765 21.1662C125.363 21.9292 126.24 22.3108 127.395 22.3108Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M145.923 24.7231V18.3231C145.923 17.9046 145.894 17.441 145.836 16.9323C145.778 16.4236 145.64 15.9354 145.424 15.4677C145.217 14.9918 144.901 14.6021 144.477 14.2985C144.061 13.9949 143.496 13.8431 142.782 13.8431C142.399 13.8431 142.021 13.9046 141.647 14.0277C141.273 14.1508 140.933 14.3641 140.625 14.6677C140.326 14.9631 140.085 15.3733 139.902 15.8985C139.719 16.4154 139.628 17.08 139.628 17.8923L137.845 17.1415C137.845 16.0092 138.065 14.9836 138.506 14.0646C138.955 13.1456 139.611 12.4154 140.475 11.8738C141.34 11.3241 142.403 11.0492 143.667 11.0492C144.664 11.0492 145.487 11.2133 146.135 11.5415C146.783 11.8697 147.298 12.2882 147.681 12.7969C148.063 13.3056 148.345 13.8472 148.528 14.4215C148.711 14.9959 148.827 15.5415 148.877 16.0585C148.936 16.5672 148.965 16.9815 148.965 17.3015V24.7231H145.923ZM136.586 24.7231V11.4308H139.266V15.5538H139.628V24.7231H136.586Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M157.87 25.0923C156.507 25.0923 155.31 24.801 154.28 24.2185C153.249 23.6359 152.443 22.8277 151.861 21.7938C151.288 20.76 151.001 19.5703 151.001 18.2246C151.001 16.7723 151.284 15.5128 151.849 14.4462C152.414 13.3713 153.199 12.5385 154.205 11.9477C155.21 11.3569 156.374 11.0615 157.695 11.0615C159.091 11.0615 160.276 11.3856 161.248 12.0338C162.229 12.6738 162.956 13.5805 163.43 14.7538C163.903 15.9272 164.082 17.3097 163.966 18.9015H160.986V17.8185C160.978 16.3744 160.72 15.32 160.213 14.6554C159.706 13.9908 158.909 13.6585 157.82 13.6585C156.59 13.6585 155.676 14.0359 155.078 14.7908C154.479 15.5374 154.18 16.6328 154.18 18.0769C154.18 19.4226 154.479 20.4646 155.078 21.2031C155.676 21.9415 156.548 22.3108 157.695 22.3108C158.435 22.3108 159.071 22.1508 159.603 21.8308C160.143 21.5026 160.558 21.0308 160.849 20.4154L163.816 21.3015C163.301 22.4995 162.503 23.4308 161.423 24.0954C160.351 24.76 159.166 25.0923 157.87 25.0923ZM153.233 18.9015V16.6615H162.495V18.9015H153.233Z"
                                    fill="#111827"
                                />
                                <path
                                    d="M24.5473 11.8941C25.1905 12.5063 25.2068 13.5149 24.5837 14.1468L18.7585 20.054C18.1354 20.686 17.1087 20.702 16.4654 20.0898C15.8222 19.4776 15.8059 18.469 16.429 17.8371L22.2542 11.9299C22.8773 11.2979 23.904 11.2819 24.5473 11.8941Z"
                                    fill="url(#paint0_linear_9129_4680)"
                                />
                                <path
                                    fill-rule="evenodd"
                                    clip-rule="evenodd"
                                    d="M0 4.54673C0 2.03564 2.07211 0 4.62819 0H21.5399V0.00124069C28.9908 0.0998525 35 6.06429 35 13.4075C35 20.8123 28.8897 26.8151 21.3523 26.8151C18.6648 26.8151 16.1587 26.052 14.0463 24.7342L6.58815 31.9057C4.13431 34.2652 0 32.5573 0 29.1841V4.54673ZM11.5194 22.7055C9.15709 20.295 7.70452 17.0179 7.70452 13.4075C7.70452 12.5277 8.43056 11.8144 9.32619 11.8144C10.2218 11.8144 10.9479 12.5277 10.9479 13.4075C10.9479 19.0526 15.6061 23.6288 21.3523 23.6288C27.0985 23.6288 31.7567 19.0526 31.7567 13.4075C31.7567 7.76248 27.0985 3.18626 21.3523 3.18626H4.62819C3.86336 3.18626 3.24334 3.79536 3.24334 4.54673V29.1841C3.24334 29.7351 3.91866 30.014 4.31948 29.6286L11.5194 22.7055Z"
                                    fill="url(#paint1_linear_9129_4680)"
                                />
                                <defs>
                                    <linearGradient
                                        id="paint0_linear_9129_4680"
                                        x1="35"
                                        y1="1.89063"
                                        x2="1.11152"
                                        y2="33.4573"
                                        gradientUnits="userSpaceOnUse"
                                    >
                                        <stop stop-color="#7C3AED" />
                                        <stop offset="0.993738" stop-color="#4F46E5" />
                                    </linearGradient>
                                    <linearGradient
                                        id="paint1_linear_9129_4680"
                                        x1="35"
                                        y1="1.89063"
                                        x2="1.11152"
                                        y2="33.4573"
                                        gradientUnits="userSpaceOnUse"
                                    >
                                        <stop stop-color="#7C3AED" />
                                        <stop offset="0.993738" stop-color="#4F46E5" />
                                    </linearGradient>
                                </defs>
                            </svg>
                        </a>
                        <p class="py-8 text-sm text-center text-gray-500 lg:max-w-xs lg:text-left">
                            Trusted in more than 100 countries & 5 million customers. Have any query ?
                        </p>
                        <a
                            href="javascript:;"
                            class="block py-2.5 px-5 mx-auto h-9 text-xs text-white bg-indigo-600 rounded-full shadow-sm transition-all duration-500 lg:mx-0 hover:bg-indigo-700 w-fit"
                        >
                            Contact us
                        </a>
                    </div>

                    <div class="text-left lg:mx-auto">
                        <h4 class="mb-7 text-lg font-medium text-gray-900">Pagedone</h4>
                        <ul class="text-sm transition-all duration-500">
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Home
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    About
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Pricing
                                </a>
                            </li>
                            <li>
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Features
                                </a>
                            </li>
                        </ul>
                    </div>

                    <div class="text-left lg:mx-auto">
                        <h4 class="mb-7 text-lg font-medium text-gray-900">Products</h4>
                        <ul class="text-sm transition-all duration-500">
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Figma UI System
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Icons Assets
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Responsive Blocks
                                </a>
                            </li>
                            <li>
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Components Library
                                </a>
                            </li>
                        </ul>
                    </div>

                    <div class="text-left lg:mx-auto">
                        <h4 class="mb-7 text-lg font-medium text-gray-900">Resources</h4>
                        <ul class="text-sm transition-all duration-500">
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    FAQs
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Quick Start
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Documentation
                                </a>
                            </li>
                            <li>
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    User Guide
                                </a>
                            </li>
                        </ul>
                    </div>

                    <div class="text-left lg:mx-auto">
                        <h4 class="mb-7 text-lg font-medium text-gray-900">Blogs</h4>
                        <ul class="text-sm transition-all duration-500">
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    News
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Tips & Tricks
                                </a>
                            </li>
                            <li class="mb-6">
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    New Updates
                                </a>
                            </li>
                            <li>
                                <a href="javascript:;" class="text-gray-600 hover:text-gray-900">
                                    Events
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="py-7 border-t border-gray-200">
                    <div class="flex flex-col justify-center items-center lg:flex-row lg:justify-between">
                        <span class="text-sm text-gray-500">
                            "©"<a href="https://pagedone.io/">pagedone</a>
                            2024, All rights reserved.
                        </span>
                        <div class="flex mt-4 space-x-4 sm:justify-center lg:mt-0">
                            <a
                                href="javascript:;"
                                class="flex justify-center items-center w-9 h-9 bg-gray-700 rounded-full hover:bg-indigo-600"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="20"
                                    height="20"
                                    viewBox="0 0 20 20"
                                    fill="none"
                                >
                                    <g id="Social Media">
                                        <path
                                            id="Vector"
                                            d="M11.3214 8.93666L16.4919 3.05566H15.2667L10.7772 8.16205L7.1914 3.05566H3.05566L8.47803 10.7774L3.05566 16.9446H4.28097L9.022 11.552L12.8088 16.9446H16.9446L11.3211 8.93666H11.3214ZM9.64322 10.8455L9.09382 10.0765L4.72246 3.95821H6.60445L10.1322 8.8959L10.6816 9.66481L15.2672 16.083H13.3852L9.64322 10.8458V10.8455Z"
                                            fill="white"
                                        />
                                    </g>
                                </svg>
                            </a>
                            <a
                                href="javascript:;"
                                class="flex justify-center items-center w-9 h-9 bg-gray-700 rounded-full hover:bg-indigo-600"
                            >
                                <svg
                                    class="text-white w-[1.25rem] h-[1.125rem]"
                                    viewBox="0 0 15 15"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M4.70975 7.93663C4.70975 6.65824 5.76102 5.62163 7.0582 5.62163C8.35537 5.62163 9.40721 6.65824 9.40721 7.93663C9.40721 9.21502 8.35537 10.2516 7.0582 10.2516C5.76102 10.2516 4.70975 9.21502 4.70975 7.93663ZM3.43991 7.93663C3.43991 9.90608 5.05982 11.5025 7.0582 11.5025C9.05658 11.5025 10.6765 9.90608 10.6765 7.93663C10.6765 5.96719 9.05658 4.37074 7.0582 4.37074C5.05982 4.37074 3.43991 5.96719 3.43991 7.93663ZM9.97414 4.22935C9.97408 4.39417 10.0236 4.55531 10.1165 4.69239C10.2093 4.82946 10.3413 4.93633 10.4958 4.99946C10.6503 5.06259 10.8203 5.07916 10.9844 5.04707C11.1484 5.01498 11.2991 4.93568 11.4174 4.81918C11.5357 4.70268 11.6163 4.55423 11.649 4.39259C11.6817 4.23095 11.665 4.06339 11.6011 3.91109C11.5371 3.7588 11.4288 3.6286 11.2898 3.53698C11.1508 3.44536 10.9873 3.39642 10.8201 3.39635H10.8197C10.5955 3.39646 10.3806 3.48424 10.222 3.64043C10.0635 3.79661 9.97434 4.00843 9.97414 4.22935ZM4.21142 13.5892C3.52442 13.5584 3.15101 13.4456 2.90286 13.3504C2.57387 13.2241 2.33914 13.0738 2.09235 12.8309C1.84555 12.588 1.69278 12.3569 1.56527 12.0327C1.46854 11.7882 1.3541 11.4201 1.32287 10.7431C1.28871 10.0111 1.28189 9.79119 1.28189 7.93669C1.28189 6.08219 1.28927 5.86291 1.32287 5.1303C1.35416 4.45324 1.46944 4.08585 1.56527 3.84069C1.69335 3.51647 1.84589 3.28513 2.09235 3.04191C2.3388 2.79869 2.57331 2.64813 2.90286 2.52247C3.1509 2.42713 3.52442 2.31435 4.21142 2.28358C4.95417 2.24991 5.17729 2.24319 7.0582 2.24319C8.9391 2.24319 9.16244 2.25047 9.90582 2.28358C10.5928 2.31441 10.9656 2.42802 11.2144 2.52247C11.5434 2.64813 11.7781 2.79902 12.0249 3.04191C12.2717 3.2848 12.4239 3.51647 12.552 3.84069C12.6487 4.08513 12.7631 4.45324 12.7944 5.1303C12.8285 5.86291 12.8354 6.08219 12.8354 7.93669C12.8354 9.79119 12.8285 10.0105 12.7944 10.7431C12.7631 11.4201 12.6481 11.7881 12.552 12.0327C12.4239 12.3569 12.2714 12.5882 12.0249 12.8309C11.7784 13.0736 11.5434 13.2241 11.2144 13.3504C10.9663 13.4457 10.5928 13.5585 9.90582 13.5892C9.16306 13.6229 8.93994 13.6296 7.0582 13.6296C5.17645 13.6296 4.95395 13.6229 4.21142 13.5892ZM4.15307 1.03424C3.40294 1.06791 2.89035 1.18513 2.4427 1.3568C1.9791 1.53408 1.58663 1.77191 1.19446 2.1578C0.802277 2.54369 0.56157 2.93108 0.381687 3.38797C0.207498 3.82941 0.0885535 4.3343 0.0543922 5.07358C0.0196672 5.81402 0.0117188 6.05074 0.0117188 7.93663C0.0117188 9.82252 0.0196672 10.0592 0.0543922 10.7997C0.0885535 11.539 0.207498 12.0439 0.381687 12.4853C0.56157 12.9419 0.802334 13.3297 1.19446 13.7155C1.58658 14.1012 1.9791 14.3387 2.4427 14.5165C2.89119 14.6881 3.40294 14.8054 4.15307 14.839C4.90479 14.8727 5.1446 14.8811 7.0582 14.8811C8.9718 14.8811 9.212 14.8732 9.96332 14.839C10.7135 14.8054 11.2258 14.6881 11.6737 14.5165C12.137 14.3387 12.5298 14.1014 12.9219 13.7155C13.3141 13.3296 13.5543 12.9419 13.7347 12.4853C13.9089 12.0439 14.0284 11.539 14.062 10.7997C14.0962 10.0587 14.1041 9.82252 14.1041 7.93663C14.1041 6.05074 14.0962 5.81402 14.062 5.07358C14.0278 4.33424 13.9089 3.82913 13.7347 3.38797C13.5543 2.93135 13.3135 2.5443 12.9219 2.1578C12.5304 1.7713 12.137 1.53408 11.6743 1.3568C11.2258 1.18513 10.7135 1.06735 9.96388 1.03424C9.21256 1.00058 8.97236 0.992188 7.05876 0.992188C5.14516 0.992188 4.90479 1.00002 4.15307 1.03424Z"
                                        fill="currentColor"
                                    />
                                </svg>

                            </a>
                            <a
                                href="javascript:;"
                                class="flex justify-center items-center w-9 h-9 bg-gray-700 rounded-full hover:bg-indigo-600"
                            >
                                <svg
                                    class="text-white w-[1rem] h-[1rem]"
                                    viewBox="0 0 13 12"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M2.8794 11.5527V3.86835H0.318893V11.5527H2.87967H2.8794ZM1.59968 2.81936C2.4924 2.81936 3.04817 2.2293 3.04817 1.49188C3.03146 0.737661 2.4924 0.164062 1.61666 0.164062C0.74032 0.164062 0.167969 0.737661 0.167969 1.49181C0.167969 2.22923 0.723543 2.8193 1.5829 2.8193H1.59948L1.59968 2.81936ZM4.29668 11.5527H6.85698V7.26187C6.85698 7.03251 6.87369 6.80255 6.94134 6.63873C7.12635 6.17968 7.54764 5.70449 8.25514 5.70449C9.18141 5.70449 9.55217 6.4091 9.55217 7.44222V11.5527H12.1124V7.14672C12.1124 4.78652 10.8494 3.68819 9.16483 3.68819C7.78372 3.68819 7.17715 4.45822 6.84014 4.98267H6.85718V3.86862H4.29681C4.33023 4.5895 4.29661 11.553 4.29661 11.553L4.29668 11.5527Z"
                                        fill="currentColor"
                                    />
                                </svg>

                            </a>
                            <a
                                href="javascript:;"
                                class="flex justify-center items-center w-9 h-9 bg-gray-700 rounded-full hover:bg-indigo-600"
                            >
                                <svg
                                    class="text-white w-[1.25rem] h-[0.875rem]"
                                    viewBox="0 0 16 12"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        clip-rule="evenodd"
                                        d="M13.9346 1.13529C14.5684 1.30645 15.0665 1.80588 15.2349 2.43896C15.5413 3.58788 15.5413 5.98654 15.5413 5.98654C15.5413 5.98654 15.5413 8.3852 15.2349 9.53412C15.0642 10.1695 14.5661 10.669 13.9346 10.8378C12.7886 11.1449 8.19058 11.1449 8.19058 11.1449C8.19058 11.1449 3.59491 11.1449 2.44657 10.8378C1.81277 10.6666 1.31461 10.1672 1.14622 9.53412C0.839844 8.3852 0.839844 5.98654 0.839844 5.98654C0.839844 5.98654 0.839844 3.58788 1.14622 2.43896C1.31695 1.80353 1.81511 1.30411 2.44657 1.13529C3.59491 0.828125 8.19058 0.828125 8.19058 0.828125C8.19058 0.828125 12.7886 0.828125 13.9346 1.13529ZM10.541 5.98654L6.72178 8.19762V3.77545L10.541 5.98654Z"
                                        fill="currentColor"
                                    />
                                </svg>

                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn BlogImageHeaderWithBackgroundEffect() -> impl IntoView {
    view! {
        <div class="overflow-hidden relative group h-[500px] bg-primary card-hidden">
            <img
                src="https://images.unsplash.com/photo-1595239244990-b609da3d95f2?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwxfHxkYXRhJTNBaW1hZ2UlMkZibXAlM0JiYXNlNjQlMkNRazMyQkFBQUFBQUFBRFlBQUFBb0FBQUFDQUFBQUFnQUFBQUJBQmdBQUFBQUFNQUFBQUFUQ3dBQUV3c0FBQUFBQUFBQUFBQUFvWnRhbnBaTm9aTTR0S05KeTd0dzBjV0Z1clYlMkZpcEJqcDZSMW41dGtsSTQlMkZvWmxLdkxWNXlNT1R0TFNLaFl0Z3I2Mk1vYUY1aUl0Tmk1RlRxN0NJdnNLa3I3T1dnSVpkdGJPYXBhYUhobzVnaEpKbnBMS1l1Y1N5ckxTZ2ZvVmZ0N1dkcXF1T2s1bDBrcDklMkZyTHFudk1pN3JiYW1nSWRtdDdPWHNLMlBwYWFGckxHV3ZzYTB4YzYlMkZzYm1vaG8xdXRiQ1BzNiUyQk50YkdSd0wlMkJsenRDOHp0TEJ0cnlwaTVKMXRLJTJCTHRhJTJCTXVyV1Z5TVdxMU5TJTJGMGRUQnVMMm9qcFIzfGVufDB8MHx8fDE3MjM0MDU0MTR8MA&ixlib=rb-4.0.3&q=80&w=1080"
                class="object-center absolute w-full h-full transition-all ease-in delay-300 group-hover:scale-105 duration-400"
            />
            <div class="flex absolute bottom-0 z-50 flex-col gap-4 justify-end p-8 w-full h-full bg-opacity-45">
                <span class="text-white text-[20px] font-canela sm:text-[24px] md:text-[28px]">
                    Hover Card to see Magic
                </span>
                <p class="hidden text-white group-hover:block text-[14px]">

                    Lorem ipsum dolor sit amet consectetur adipisicing elit. Odio accusantium dolores, in cumque labore non assumenda necessitatibus vel at voluptates deleniti minus nesciunt nobis repellendus dolorum blanditiis ex magni impedit?

                </p>
                <a href="contactus.html" class="flex gap-2 items-center">

                    <div class="flex justify-center items-center max-h-10 rounded-full bg-primary min-h-10 min-w-10 max-w-10 group-hover:bg-secondary">
                        <img
                            src="https://images.unsplash.com/photo-1595239244990-b609da3d95f2?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwxfHxkYXRhJTNBaW1hZ2UlMkZibXAlM0JiYXNlNjQlMkNRazMyQkFBQUFBQUFBRFlBQUFBb0FBQUFDQUFBQUFnQUFBQUJBQmdBQUFBQUFNQUFBQUFUQ3dBQUV3c0FBQUFBQUFBQUFBQUFvWnRhbnBaTm9aTTR0S05KeTd0dzBjV0Z1clYlMkZpcEJqcDZSMW41dGtsSTQlMkZvWmxLdkxWNXlNT1R0TFNLaFl0Z3I2Mk1vYUY1aUl0Tmk1RlRxN0NJdnNLa3I3T1dnSVpkdGJPYXBhYUhobzVnaEpKbnBMS1l1Y1N5ckxTZ2ZvVmZ0N1dkcXF1T2s1bDBrcDklMkZyTHFudk1pN3JiYW1nSWRtdDdPWHNLMlBwYWFGckxHV3ZzYTB4YzYlMkZzYm1vaG8xdXRiQ1BzNiUyQk50YkdSd0wlMkJsenRDOHp0TEJ0cnlwaTVKMXRLJTJCTHRhJTJCTXVyV1Z5TVdxMU5TJTJGMGRUQnVMMm9qcFIzfGVufDB8MHx8fDE3MjM0MDU0MTR8MA&ixlib=rb-4.0.3&q=80&w=1080"
                            class="overflow-hidden h-8 rounded-full w-[28px]"
                            alt=""
                        />
                    </div>
                    <h2 class="font-bold text-white text-[16px]">Contact Us</h2>
                </a>
            </div>
            <div class="absolute bottom-0 z-30 w-full text-white bg-gradient-to-b from-transparent to-black transition-all ease-in group-hover:bottom-0 duration-400 min-h-[650px] group-hover:min-h-[900px]"></div>
        </div>
    }
}

#[component]
pub fn StickySections() -> impl IntoView {
    view! {
        <div class="relative">
            <div class="flex sticky top-0 flex-col justify-center items-center h-screen bg-gradient-to-b from-green-200 to-blue-200">
                <h2 class="text-4xl font-bold">The First slide</h2>
                <p class="mt-2">Scroll Down for next slide</p>
            </div>
            <div class="flex sticky top-0 flex-col justify-center items-center h-screen text-white bg-gradient-to-b from-indigo-800 to-purple-800">
                <h2 class="text-4xl font-bold">The Second slide</h2>
                <p class="mt-2">Scroll Down for next slide</p>
            </div>
            <div class="flex sticky top-0 flex-col justify-center items-center h-screen text-white bg-gradient-to-b from-purple-800 to-pink-800">
                <h2 class="text-4xl font-bold">The Third slide</h2>
                <p class="mt-2">Scroll Down</p>
            </div>
            <div class="flex sticky top-0 flex-col justify-center items-center h-screen text-black bg-gradient-to-b from-blue-200 to-indigo-100">
                <h2 class="text-4xl font-bold">The Fourth slide</h2>
            </div>
        </div>
    }
}

#[component]
pub fn HoverEffectCard() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative flex-col justify-center py-6 min-h-screen bg-gray-50 sm:py-12">
            <div class="overflow-hidden relative px-6 pt-10 pb-8 bg-white ring-1 shadow-xl transition-all duration-300 cursor-pointer sm:px-10 sm:mx-auto sm:max-w-sm sm:rounded-lg hover:shadow-2xl hover:-translate-y-1 group ring-gray-900/5">
                <span class="absolute top-10 z-0 w-20 h-20 rounded-full transition-all duration-300 bg-sky-500 group-hover:scale-[10]"></span>
                <div class="relative z-10 mx-auto max-w-md">
                    <span class="grid place-items-center w-20 h-20 rounded-full transition-all duration-300 bg-sky-500 group-hover:bg-sky-400">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-10 h-10 text-white transition-all"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M8.625 9.75a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H8.25m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H12m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0h-.375m-13.5 3.01c0 1.6 1.123 2.994 2.707 3.227 1.087.16 2.185.283 3.293.369V21l4.184-4.183a1.14 1.14 0 01.778-.332 48.294 48.294 0 005.83-.498c1.585-.233 2.708-1.626 2.708-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z"
                            />
                        </svg>
                    </span>
                    <div class="pt-5 space-y-6 text-base leading-7 text-gray-600 transition-all duration-300 group-hover:text-white/90">
                        <p>
                            Perfect for learning how the framework works, prototyping a new idea, or creating a demo to share
                             online.
                        </p>
                    </div>
                    <div class="pt-5 text-base font-semibold leading-7">
                        <p>
                            <a
                                href="#"
                                class="transition-all duration-300 group-hover:text-white text-sky-500"
                            >
                                Read the docs
                                &rarr;
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CardRibbon() -> impl IntoView {
    view! {
        <div class="overflow-hidden relative w-56 h-56 bg-white border">
            <div class="absolute top-0 right-0 w-16 h-16">
                <div class="absolute py-1 font-semibold text-center text-white bg-green-600 transform rotate-45 right-[-35px] top-[32px] w-[170px]">
                    20% off
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn FancyUnderline() -> impl IntoView {
    view! {
        <p class="text-3xl font-bold">
            this is fancy
            <span class="inline-block relative mx-1 text-4xl font-extrabold text-green-500 stroke-current">
                underline
                <svg
                    class="absolute -bottom-0.5 w-full max-h-1.5"
                    viewBox="0 0 55 5"
                    xmlns="http://www.w3.org/2000/svg"
                    preserveAspectRatio="none"
                >
                    <path
                        d="M0.652466 4.00002C15.8925 2.66668 48.0351 0.400018 54.6853 2.00002"
                        stroke-width="2"
                    ></path>
                </svg>
            </span>text
        </p>
    }
}

#[component]
pub fn TypingEffect() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-10 min-h-screen bg-gradient-to-tr from-indigo-900 to-blue-700">
            <div class="w-max">
                <h1 class="overflow-hidden pr-5 text-5xl font-bold text-white whitespace-nowrap border-r-4 animate-typing border-r-white">
                    Hello World
                </h1>
            </div>
        </div>
    }
}

#[component]
pub fn TypingIndicator() -> impl IntoView {
    view! {
        <div class="flex space-x-1 justify-left">
            <div class="p-3 rounded-lg bg-muted">
                <div class="flex -space-x-2.5">
                    <Dot class="size-5 animate-typing-dot-bounce" />
                    <Dot class="size-5 animate-typing-dot-bounce [animation-delay:90ms]" />
                    <Dot class="size-5 animate-typing-dot-bounce [animation-delay:180ms]" />
                </div>
            </div>
        </div>
    }
}
