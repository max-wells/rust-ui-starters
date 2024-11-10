use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoQuickTest() -> impl IntoView {
    view! {
        <Stylesheet id="css-xx" href="/xx.css" />
        // <script src="/xx.js"></script>

        <div class="relative antialiased font-inter">

            <div class="flex flex-col justify-center items-center space-y-10 w-screen min-h-screen bg-gray-900">

                <form class="flex space-x-2">
                    <input class="hidden form-1-input" id="1" type="radio" name="a" />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="1"
                    ></label>
                    <input class="hidden form-1-input" id="2" type="radio" name="a" />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="2"
                    ></label>
                    <input class="hidden form-1-input" id="3" type="radio" name="a" />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="3"
                    ></label>
                    <input class="hidden form-1-input" id="4" type="radio" name="a" />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="4"
                    ></label>
                    <input class="hidden form-1-input" id="5" type="radio" name="a" checked />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="5"
                    ></label>
                    <input class="hidden form-1-input" id="6" type="radio" name="a" />
                    <label
                        class="w-4 h-4 bg-purple-600 rounded-full ring-purple-900 transition-all duration-100 cursor-pointer hover:ring-4"
                        for="6"
                    ></label>
                </form>

                <form class="flex space-x-2">
                    <input class="hidden form-2-input" id="7" type="radio" name="b" />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="7"
                    ></label>
                    <input class="hidden form-2-input" id="8" type="radio" name="b" />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="8"
                    ></label>
                    <input class="hidden form-2-input" id="9" type="radio" name="b" />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="9"
                    ></label>
                    <input class="hidden form-2-input" id="10" type="radio" name="b" />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="10"
                    ></label>
                    <input class="hidden form-2-input" id=11 type="radio" name="b" checked />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="11"
                    ></label>
                    <input class="hidden form-2-input" id="12" type="radio" name="b" />
                    <label
                        class="w-4 h-4 rounded-full border-2 border-gray-700 transition-all duration-100 cursor-pointer hover:border-purple-700"
                        for="12"
                    ></label>
                </form>

                <form class="flex space-x-5">
                    <input class="hidden form-2-input" id="13" type="radio" name="c" />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="13"
                    ></label>
                    <input class="hidden form-2-input" id="14" type="radio" name="c" />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="14"
                    ></label>
                    <input class="hidden form-2-input" id="15" type="radio" name="c" />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="15"
                    ></label>
                    <input class="hidden form-2-input" id="16" type="radio" name="c" />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="16"
                    ></label>
                    <input class="hidden form-2-input" id="17" type="radio" name="c" checked />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="17"
                    ></label>
                    <input class="hidden form-2-input" id="18" type="radio" name="c" />
                    <label
                        class="w-4 h-4 bg-gray-700 rounded-sm ring-purple-900 transition-all duration-100 transform rotate-45 cursor-pointer hover:bg-purple-700 hover:ring-4"
                        for="18"
                    ></label>
                </form>

            </div>
        </div>
    }
}
