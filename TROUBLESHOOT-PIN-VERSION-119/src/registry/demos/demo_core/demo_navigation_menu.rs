use leptos::*;

#[component]
pub fn DemoNavigationMenu() -> impl IntoView {
    view! {
        <div class="flex justify-center items-start h-[400px]">
            <nav
                id="menu"
                class="flex relative flex-wrap justify-evenly p-2 mx-auto border border-purple-500 z-[2] custom-nav"
            >
                <div class="flex flex-grow justify-center p-4 border border-orange-500 menu-item">
                    <div class="p-2 rounded-md cursor-pointer bg-muted h-fit menu-text">
                        <a href="#">Products</a>
                    </div>
                    <div class="sub-menu">
                        <div class="border icon-box border-sky-500">
                            <div class="text">
                                <div class="title">Button</div>
                                <div class="sub-text">
                                    <a
                                        href="#"
                                        onclick="window.location.href='/demos-core/button'; return false;"
                                    >
                                        "🔗 Button (little trick)"
                                    </a>
                                </div>

                            </div>
                        </div>
                        <div class="border icon-box border-sky-500">

                            <div class="text">
                                <div class="title">Idea Creator</div>
                                <div class="sub-text">
                                    Think of an idea, and have an AI create it.
                                </div>
                            </div>
                        </div>
                        <div class="border icon-box border-sky-500">

                            <div class="text">
                                <div class="title">Super Collider</div>
                                <div class="sub-text">Remove mass from any object.</div>
                            </div>
                        </div>
                        <div class="sub-menu-holder"></div>
                    </div>
                </div>

                //
                //

                <div class="flex flex-grow justify-center p-4 border border-orange-500 menu-item">
                    <div class="p-2 rounded-md cursor-pointer bg-muted h-fit menu-text">
                        <a href="#">Services</a>
                    </div>
                    <div class="sub-menu double">
                        <div class="icon-box gb a">

                            <div class="text">
                                <div class="title">Consult</div>
                                <div class="sub-text">From Professionals</div>
                            </div>
                        </div>
                        <div class="icon-box gb b">

                            <div class="text">
                                <div class="title">Teach</div>
                                <div class="sub-text">In Classroom</div>
                            </div>
                        </div>
                        <div class="icon-box gb c">

                            <div class="text">
                                <div class="title">Learn</div>
                                <div class="sub-text">By Video</div>
                            </div>
                        </div>
                        <div class="icon-box gb d">

                            <div class="text">
                                <div class="title">Keep</div>
                                <div class="sub-text">The Castle</div>
                            </div>
                        </div>
                        <div class="icon-box gb e">

                            <div class="text">
                                <div class="title">Become</div>
                                <div class="sub-text">A Creator</div>
                            </div>
                        </div>
                        <div class="icon-box gb f">

                            <div class="text">
                                <div class="title">Understand</div>
                                <div class="sub-text">Our Journey</div>
                            </div>
                        </div>
                        <div class="bottom-container">
                            Ready to dive in? <a href="#">Get Started</a>
                        </div>
                        <div class="sub-menu-holder"></div>
                    </div>
                </div>

                <div id="sub-menu-container">
                    <div id="sub-menu-holder">
                        <div id="sub-menu-bottom"></div>
                    </div>
                </div>
            </nav>

        </div>
    }
}
