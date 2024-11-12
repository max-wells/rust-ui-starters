use leptos::*;

#[component]
pub fn DemoParallaxSimple() -> impl IntoView {
    view! {
        <header
            class="flex justify-center items-center pb-12 h-screen bg-fixed bg-center bg-no-repeat bg-cover"
            style="background-image: url('https://picsum.photos/1920/1920/?random');"
        >
            <a href="https://joca.dev/en/me/?ref=parallax-codepen" target="_blank">
                <div class="p-4 text-center bg-white rounded-xl shadow-xl transition duration-300 md:p-8 hover:bg-gray-100">
                    <p class="text-sm italic">November 1, 2023</p>
                    <h1 class="text-5xl uppercase">Parrallax effect</h1>
                    <p class="text-lg">Joca.dev</p>
                </div>
            </a>
        </header>

        <div class="py-12 px-4 mx-auto max-w-xl leading-normal">
            <p class="mb-4 text-lg">
                The parallax effect in HTML is a web design technique that creates the illusion of depth by making elements on a
                page move at different speeds as the user scrolls through the site. This effect is achieved by displacing the
                backgrounds of page sections at different rates in relation to the foreground content. The result is a visually
                appealing experience that adds dynamism and depth to the web page.
            </p>
            <p class="mb-4 text-lg">
                In more technical terms, the parallax effect is achieved by manipulating the position and scroll properties of HTML
                and CSS elements. Typically, background images or gradients are used, and they move at different speeds compared to
                the main content. This creates the sensation of layers moving at different speeds, generating the illusion of
                three-dimensional depth and motion.
            </p>
            <p class="mb-4 text-lg">
                The parallax effect has become popular in modern web design due to its ability to enhance the user experience and
                make pages feel more interactive and engaging. It can be implemented using HTML, CSS, and JavaScript, and there are
                libraries and frameworks that simplify its integration into websites.
            </p>
        </div>

        <div
            class="container flex justify-center items-center mx-auto h-screen bg-fixed bg-center bg-no-repeat bg-cover"
            style="background-image: url('https://picsum.photos/1600/1600/?random');"
        >
            <blockquote class="p-4 mx-4 text-center text-gray-800 bg-gray-300 rounded-xl shadow-xl transition duration-300 md:p-8 hover:bg-gray-400">
                <p class="text-3xl italic font-bold">
                    &ldquo;Parallax effect adds depth by moving layers at different speeds during scrolling.&rdquo;
                </p>
            </blockquote>
        </div>

        <div class="py-12 px-4 mx-auto max-w-xl leading-normal">
            <h2 class="mb-2 text-lg font-semibold text-gray-900">Multiple Layers:</h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    The parallax effect involves dividing the page into different layers, with each layer containing different
                    elements
                    or content sections.
                </li>
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    Each layer moves at a different speed as the user scrolls, creating the illusion of depth and movement.
                </li>
            </ul>

            <h2 class="mb-2 text-lg font-semibold text-gray-900">
                Implementation with CSS and JavaScript:
            </h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    CSS is used to set the position, size, and z-index properties of the layers.
                </li>
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    JavaScript can be used to control the movement of the layers in response to user scrolling.
                </li>
            </ul>

            <h2 class="mb-2 text-lg font-semibold text-gray-900">Images and Backgrounds:</h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    Frequently, large background images are used for each layer, contributing to the parallax effect.
                </li>
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    These images can be static or, in some cases, even videos or multimedia elements.
                </li>
            </ul>

            <h2 class="mb-2 text-lg font-semibold text-gray-900">Scrolling Speeds:</h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    The key to the effect lies in assigning different scrolling speeds to each layer.
                </li>
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    Layers closer to the viewer (usually in the foreground) move faster, while farther layers move more slowly.
                </li>
            </ul>

            <h2 class="mb-2 text-lg font-semibold text-gray-900">Responsive Design:</h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">

                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    " When implementing the parallax effect, it's important to consider responsive design to ensure it works properly on
                     a
                     variety of devices and screen sizes."
                </li>
            </ul>

            <h2 class="mb-2 text-lg font-semibold text-gray-900">Frameworks and Libraries:</h2>
            <ul class="space-y-1 max-w-xl list-inside text-gray-500">
                <li class="flex items-center">
                    <svg
                        class="flex-shrink-0 w-3.5 h-3.5 text-green-500 me-2"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" />
                    </svg>
                    Frameworks and libraries like Parallax.js, ScrollMagic, and AOS (Animate On Scroll) facilitate the implementation
                    of
                    the parallax effect without the need for extensive custom code.
                </li>
            </ul>
        </div>
        <footer class="p-4 w-full bg-white border-t border-gray-200 shadow md:flex md:justify-between md:items-center md:p-6">
            <span class="text-sm text-gray-500 sm:text-center">
                2023
                <a
                    href="https://joca.dev/en/me/?ref=parallax-codepen"
                    target="_blank"
                    class="hover:underline"
                >
                    Joca.dev
                </a>. All Rights Reserved.
            </span>
        </footer>
    }
}
