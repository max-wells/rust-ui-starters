use leptos::*;
use leptos_meta::Stylesheet;

// Source: https://codepen.io/tdoughty/pen/bGGZWqg (more trees and birds)

#[component]
pub fn DemoCssPillMountain() -> impl IntoView {
    view! {
        <Stylesheet id="css-pill-mountain" href="/components/css-pill-mountain.css" />

        <div class="flex justify-center items-center mx-auto w-full max-w-2xl">

            <div class="mx-auto w-full max-w-2xl">
                <div class="containerPillMountain">
                    <div class="birds front">
                        <div class="bird b1">
                            <div class="body">
                                <div class="wing1">
                                    <div class="wing2">
                                        <div class="wing3"></div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bird b12">
                            <div class="body">
                                <div class="wing1">
                                    <div class="wing2">
                                        <div class="wing3"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="birds back">
                        <div class="bird b1">
                            <div class="body">
                                <div class="wing1">
                                    <div class="wing2">
                                        <div class="wing3"></div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bird b12">
                            <div class="body">
                                <div class="wing1">
                                    <div class="wing2">
                                        <div class="wing3"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="cloud big">
                        <div class="circle c0"></div>

                        <div class="circle c8"></div>
                    </div>
                    <div class="cloud small">
                        <div class="circle c0"></div>

                        <div class="circle c8"></div>
                    </div>
                    <div class="mountain">
                        <div class="backdrop"></div>
                        <div class="zig zag0"></div>

                        <div class="zig zag4"></div>
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>
                    <div class="range">
                        <div class="r1"></div>

                        <div class="r7"></div>
                    </div>
                    <div class="tree treeBack tree1">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>

                    <div class="tree treeBack tree8">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>
                    <div class="tower">
                        <div class="shadow"></div>
                        <div class="flagPole"></div>
                        <div class="roof1"></div>
                        <div class="roof2"></div>
                        <div class="wall">
                            <div class="w1"></div>

                            <div class="w5"></div>
                        </div>
                        <div class="legs">
                            <div class="left"></div>
                            <div class="right"></div>
                            <div class="support1">
                                <div class="criss"></div>
                                <div class="cross"></div>
                                <div class="flat"></div>
                            </div>
                            <div class="support2">
                                <div class="criss"></div>
                                <div class="cross"></div>
                                <div class="flat"></div>
                            </div>
                        </div>
                        <div class="railing">
                            <div class="top"></div>
                            <div class="bot1"></div>
                            <div class="bot2"></div>
                            <div class="r1"></div>

                            <div class="r9"></div>
                        </div>
                    </div>
                    <div class="tree treeMid tree1">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>

                    <div class="tree treeMid tree5">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>
                    <div class="tree treeFront tree1">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>

                    <div class="tree treeFront tree4">
                        <div class="top"></div>
                        <div class="mid"></div>
                        <div class="bot"></div>
                        <div class="base"></div>
                    </div>
                </div>
            </div>

        </div>
    }
}
