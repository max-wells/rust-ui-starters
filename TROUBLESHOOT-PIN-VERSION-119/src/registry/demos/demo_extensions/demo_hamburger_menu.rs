use leptos::*;

#[component]
pub fn DemoHamburgerMenu() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-50">
                <div class="py-24 px-4 mx-auto w-full max-w-6xl md:px-6">
                    <div class="flex justify-center">

                        <div class="flex gap-6 justify-center">

                            <button
                                class="inline-flex justify-center items-center w-12 h-12 text-center bg-white rounded transition group text-slate-800 shadow-[0_1px_0_theme(colors.slate.950/.04),0_1px_2px_theme(colors.slate.950/.12),inset_0_-2px_0_theme(colors.slate.950/.04)] hover:shadow-[0_1px_0_theme(colors.slate.950/.04),0_4px_8px_theme(colors.slate.950/.12),inset_0_-2px_0_theme(colors.slate.950/.04)]"
                                aria-pressed="false"
                                onclick="this.setAttribute('aria-pressed', !(this.getAttribute('aria-pressed') === 'true'))"
                            >
                                <span class="sr-only">Menu</span>
                                <svg
                                    class="w-6 h-6 pointer-events-none fill-current"
                                    viewBox="0 0 16 16"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:rotate-[315deg] group-[[aria-pressed=true]]:[y:7] group-[[aria-pressed=true]]:[x:0] ease-[cubic-bezier(.5,.85,.25,1.1)]"
                                        y="2"
                                        x="7"
                                        width="9"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:rotate-45 ease-[cubic-bezier(.5,.85,.25,1.8)]"
                                        y="7"
                                        width="16"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:rotate-[135deg] group-[[aria-pressed=true]]:[y:7] group-[[aria-pressed=true]]:[x:0] ease-[cubic-bezier(.5,.85,.25,1.1)]"
                                        y="12"
                                        width="9"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                </svg>
                            </button>

                            <button
                                class="inline-flex justify-center items-center w-12 h-12 text-center bg-white rounded transition group text-slate-800 shadow-[0_1px_0_theme(colors.slate.950/.04),0_1px_2px_theme(colors.slate.950/.12),inset_0_-2px_0_theme(colors.slate.950/.04)] hover:shadow-[0_1px_0_theme(colors.slate.950/.04),0_4px_8px_theme(colors.slate.950/.12),inset_0_-2px_0_theme(colors.slate.950/.04)]"
                                aria-pressed="false"
                                onclick="this.setAttribute('aria-pressed', !(this.getAttribute('aria-pressed') === 'true'))"
                            >
                                <span class="sr-only">Menu</span>
                                <svg
                                    class="w-6 h-6 pointer-events-none fill-current"
                                    viewBox="0 0 16 16"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:rotate-[315deg] group-[[aria-pressed=true]]:[y:7] group-[[aria-pressed=true]]:[x:0] ease-[cubic-bezier(.5,.85,.25,1.1)]"
                                        y="2"
                                        x="7"
                                        width="9"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:rotate-45 ease-[cubic-bezier(.5,.85,.25,1.8)]"
                                        y="7"
                                        width="16"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                    <rect
                                        class="transition-all duration-300 origin-center group-[[aria-pressed=true]]:-rotate-[225deg] group-[[aria-pressed=true]]:[y:7] group-[[aria-pressed=true]]:[x:0] ease-[cubic-bezier(.5,.85,.25,1.1)]"
                                        y="12"
                                        width="9"
                                        height="2"
                                        rx="1"
                                    ></rect>
                                </svg>
                            </button>

                        </div>

                    </div>
                </div>
            </main>

        </div>
    }
}
