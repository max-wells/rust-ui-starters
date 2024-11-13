use leptos::*;

#[component]
pub fn DemoFloatingButtonMenu() -> impl IntoView {
    view! {
        <div class="fixed bottom-8 left-8 z-10">
            <div class="flex flex-col-reverse gap-2 group">
                <button class="inline-flex relative z-50 gap-2 justify-center items-center self-center px-6 h-12 text-sm font-medium tracking-wide text-white whitespace-nowrap bg-emerald-500 rounded transition duration-300 hover:bg-emerald-600 focus:bg-emerald-700 focus-visible:outline-none disabled:bg-emerald-300 disabled:border-emerald-300 disabled:shadow-none disabled:cursor-not-allowed group">
                    <span class="relative transition duration-300 group-hover:rotate-45 only:-mx-6">
                        <span class="sr-only">Button description</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                            aria-label="Plus icon"
                            role="graphics-symbol"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M12 4.5v15m7.5-7.5h-15"
                            />
                        </svg>
                    </span>
                </button>
                <button class="inline-flex overflow-hidden gap-2 justify-center justify-self-center items-center self-center px-6 w-0 h-0 text-sm font-medium tracking-wide text-emerald-500 whitespace-nowrap bg-emerald-50 rounded opacity-0 transition duration-300 translate-y-2 group-hover:w-12 group-hover:h-12 group-hover:opacity-100 group-hover:translate-y-0 hover:text-emerald-600 hover:bg-emerald-100 focus:text-emerald-700 focus:bg-emerald-200 focus-visible:outline-none disabled:text-emerald-400 disabled:bg-emerald-100 disabled:border-emerald-300 disabled:shadow-none disabled:cursor-not-allowed">
                    <span class="relative only:-mx-6">
                        <span class="sr-only">Button description</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                            aria-label="pencil-square icon"
                            role="graphics-symbol"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                            />
                        </svg>
                    </span>
                </button>
                <button class="inline-flex overflow-hidden gap-2 justify-center justify-self-center items-center self-center px-6 w-0 h-0 text-sm font-medium tracking-wide text-emerald-500 whitespace-nowrap bg-emerald-50 rounded opacity-0 transition duration-300 translate-y-2 group-hover:w-12 group-hover:h-12 group-hover:opacity-100 group-hover:translate-y-0 hover:text-emerald-600 hover:bg-emerald-100 focus:text-emerald-700 focus:bg-emerald-200 focus-visible:outline-none disabled:text-emerald-400 disabled:bg-emerald-100 disabled:border-emerald-300 disabled:shadow-none disabled:cursor-not-allowed delay-[0.05s]">
                    <span class="relative only:-mx-6">
                        <span class="sr-only">Button description</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                            aria-label="document-duplicate icon"
                            role="graphics-symbol"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75"
                            />
                        </svg>
                    </span>
                </button>
                <button class="inline-flex overflow-hidden gap-2 justify-center justify-self-center items-center self-center px-6 w-0 h-0 text-sm font-medium tracking-wide text-emerald-500 whitespace-nowrap bg-emerald-50 rounded opacity-0 transition duration-300 translate-y-2 group-hover:w-12 group-hover:h-12 group-hover:opacity-100 group-hover:translate-y-0 hover:text-emerald-600 hover:bg-emerald-100 focus:text-emerald-700 focus:bg-emerald-200 focus-visible:outline-none disabled:text-emerald-400 disabled:bg-emerald-100 disabled:border-emerald-300 disabled:shadow-none disabled:cursor-not-allowed delay-[0.10s]">
                    <span class="relative only:-mx-6">
                        <span class="sr-only">Button description</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                            aria-label="camera icon"
                            role="graphics-symbol"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M6.827 6.175A2.31 2.31 0 0 1 5.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 0 0-1.134-.175 2.31 2.31 0 0 1-1.64-1.055l-.822-1.316a2.192 2.192 0 0 0-1.736-1.039 48.774 48.774 0 0 0-5.232 0 2.192 2.192 0 0 0-1.736 1.039l-.821 1.316Z"
                            />
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M16.5 12.75a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0ZM18.75 10.5h.008v.008h-.008V10.5Z"
                            />
                        </svg>
                    </span>
                </button>
                <button class="inline-flex overflow-hidden gap-2 justify-center justify-self-center items-center self-center px-6 w-0 h-0 text-sm font-medium tracking-wide text-emerald-500 whitespace-nowrap bg-emerald-50 rounded opacity-0 transition duration-300 translate-y-2 group-hover:w-12 group-hover:h-12 group-hover:opacity-100 group-hover:translate-y-0 hover:text-emerald-600 hover:bg-emerald-100 focus:text-emerald-700 focus:bg-emerald-200 focus-visible:outline-none disabled:text-emerald-400 disabled:bg-emerald-100 disabled:border-emerald-300 disabled:shadow-none disabled:cursor-not-allowed delay-[0.15s]">
                    <span class="relative only:-mx-6">
                        <span class="sr-only">Button description</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                            aria-label="camera icon"
                            role="graphics-symbol"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M7.217 10.907a2.25 2.25 0 1 0 0 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186 9.566-5.314m-9.566 7.5 9.566 5.314m0 0a2.25 2.25 0 1 0 3.935 2.186 2.25 2.25 0 0 0-3.935-2.186Zm0-12.814a2.25 2.25 0 1 0 3.933-2.185 2.25 2.25 0 0 0-3.933 2.185Z"
                            />
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    }
}
