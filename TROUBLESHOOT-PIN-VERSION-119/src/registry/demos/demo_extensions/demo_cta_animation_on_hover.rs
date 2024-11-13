use leptos::*;

#[component]
pub fn DemoCtaAnimationOnHover() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-50">
                <div class="py-24">

                    <section class="relative z-0">
                        <div class="py-48 px-4 mx-auto w-full max-w-5xl md:px-6">
                            <div class="text-center">
                                <a
                                    class="flex flex-col justify-center items-center space-y-4 text-3xl font-semibold sm:text-4xl md:text-5xl lg:flex-row lg:space-y-0 lg:space-x-6 text-slate-900 before:absolute before:inset-0 before:-z-10 before:transition-colors before:duration-500 group hover:before:bg-slate-900"
                                    href="#0"
                                >

                                    <span class="flex overflow-hidden relative justify-center items-center p-0.5 rounded-full transition duration-500 bg-slate-200 before:opacity-0 before:absolute before:w-1/2 before:pb-[100%] before:bg-[linear-gradient(90deg,_theme(colors.indigo.500/0)_0%,_theme(colors.indigo.500)_35%,_theme(colors.indigo.200)_50%,_theme(colors.indigo.500)_65%,_theme(colors.indigo.500/0)_100%)] before:animate-[spin_3s_linear_infinite] group-hover:bg-slate-800 group-hover:before:opacity-100">
                                        <span class="relative whitespace-nowrap">

                                            <span class="block z-10 py-6 px-8 bg-gradient-to-r rounded-full transition-opacity duration-500 ease-in-out group-hover:opacity-0 from-slate-200 to-slate-100">
                                                Build the UI you need
                                            </span>

                                            <span
                                                class="inline-flex overflow-hidden absolute inset-0 z-10 items-center whitespace-nowrap bg-gradient-to-r rounded-full opacity-0 transition-opacity duration-500 group-hover:opacity-100 from-slate-900 to-slate-800 before:bg-clip-text before:text-transparent before:bg-gradient-to-r before:from-indigo-500 before:to-indigo-300 after:bg-clip-text after:text-transparent after:bg-gradient-to-r after:from-indigo-500 after:to-indigo-300 before:content-['Create_beautiful_user_interfaces'] after:content-['Create_beautiful_user_interfaces'] before:px-2 after:px-2 before:animate-infinite-scroll after:animate-infinite-scroll"
                                                aria-hidden="true"
                                            ></span>
                                        </span>
                                    </span>

                                    <span class="transition-colors duration-500 ease-in-out group-hover:text-slate-300">
                                        with Cruip
                                    </span>

                                </a>
                            </div>
                        </div>
                    </section>

                </div>
            </main>

        </div>
    }
}
