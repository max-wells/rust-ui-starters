use leptos::*;

#[component]
pub fn CssTrickCardBlog() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center min-h-screen bg-neutral-800">
            <div class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
                <div class="overflow-hidden relative justify-center items-center transition-shadow cursor-pointer hover:shadow-xl group hover:shadow-black/30">
                    <div class="w-72 h-96">
                        <img
                            class="object-cover w-full h-full transition-transform duration-500 group-hover:scale-125 group-hover:rotate-3"
                            src="https://images.unsplash.com/photo-1524504388940-b1c1722653e1?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=687&q=80"
                            alt=""
                        />
                    </div>
                    <div class="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black group-hover:from-black/70 group-hover:via-black/60 group-hover:to-black/70"></div>
                    <div class="flex absolute inset-0 flex-col justify-center items-center px-9 text-center transition-all duration-500 group-hover:translate-y-0 translate-y-[60%]">
                        <h1 class="text-3xl font-bold text-white font-dmserif">Beauty</h1>
                        <p class="mb-3 text-lg italic text-white opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            Lorem ipsum dolor sit amet consectetur adipisicing elit. Facilis dolore adipisci placeat.
                        </p>
                        <button class="py-2 px-3.5 text-sm text-white capitalize rounded-full shadow bg-neutral-900 font-com shadow-black/60">
                            See More
                        </button>
                    </div>
                </div>
                <div class="overflow-hidden relative justify-center items-center transition-shadow cursor-pointer hover:shadow-xl group hover:shadow-black/30">
                    <div class="w-72 h-96">
                        <img
                            class="object-cover w-full h-full transition-transform duration-500 group-hover:scale-125 group-hover:rotate-3"
                            src="https://images.unsplash.com/photo-1494145904049-0dca59b4bbad?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=688&q=80"
                            alt=""
                        />
                    </div>
                    <div class="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black group-hover:from-black/70 group-hover:via-black/60 group-hover:to-black/70"></div>
                    <div class="flex absolute inset-0 flex-col justify-center items-center px-9 text-center transition-all duration-500 group-hover:translate-y-0 translate-y-[60%]">
                        <h1 class="text-3xl font-bold text-white font-dmserif">Beyond Builder</h1>
                        <p class="mb-3 text-lg italic text-white opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            Lorem ipsum dolor sit amet consectetur adipisicing elit. Facilis dolore adipisci placeat.
                        </p>
                        <button class="py-2 px-3.5 text-sm text-white capitalize rounded-full shadow bg-neutral-900 font-com shadow-black/60">
                            See More
                        </button>
                    </div>
                </div>
                <div class="overflow-hidden relative justify-center items-center transition-shadow cursor-pointer hover:shadow-xl group hover:shadow-black/30">
                    <div class="w-72 h-96">
                        <img
                            class="object-cover w-full h-full transition-transform duration-500 group-hover:scale-125 group-hover:rotate-3"
                            src="https://images.unsplash.com/photo-1502675135487-e971002a6adb?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=688&q=80"
                            alt=""
                        />
                    </div>
                    <div class="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-black group-hover:from-black/70 group-hover:via-black/60 group-hover:to-black/70"></div>
                    <div class="flex absolute inset-0 flex-col justify-center items-center px-9 text-center transition-all duration-500 group-hover:translate-y-0 translate-y-[60%]">
                        <h1 class="text-3xl font-bold text-white font-dmserif">Shooting Star</h1>
                        <p class="mb-3 text-lg italic text-white opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                            Lorem ipsum dolor sit amet consectetur adipisicing elit. Facilis dolore adipisci placeat.
                        </p>
                        <button class="py-2 px-3.5 text-sm text-white capitalize rounded-full shadow bg-neutral-900 font-com shadow-black/60">
                            See More
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
