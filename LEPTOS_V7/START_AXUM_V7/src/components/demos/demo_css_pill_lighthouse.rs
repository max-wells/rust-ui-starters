use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCssPillLighthouse() -> impl IntoView {
    view! {
        <Stylesheet id="css-pill-lighthouse" href="/components/css-pill-lighthouse.css" />

        <div class="flex items-center justify-center w-full max-w-2xl mx-auto">
            <div class="container-pill">
                <div class="shooting">
                    <div class="star"></div>
                    <div class="star0"></div>
                </div>
                <div class="lens"></div>
                <div class="flash"></div>
                <div class="pPos">
                    <div class="pyramid one"></div>
                    <div class="pyramid two"></div>
                    <div class="pyramid three"></div>
                    <div class="lPos">
                        <div class="lightTop"></div>
                        <div class="lightMid"></div>
                        <div class="lightBot">
                            <div class="window1"></div>
                            <div class="window2"></div>
                        </div>
                    </div>
                </div>
                <div class="cPos">
                    <div class="cc">
                        <div class="circle one"></div>
                        <div class="circle two"></div>
                        <div class="circle three"></div>
                        <div class="circle four"></div>
                        <div class="circle five"></div>
                    </div>
                </div>
            </div>

        </div>
    }
}
