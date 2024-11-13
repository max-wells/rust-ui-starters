use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCarouselSnapScroll() -> impl IntoView {
    view! {
        <Stylesheet id="carousel-snap-scroll" href="/components/carousel-snap-scroll.css" />

        <div class="mainDiv">
            <div class="scrollsnap-carousel">
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide one</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide two</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide three</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide four</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide five</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide six</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide seven</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide eight</div>
                    </div>
                </div>
            </div>

        </div>
    }
}
