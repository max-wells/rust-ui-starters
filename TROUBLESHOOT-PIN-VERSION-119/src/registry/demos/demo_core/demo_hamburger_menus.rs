use leptos::*;

#[component]
pub fn DemoHamburgerMenus() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-20 min-w-full bg-gradient-to-b from-sky-100 to-sky-900">
            <div class="grid grid-cols-1 gap-16 m-4 sm:grid-cols-3">
                <div>
                    <button class="relative group">
                        <div class="flex relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex flex-col justify-between transition-all duration-300 transform origin-center w-[20px] h-[20px] group-focus:-rotate-[45deg]">
                                <div class="w-1/2 bg-white rounded transition-all duration-300 delay-75 transform origin-right h-[2px] group-focus:-rotate90 group-focus:h-[1px] group-focus:-translate-y-[1px]"></div>
                                <div class="bg-white rounded h-[1px]"></div>
                                <div class="self-end w-1/2 bg-white rounded transition-all duration-300 delay-75 transform origin-left h-[2px] group-focus:-rotate90 group-focus:h-[1px] group-focus:translate-y-[1px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-300 transform origin-left h-[2px] group-focus:rotate-[42deg]"></div>
                                <div class="w-1/2 bg-white rounded transition-all duration-300 transform group-focus:-translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 transform origin-left h-[2px] group-focus:-rotate-[42deg]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center group-focus:translate-x-1.5 w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:rotate-[42deg]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 transform group-focus:translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:-rotate-[42deg]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center group-focus:-translate-x-1.5 w-[20px] h-[20px] group-focus:rotate180">
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:rotate-[42deg]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 transform group-focus:translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:-rotate-[42deg]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center group-focus:-translate-y-1.5 w-[20px] h-[20px] group-focus:-rotate90">
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:rotate-[42deg]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 transform group-focus:translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:w-2/3 h-[2px] group-focus:-rotate-[42deg]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-300 transform origin-left group-focus:translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 delay-75 transform group-focus:translate-x-10 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-150 transform origin-left group-focus:translate-x-10 h-[2px]"></div>
                                <div class="flex absolute top-2.5 justify-between items-center w-0 transition-all duration-500 transform -translate-x-10 group-focus:w-12 group-focus:translate-x-0">
                                    <div class="absolute w-5 bg-white transition-all duration-500 delay-300 transform rotate-0 group-focus:rotate-45 h-[2px]"></div>
                                    <div class="absolute w-5 bg-white transition-all duration-500 delay-300 transform group-focus:-rotate-45 h-[2px] -rotate-0"></div>
                                </div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-300 delay-100 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 delay-75 transform group-focus:translate-y-6 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                                <div class="flex absolute top-2.5 justify-between items-center w-0 transition-all duration-500 transform -translate-x-10 group-focus:w-12 group-focus:translate-x-0">
                                    <div class="absolute w-5 bg-white transition-all duration-500 delay-300 transform rotate-0 group-focus:rotate-45 h-[2px]"></div>
                                    <div class="absolute w-5 bg-white transition-all duration-500 delay-300 transform group-focus:-rotate-45 h-[2px] -rotate-0"></div>
                                </div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative flex-col justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="overflow-hidden transition-all duration-150 transform -translate-y-5 group-focus:translate-y-3">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="w-6 h-6 text-white animate-bounce"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M5 15l7-7 7 7"
                                    ></path>
                                </svg>
                            </div>
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center -translate-y-3 w-[20px] h-[20px]">
                                <div class="mb-1.5 w-7 bg-white transition-all duration-300 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                                <div class="mb-1.5 w-7 bg-white rounded transition-all duration-300 delay-75 transform group-focus:translate-y-6 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-100 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative flex-col justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="overflow-hidden transition-all duration-150 transform -translate-y-5 group-focus:translate-y-3">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="w-6 h-6 text-white animate-bounce w-6h-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M19 9l-7 7-7-7"
                                    ></path>
                                </svg>
                            </div>
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-300 transform origin-center -translate-y-3 w-[20px] h-[20px]">
                                <div class="mb-1.5 w-7 bg-white transition-all duration-300 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                                <div class="mb-1.5 w-7 bg-white rounded transition-all duration-300 delay-75 transform group-focus:translate-y-6 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-100 transform origin-left group-focus:translate-y-6 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between pt-1 transition-all duration-100 transform origin-center w-[20px] h-[20px] group-focus:rotate90">
                                <div class="w-7 bg-white transition-all duration-300 delay-75 transform group-focus:w-0 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-300 delay-75 transform group-focus:w-0 h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-300 delay-75 transform group-focus:w-0 h-[2px]"></div>
                                <div class="overflow-hidden w-0 h-0 transition-all duration-300 delay-150 transform group-focus:-mt-2 group-focus:w-12 group-focus:h-12">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="w-6 h-6 text-white w-6h-6"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                        stroke-width="2"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M19 9l-7 7-7-7"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-500 transform origin-center w-[20px] h-[20px] group-focus:-rotate180">
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:rotate-45 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-500 transform h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:-rotate-45 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-500 transform origin-center w-[20px] h-[20px] group-focus:rotate180">
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:-rotate-45 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-500 transform h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:rotate-45 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-500 transform origin-center w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:-rotate-45 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-500 transform h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:rotate-45 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
                <div>
                    <button class="relative group">
                        <div class="flex overflow-hidden relative justify-center items-center rounded-full ring-0 ring-gray-300 ring-opacity-30 shadow-md transition-all duration-200 transform group-focus:ring-4 hover:ring-8 w-[50px] h-[50px] bg-slate-700">
                            <div class="flex overflow-hidden flex-col justify-between transition-all duration-500 transform origin-center w-[20px] h-[20px]">
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:rotate-45 h-[2px]"></div>
                                <div class="w-7 bg-white rounded transition-all duration-500 transform h-[2px]"></div>
                                <div class="w-7 bg-white transition-all duration-500 transform -translate-x-1 group-focus:-rotate-45 h-[2px]"></div>
                            </div>
                        </div>
                    </button>
                </div>
            </div>
        </div>
    }
}
