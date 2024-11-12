use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoSelectPriceTable() -> impl IntoView {
    view! {
        <Stylesheet id="css-select-price-table" href="/components/select-price-table.css" />

        <div class="relative antialiased font-inter">

            <div class="flex flex-col justify-center items-center w-screen min-h-screen">

                <form class="grid grid-cols-4 gap-4 w-full max-w-screen-lg text-gray-700">
                    <div>
                        <input
                            class="hidden"
                            id="option_1"
                            type="radio"
                            name="options"
                            value="option1 "
                        />
                        <label
                            class="flex flex-col p-8 rounded-3xl border-2 border-white hover:border-gray-200"
                            for="option_1"
                        >
                            <h3 class="text-lg">Starter</h3>
                            <div class="flex items-center mt-4">
                                <span class="text-4xl font-semibold text-purple-900">$20</span>
                                <p class="ml-2 text-sm text-gray-500">/ month</p>
                            </div>
                            <p class="mt-4 text-xs">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
                            </p>
                            <ul class="mt-8 space-y-4 text-sm">
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 1</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 2</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 3</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 4</span>
                                </li>
                            </ul>
                            <div class="flex justify-center items-center mt-8 w-full h-12 text-sm font-semibold text-purple-600 rounded-lg border border-purple-600 cursor-pointer hover:text-purple-50 hover:bg-purple-600 button">
                                Select
                            </div>
                        </label>
                    </div>
                    <div>
                        <input
                            class="hidden"
                            id="option_2"
                            type="radio"
                            name="options"
                            value="option1"
                        />
                        <label
                            class="flex flex-col p-8 rounded-3xl border-2 border-white hover:border-gray-200"
                            for="option_2"
                        >
                            <h3 class="text-lg">Pro</h3>
                            <div class="flex items-center mt-4">
                                <span class="text-4xl font-semibold text-purple-900">$50</span>
                                <p class="ml-2 text-sm text-gray-500">/ month</p>
                            </div>
                            <p class="mt-4 text-xs">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
                            </p>
                            <ul class="mt-8 space-y-4 text-sm">
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 1</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 2</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 3</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 4</span>
                                </li>
                            </ul>
                            <div class="flex justify-center items-center mt-8 w-full h-12 text-sm font-semibold text-purple-600 rounded-lg border border-purple-600 cursor-pointer hover:text-purple-50 hover:bg-purple-600 button">
                                Select
                            </div>
                        </label>
                    </div>
                    <div>
                        <input
                            class="hidden"
                            id="option_3"
                            type="radio"
                            name="options"
                            value="option3"
                            checked
                        />
                        <label
                            class="flex flex-col p-8 rounded-3xl border-2 border-white hover:border-gray-200"
                            for="option_3"
                        >
                            <h3 class="text-lg">Master</h3>
                            <div class="flex items-center mt-4">
                                <span class="text-4xl font-semibold text-purple-900">$200</span>
                                <p class="ml-2 text-sm text-gray-500">/ month</p>
                            </div>
                            <p class="mt-4 text-xs">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
                            </p>
                            <ul class="mt-8 space-y-4 text-sm">
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 1</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 2</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 3</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 4</span>
                                </li>
                            </ul>
                            <div class="flex justify-center items-center mt-8 w-full h-12 text-sm font-semibold text-purple-600 rounded-lg border border-purple-600 cursor-pointer hover:text-purple-50 hover:bg-purple-600 button">
                                Select
                            </div>
                        </label>
                    </div>
                    <div>
                        <input
                            class="hidden"
                            id="option_4"
                            type="radio"
                            name="options"
                            value="option1"
                        />
                        <label
                            class="flex flex-col p-8 rounded-3xl border-2 border-white hover:border-gray-200"
                            for="option_4"
                        >
                            <h3 class="text-lg">Enterprise</h3>
                            <div class="flex items-center mt-4">
                                <span class="text-4xl font-semibold text-purple-900">$599</span>
                                <p class="ml-2 text-sm text-gray-500">/ month</p>
                            </div>
                            <p class="mt-4 text-xs">
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
                            </p>
                            <ul class="mt-8 space-y-4 text-sm">
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 1</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 2</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 3</span>
                                </li>
                                <li class="flex items-center">
                                    <svg
                                        class="w-5 h-5 text-purple-900 opacity-40 fill-current"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    <span class="ml-2">Feature 4</span>
                                </li>
                            </ul>
                            <div class="flex justify-center items-center mt-8 w-full h-12 text-sm font-semibold text-purple-600 rounded-lg border border-purple-600 cursor-pointer hover:text-purple-50 hover:bg-purple-600 button">
                                Select
                            </div>
                        </label>
                    </div>
                </form>

            </div>
        </div>
    }
}
