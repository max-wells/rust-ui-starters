use leptos::*;

#[component]
pub fn CssTrickNthChildSelector() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-50">
                <div class="py-24 px-4 mx-auto w-full max-w-5xl md:px-6">

                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    first:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border first:bg-indigo-200 first:border-indigo-400 border-slate-300 bg-slate-100"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the first item</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    last:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border last:bg-indigo-200 last:border-indigo-400 border-slate-300 bg-slate-100"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the last item</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    odd:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border odd:bg-indigo-200 odd:border-indigo-400 border-slate-300 bg-slate-100"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get odd items</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    even:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                                <span class="border even:bg-indigo-200 even:border-indigo-400 border-slate-300 bg-slate-100"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get even items</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(3n)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n)]:border-indigo-400 [&:nth-child(3n)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get every 3rd element</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(3n)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n)]:border-indigo-400 [&:nth-last-child(3n)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get every 3rd item from the end</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(3n+1)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n+1)]:border-indigo-400 [&:nth-child(3n+1)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get every 3rd item, starting from the first
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(3n+1)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n+1)]:border-indigo-400 [&:nth-last-child(3n+1)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get every 3rd item from the end, starting from the first
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(3n-1)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3n-1)]:border-indigo-400 [&:nth-child(3n-1)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get every 3rd item, starting from the second
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(3n-1)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3n-1)]:border-indigo-400 [&:nth-last-child(3n-1)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get every 3rd item from the end, starting from the second
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(3)]:border-indigo-400 [&:nth-child(3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the 3rd element</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(3)]:border-indigo-400 [&:nth-last-child(3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the 3rd to last item</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(-n+3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+3)]:border-indigo-400 [&:nth-child(-n+3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the first 3 elements</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(-n+3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-last-child(-n+3)]:border-indigo-400 [&:nth-last-child(-n+3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">Get the last 3 elements</h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(n+3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3)]:border-indigo-400 [&:nth-child(n+3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get all items starting from the 3rd
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:not(:nth-last-child(-n+2))]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:not(:nth-last-child(-n+2))]:border-indigo-400 [&:not(:nth-last-child(-n+2))]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get all items except the first 2, starting from the end
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(n+3):nth-last-child(n+3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(n+3):nth-last-child(n+3)]:border-indigo-400 [&:nth-child(n+3):nth-last-child(n+3)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get items in a range between third and third-to-last
                            </h2>
                        </div>
                        <div class="p-5 bg-white rounded-xl border border-slate-200">
                            <div class="mb-4">
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-child(-n+3)]:
                                </code>
                                <code class="py-0.5 px-1 font-mono text-sm text-indigo-500 bg-indigo-50 rounded-sm">
                                    [&:nth-last-child(-n+3)]:
                                </code>
                            </div>
                            <div class="grid grid-cols-9 gap-2 mb-8 max-w-[352px] [&_span]:block [&_span]:max-w-8 [&_span]:aspect-square [&_span]:rounded-full">
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                                <span class="border border-slate-300 bg-slate-100 [&:nth-child(-n+2)]:border-indigo-400 [&:nth-child(-n+2)]:bg-indigo-200 [&:nth-last-child(-n+2)]:border-indigo-400 [&:nth-last-child(-n+2)]:bg-indigo-200"></span>
                            </div>
                            <h2 class="text-sm font-semibold">
                                Get all items except the ones between the third and third-to-last
                            </h2>
                        </div>
                    </div>

                </div>
            </main>

        </div>
    }
}
