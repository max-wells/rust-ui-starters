use leptos::*;

#[component]
pub fn CssTrickStickBottomCards() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-50">
                <div class="py-24 px-4 mx-auto w-full max-w-5xl md:px-6">

                    <div class="grid gap-6 items-start mx-auto max-w-xs lg:grid-cols-3 lg:max-w-none">

                        <div class="flex overflow-hidden flex-col h-full bg-white rounded-2xl border shadow border-slate-200 shadow-slate-950/5">

                            <img
                                class="object-cover w-full h-48"
                                src="https://cruip-tutorials.vercel.app/equal-height-cards/equal-height-01.jpg"
                                width="304"
                                height="192"
                                alt="Course 01"
                            />

                            <div class="flex flex-col flex-1 p-6">

                                <div class="flex-1">

                                    <header class="mb-2">
                                        <h2 class="text-xl font-extrabold leading-snug">
                                            <a
                                                class="focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-900"
                                                href="#0"
                                            >
                                                Unlocking the Secrets of Productivity
                                            </a>
                                        </h2>
                                    </header>

                                    <div class="mb-8 text-sm text-slate-600">
                                        <p>
                                            Boost efficiency, accomplish more. Learn proven strategies.
                                        </p>
                                    </div>
                                </div>

                                <div class="flex justify-end space-x-2">
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium whitespace-nowrap bg-transparent rounded-lg transition-colors focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-500 hover:bg-slate-100"
                                        href="#0"
                                    >
                                        Cancel
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-indigo-500 whitespace-nowrap bg-indigo-50 rounded-lg transition-colors hover:bg-indigo-100 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Preview
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-white whitespace-nowrap bg-indigo-500 rounded-lg transition-colors hover:bg-indigo-600 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Buy Now
                                    </a>
                                </div>
                            </div>
                        </div>

                        <div class="flex overflow-hidden flex-col h-full bg-white rounded-2xl border shadow border-slate-200 shadow-slate-950/5">

                            <img
                                class="object-cover w-full h-48"
                                src="https://cruip-tutorials.vercel.app/equal-height-cards/equal-height-02.jpg"
                                width="304"
                                height="192"
                                alt="Course 02"
                            />

                            <div class="flex flex-col flex-1 p-6">

                                <div class="flex-1">

                                    <header class="mb-2">
                                        <h2 class="text-xl font-extrabold leading-snug">
                                            <a
                                                class="focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-900"
                                                href="#0"
                                            >
                                                The Ultimate JavaScript Course
                                            </a>
                                        </h2>
                                    </header>

                                    <div class="mb-8 text-sm text-slate-600">
                                        <p>
                                            The JavaScript course for everyone! Master JavaScript with projects, challenges and theory.
                                        </p>
                                    </div>
                                </div>

                                <div class="flex justify-end space-x-2">
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium whitespace-nowrap bg-transparent rounded-lg transition-colors focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-500 hover:bg-slate-100"
                                        href="#0"
                                    >
                                        Cancel
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-indigo-500 whitespace-nowrap bg-indigo-50 rounded-lg transition-colors hover:bg-indigo-100 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Preview
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-white whitespace-nowrap bg-indigo-500 rounded-lg transition-colors hover:bg-indigo-600 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Buy Now
                                    </a>
                                </div>
                            </div>
                        </div>

                        <div class="flex overflow-hidden flex-col h-full bg-white rounded-2xl border shadow border-slate-200 shadow-slate-950/5">

                            <img
                                class="object-cover w-full h-48"
                                src="https://cruip-tutorials.vercel.app/equal-height-cards/equal-height-03.jpg"
                                width="304"
                                height="192"
                                alt="Course 03"
                            />

                            <div class="flex flex-col flex-1 p-6">

                                <div class="flex-1">

                                    <header class="mb-2">
                                        <h2 class="text-xl font-extrabold leading-snug">
                                            <a
                                                class="focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-900"
                                                href="#0"
                                            >
                                                Mastering Python Course
                                            </a>
                                        </h2>
                                    </header>

                                    <div class="mb-8 text-sm text-slate-600">
                                        <p>
                                            Unlock the power of Python. From basics to advanced techniques, become a coding maestro with our comprehensive course.
                                        </p>
                                    </div>
                                </div>

                                <div class="flex justify-end space-x-2">
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium whitespace-nowrap bg-transparent rounded-lg transition-colors focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none text-slate-500 hover:bg-slate-100"
                                        href="#0"
                                    >
                                        Cancel
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-indigo-500 whitespace-nowrap bg-indigo-50 rounded-lg transition-colors hover:bg-indigo-100 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Preview
                                    </a>
                                    <a
                                        class="inline-flex justify-center py-2 px-3 text-sm font-medium text-white whitespace-nowrap bg-indigo-500 rounded-lg transition-colors hover:bg-indigo-600 focus-visible:ring focus-visible:ring-indigo-300 focus-visible:outline-none"
                                        href="#0"
                                    >
                                        Buy Now
                                    </a>
                                </div>
                            </div>
                        </div>

                    </div>

                </div>
            </main>

        </div>
    }
}
