use leptos::*;

#[component]
pub fn DemoMasonry() -> impl IntoView {
    view! {
        <div class="relative antialiased font-inter">

            <main class="flex overflow-hidden relative flex-col justify-center min-h-screen bg-slate-300">
                <div class="py-24 px-4 mx-auto w-full max-w-5xl md:px-6">

                    <div class="space-y-20">

                        <div>
                            <h2 class="mb-4 text-lg font-bold">With CSS Grid</h2>

                            <div class="grid grid-cols-2 gap-4 md:grid-cols-4">

                                <div class="grid gap-4">
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-01.jpg"
                                            width="232"
                                            height="290"
                                            alt="Image 01"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-02.jpg"
                                            width="232"
                                            height="290"
                                            alt="Image 02"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-03.jpg"
                                            width="232"
                                            height="174"
                                            alt="Image 03"
                                        />
                                    </div>
                                </div>

                                <div class="grid gap-4">
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-04.jpg"
                                            width="232"
                                            height="155"
                                            alt="Image 04"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-05.jpg"
                                            width="232"
                                            height="349"
                                            alt="Image 05"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-06.jpg"
                                            width="232"
                                            height="250"
                                            alt="Image 06"
                                        />
                                    </div>
                                </div>

                                <div class="grid gap-4">
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-07.jpg"
                                            width="232"
                                            height="349"
                                            alt="Image 07"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-08.jpg"
                                            width="232"
                                            height="155"
                                            alt="Image 08"
                                        />
                                    </div>
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-09.jpg"
                                            width="232"
                                            height="250"
                                            alt="Image 09"
                                        />
                                    </div>
                                </div>

                                <div class="grid gap-4">
                                    <div>
                                        <img
                                            class="w-full rounded-xl shadow"
                                            src="https://cruip-tutorials.vercel.app/masonry/masonry-10.jpg"
                                            width="232"
                                            height="290"
                                            alt="Image 10"
                                        />
                                    </div>
                                    <img
                                        class="w-full rounded-xl shadow"
                                        src="https://cruip-tutorials.vercel.app/masonry/masonry-11.jpg"
                                        width="232"
                                        height="155"
                                        alt="Image 11"
                                    />
                                    <img
                                        class="w-full rounded-xl shadow"
                                        src="https://cruip-tutorials.vercel.app/masonry/masonry-12.jpg"
                                        width="232"
                                        height="309"
                                        alt="Image 12"
                                    />
                                </div>
                            </div>
                        </div>

                        <div>
                            <h2 class="mb-4 text-lg font-bold">With CSS Columns</h2>

                            <div class="gap-4 space-y-4 columns-2 md:columns-4">
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-01.jpg"
                                    width="232"
                                    height="290"
                                    alt="Image 01"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-02.jpg"
                                    width="232"
                                    height="290"
                                    alt="Image 02"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-03.jpg"
                                    width="232"
                                    height="174"
                                    alt="Image 03"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-04.jpg"
                                    width="232"
                                    height="155"
                                    alt="Image 04"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-05.jpg"
                                    width="232"
                                    height="349"
                                    alt="Image 05"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-06.jpg"
                                    width="232"
                                    height="250"
                                    alt="Image 06"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-07.jpg"
                                    width="232"
                                    height="349"
                                    alt="Image 07"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-08.jpg"
                                    width="232"
                                    height="155"
                                    alt="Image 08"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-09.jpg"
                                    width="232"
                                    height="250"
                                    alt="Image 09"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-10.jpg"
                                    width="232"
                                    height="290"
                                    alt="Image 10"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-11.jpg"
                                    width="232"
                                    height="155"
                                    alt="Image 11"
                                />
                                <img
                                    class="w-full rounded-xl shadow"
                                    src="https://cruip-tutorials.vercel.app/masonry/masonry-12.jpg"
                                    width="232"
                                    height="309"
                                    alt="Image 12"
                                />
                            </div>
                        </div>

                    </div>

                </div>
            </main>

        </div>
    }
}
