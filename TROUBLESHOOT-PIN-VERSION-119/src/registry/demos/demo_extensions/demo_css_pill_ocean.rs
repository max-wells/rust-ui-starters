use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCssPillOcean() -> impl IntoView {
    view! {
        <Stylesheet id="css-pill-ocean" href="/components/css-pill-ocean.css" />

        <div>
            <div class="cssPillOceanContainer">
                <div class="whaleContainer">
                    <div class="whalePos size1">
                        <div class="whaleRotate size1">
                            <div class="whale"></div>
                            <div class="fin"></div>
                        </div>
                    </div>
                    <div class="whalePos size2">
                        <div class="whaleRotate size2">
                            <div class="whale"></div>
                            <div class="fin"></div>
                        </div>
                    </div>
                    <div class="whalePos size3">
                        <div class="whaleRotate size3">
                            <div class="whale"></div>
                            <div class="fin"></div>
                        </div>
                    </div>
                    <div class="whalePos size4">
                        <div class="whaleRotate size4">
                            <div class="whale"></div>
                            <div class="fin"></div>
                        </div>
                    </div>
                </div>
                <div class="cPos">
                    <div class="cc">
                        <div class="circle one"></div>
                        <div class="circle two"></div>
                        <div class="circle three"></div>
                        <div class="circle four"></div>
                    </div>
                </div>
                <div class="triangleContainer">
                    <div class="triangleBar"></div>

                    <span class="triangle"></span>
                    <span class="triangle"></span>

                </div>
                <div class="gradientContainer">
                    <div class="overlay one"></div>
                    <div class="gradient">

                        <span class="ray1"></span>
                        <span class="ray2"></span>

                    </div>
                </div>
                <div class="rocks">
                    <div class="rock one"></div>
                    <div class="rock two"></div>
                    <div class="rock three"></div>
                    <div class="rock four"></div>
                </div>
                <div class="bubbleContainer">

                    <span class="bubbleY1">
                        <span class="bubbleX1"></span>
                    </span>
                    <span class="bubbleY2">
                        <span class="bubbleX2"></span>
                    </span>

                </div>
            </div>

        </div>
    }
}
