use leptos::*;

#[component]
pub fn PageBentoGrids() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <BentoGridOne />
            <BentoGridTwo />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn BentoGridOne() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="py-4 px-2 mx-auto max-w-screen-xl sm:py-4 lg:px-6">
                <div class="grid grid-cols-1 gap-4 h-full sm:grid-cols-2 md:grid-cols-5">
                    <div class="flex flex-col col-span-2 h-auto bg-gray-50 sm:col-span-1 md:col-span-2 md:h-full">
                        <a
                            href=""
                            class="flex overflow-hidden relative flex-col flex-grow px-4 pt-40 pb-4 rounded-lg group"
                        >
                            <img
                                src="https://images.unsplash.com/photo-1510812431401-41d2bd2722f3?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                                alt=""
                                class="object-cover absolute inset-0 w-full h-full transition-transform duration-500 ease-in-out group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-gradient-to-b from-gray-900/25 to-gray-900/5"></div>
                            <h3 class="absolute top-0 left-0 z-10 p-4 text-2xl font-medium text-white md:text-3xl xs:text-xl">
                                Wines
                            </h3>
                        </a>
                    </div>
                    <div class="col-span-2 sm:col-span-1 md:col-span-2 bg-stone-50">
                        <a
                            href=""
                            class="flex overflow-hidden relative flex-col px-4 pt-40 pb-4 mb-4 rounded-lg group"
                        >
                            <img
                                src="https://images.unsplash.com/photo-1504675099198-7023dd85f5a3?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                                alt=""
                                class="object-cover absolute inset-0 w-full h-full transition-transform duration-500 ease-in-out group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-gradient-to-b from-gray-900/25 to-gray-900/5"></div>
                            <h3 class="absolute top-0 left-0 z-10 p-4 text-2xl font-medium text-white md:text-3xl xs:text-xl">
                                Gin
                            </h3>
                        </a>
                        <div class="grid grid-cols-2 gap-4 sm:grid-cols-2 lg:grid-cols-2">
                            <a
                                href=""
                                class="flex overflow-hidden relative flex-col px-4 pt-40 pb-4 rounded-lg group"
                            >
                                <img
                                    src="https://images.unsplash.com/photo-1571104508999-893933ded431?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                                    alt=""
                                    class="object-cover absolute inset-0 w-full h-full transition-transform duration-500 ease-in-out group-hover:scale-105"
                                />
                                <div class="absolute inset-0 bg-gradient-to-b from-gray-900/25 to-gray-900/5"></div>
                                <h3 class="absolute top-0 left-0 z-10 p-4 text-2xl font-medium text-white md:text-3xl xs:text-xl">
                                    Whiskey
                                </h3>
                            </a>
                            <a
                                href=""
                                class="flex overflow-hidden relative flex-col px-4 pt-40 pb-4 rounded-lg group"
                            >
                                <img
                                    src="https://images.unsplash.com/photo-1626897505254-e0f811aa9bf7?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                                    alt=""
                                    class="object-cover absolute inset-0 w-full h-full transition-transform duration-500 ease-in-out group-hover:scale-105"
                                />
                                <div class="absolute inset-0 bg-gradient-to-b from-gray-900/25 to-gray-900/5"></div>
                                <h3 class="absolute top-0 left-0 z-10 p-4 text-2xl font-medium text-white md:text-3xl xs:text-xl">
                                    Vodka
                                </h3>
                            </a>
                        </div>
                    </div>
                    <div class="flex flex-col col-span-2 h-auto sm:col-span-1 md:col-span-1 md:h-full bg-sky-50">
                        <a
                            href=""
                            class="flex overflow-hidden relative flex-col flex-grow px-4 pt-40 pb-4 rounded-lg group"
                        >
                            <img
                                src="https://images.unsplash.com/photo-1693680501357-a342180f1946?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                                alt=""
                                class="object-cover absolute inset-0 w-full h-full transition-transform duration-500 ease-in-out group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-gradient-to-b from-gray-900/25 to-gray-900/5"></div>
                            <h3 class="absolute top-0 left-0 z-10 p-4 text-2xl font-medium text-white md:text-3xl xs:text-xl">
                                Brandy
                            </h3>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn BentoGridTwo() -> impl IntoView {
    view! {
        <div class="bg-gray-100">
            <div class="container py-8 px-4 mx-auto">
                <h1 class="mb-8 text-4xl font-bold text-center">Bento Grid Layout with Images</h1>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-4">

                    <div class="overflow-hidden relative rounded-2xl shadow-lg md:col-span-2 md:row-span-2 group">
                        <img
                            src="https://images.unsplash.com/photo-1469474968028-56623f02e42e?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwxfHxuYXR1cmV8ZW58MHwwfHx8MTcyMTA0MjYwMXww&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Nature"
                            class="object-cover w-full h-full"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h3 class="text-2xl font-bold text-white">Explore Nature</h3>
                                <p class="text-white">Discover the beauty of the natural world</p>
                            </div>
                        </div>
                    </div>

                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1493770348161-369560ae357d?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHw2fHxmb29kfGVufDB8MHx8fDE3MjEwNDI2MTR8MA&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Food"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Culinary Delights</h4>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1519389950473-47ba0277781c?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHw1fHx0ZWNobm9sb2d5fGVufDB8MHx8fDE3MjEwNDI2Mjh8MA&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Technology"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Tech Innovations</h4>
                            </div>
                        </div>
                    </div>

                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1503220317375-aaad61436b1b?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHw1fHx0cmF2ZWx8ZW58MHwwfHx8MTcyMTA0MjY0MXww&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Travel"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Travel Adventures</h4>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1513364776144-60967b0f800f?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwxfHxhcnR8ZW58MHwwfHx8MTcyMTA0MjY5Nnww&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Art"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Artistic Expressions</h4>
                            </div>
                        </div>
                    </div>

                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1530549387789-4c1017266635?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwyfHxzd2ltbWluZ3xlbnwwfDB8fHwxNzIxMDQzMjkxfDA&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Sport"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Swimming</h4>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1611195974226-a6a9be9dd763?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHwxMnx8Y2hlc3N8ZW58MHwwfHx8MTcyMTA0MzI0Nnww&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Sport"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Chess</h4>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1553778263-73a83bab9b0c?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHw1fHxmb290YmFsbHxlbnwwfDB8fHwxNzIxMDQzMjExfDA&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Sport"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Football</h4>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-hidden relative rounded-2xl shadow-lg group">
                        <img
                            src="https://images.unsplash.com/photo-1624526267942-ab0ff8a3e972?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w0NzEyNjZ8MHwxfHNlYXJjaHw3fHxjcmlja2V0fGVufDB8MHx8fDE3MjEwNDMxNTh8MA&ixlib=rb-4.0.3&q=80&w=1080"
                            alt="Sport"
                            class="object-cover w-full h-48"
                        />
                        <div class="absolute inset-0 bg-black bg-opacity-60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            <div class="absolute right-0 bottom-0 left-0 p-4">
                                <h4 class="text-xl font-bold text-white">Cricket</h4>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
