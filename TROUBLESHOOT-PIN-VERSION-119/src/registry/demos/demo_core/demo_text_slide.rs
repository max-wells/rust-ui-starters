use leptos::*;

#[component]
pub fn DemoTextSlide() -> impl IntoView {
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
